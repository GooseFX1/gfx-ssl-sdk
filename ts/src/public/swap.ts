import { ISwapToken } from "../states/ssl";
import { BN, Program, Provider } from "@project-serum/anchor";
import { PAIR_LAYOUT } from "../states";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createCloseAccountInstruction,
  createSyncNativeInstruction,
} from "@solana/spl-token-sdk";
import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  TransactionSignature,
  SYSVAR_RENT_PUBKEY,
  Signer,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { Buffer } from "buffer";
import {
  poolAddress,
  poolController,
  tokens,
  SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
  SYSTEM,
} from "../constants";

const SwapIDL = require("../idl/gfx_ssl_idl.json");

const getPairDetails = async (tokenA: ISwapToken, tokenB: ISwapToken) => {
  const addresses = [
    new PublicKey(tokenA.address).toBuffer(),
    new PublicKey(tokenB.address).toBuffer(),
  ].sort(Buffer.compare);

  const pairArr = await PublicKey.findProgramAddress(
    [
      new Buffer("GFX-SSL-Pair", "utf-8"),
      poolController.toBuffer(),
      addresses[0],
      addresses[1],
    ],
    poolAddress
  );

  const pair = pairArr[0];

  return pair;
};

const genSSL = async (address: string) => {
  return await PublicKey.findProgramAddress(
    [
      new Buffer("GFX-SSL", "utf-8"),
      poolController.toBuffer(),
      new PublicKey(address).toBuffer(),
    ],
    poolAddress
  );
};

const findAssociatedTokenAddress = async (
  walletAddress: PublicKey,
  tokenMintAddress: PublicKey
): Promise<PublicKey> => {
  return (
    await PublicKey.findProgramAddress(
      [
        walletAddress.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        tokenMintAddress.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
  )[0];
};

const createAssociatedTokenAccountIx = (
  mint: PublicKey,
  associatedAccount: PublicKey,
  owner: PublicKey
) =>
  createAssociatedTokenAccountInstruction(
    owner,
    associatedAccount,
    owner,
    mint
  );

const wrapSolToken = async (wallet: any, amount: number) => {
  const tx = new Transaction();
  try {
    const associatedTokenAccount = await getAssociatedTokenAddress(
      NATIVE_MINT,
      wallet.publicKey
    );
    // Create token account to hold your wrapped SOL
    if (associatedTokenAccount)
      tx.add(
        createAssociatedTokenAccountInstruction(
          wallet.publicKey,
          associatedTokenAccount,
          wallet.publicKey,
          NATIVE_MINT
        )
      );

    // Transfer SOL to associated token account and use SyncNative to update wrapped SOL balance
    tx.add(
      SystemProgram.transfer({
        fromPubkey: wallet.publicKey,
        toPubkey: associatedTokenAccount,
        lamports: amount,
      }),
      createSyncNativeInstruction(associatedTokenAccount)
    );

    return tx; //signAndSendRawTransaction(connection, tx, wallet)
  } catch {
    return tx;
  }
};

export const getQuote = async (
  firstToken: string,
  secondToken: string,
  inTokenAmount: number,
  connection: Connection
) => {
  const [tokenA, tokenB]: [ISwapToken, ISwapToken] = [
    tokens[firstToken],
    tokens[secondToken],
  ];
  const result = await preSwapAmount(tokenA, tokenB, inTokenAmount, connection);

  return Number(result.preSwapResult);
};

export const getMinimumQuote = async (
  tokenA: string,
  tokenB: string,
  inTokenAmount: number,
  connection: Connection,
  slippage: number
) => {
  const result = await getQuote(tokenA, tokenB, inTokenAmount, connection);
  const minAmountOut = result * (1 - slippage);
  return minAmountOut;
};

export const getPriceImpact = async (
  firstToken: string,
  secondToken: string,
  inTokenAmount: number,
  connection: Connection
) => {
  const [tokenA, tokenB] = [tokens[firstToken], tokens[secondToken]];
  const result = await preSwapAmount(tokenA, tokenB, inTokenAmount, connection);

  return Number(result.impact);
};

export const preSwapAmount = async (
  tokenA: ISwapToken,
  tokenB: ISwapToken,
  inTokenAmount: number,
  connection: Connection
): Promise<{
  preSwapResult: string | undefined;
  impact: number;
}> => {
  try {
    const wasm = await import("gfx-ssl-wasm"); //
    const swapWASM = wasm.swap;
    const OracleRegistry = wasm.OracleRegistry;
    if (!inTokenAmount || inTokenAmount === 0)
      return { impact: 0, preSwapResult: "0" };

    const pair = await getPairDetails(tokenA, tokenB);
    const pairData = await connection.getAccountInfo(pair);
    const sslIn = await genSSL(tokenA.address);
    const sslOut = await genSSL(tokenB.address);
    const tokenASSLData = await connection.getAccountInfo(
      new PublicKey(sslIn[0])
    );
    const tokenBSSLData = await connection.getAccountInfo(
      new PublicKey(sslOut[0])
    );

    if (!tokenASSLData || !tokenBSSLData || !pairData?.data) {
      return {
        preSwapResult: "0",
        impact: 0,
      };
    }

    const decoded = PAIR_LAYOUT.decode(pairData?.data);
    const { oracles, nOracle } = decoded;
    const n = Number(nOracle.toString());
    const registry = new OracleRegistry();
    for (const oracle of oracles.slice(0, n)) {
      const n = Number(oracle.n);

      for (const elem of oracle.elements.slice(0, n)) {
        const acctInfo = await connection.getAccountInfo(elem.address);
        if (acctInfo?.data) {
          registry.add_oracle(elem.address.toBuffer(), acctInfo.data);
        }
      }
    }
    const pseudoAmount = 10000000;
    const scale = pseudoAmount / inTokenAmount;
    const out = swapWASM(
      tokenASSLData.data,
      tokenBSSLData.data,
      pairData.data,
      registry,
      BigInt(pseudoAmount),
      BigInt(0)
    );

    const differenceInDecimals = tokenA.decimals - tokenB.decimals;
    const trueValue = Number(out.out.toString()) / scale;

    return {
      preSwapResult: +(trueValue * 10 ** differenceInDecimals).toFixed(7) + "",
      impact: out.price_impact,
    };
  } catch (e) {
    console.log(e);
    return {
      preSwapResult: "0",
      impact: 0,
    };
  }
};

export const createSwapInstruction = async (
  tokenA: ISwapToken,
  tokenB: ISwapToken,
  inTokenAmount: number,
  outTokenAmount: number,
  slippage: number,
  wallet: any,
  connection: Connection,
  txn?: Transaction
): Promise<Transaction> => {
  const tx = txn || new Transaction();
  if (!wallet.publicKey || !wallet.signTransaction) return tx;

  const program = new Program(
    SwapIDL,
    poolAddress.toBase58(),
    new Provider(connection, wallet as any, { commitment: "processed" })
  );
  const inst: any = program.instruction;
  const amountIn = new BN(inTokenAmount * 10 ** tokenA.decimals);
  const minimumAmountOut = new BN(
    outTokenAmount * 10 ** tokenB.decimals * (1 - slippage)
  );
  const pair = await getPairDetails(tokenA, tokenB);

  const [inTokenAtaUser, outTokenAtaUser] = await Promise.all([
    await findAssociatedTokenAddress(
      wallet.publicKey,
      new PublicKey(tokenA.address)
    ),
    await findAssociatedTokenAddress(
      wallet.publicKey,
      new PublicKey(tokenB.address)
    ),
  ]);

  const sslIn = await genSSL(tokenA.address);
  const sslOut = await genSSL(tokenB.address);
  const vaultIn = await findAssociatedTokenAddress(
    sslIn[0],
    new PublicKey(tokenA.address)
  );
  const vaultOut = await findAssociatedTokenAddress(
    sslOut[0],
    new PublicKey(tokenB.address)
  );

  if (!(await connection.getAccountInfo(outTokenAtaUser))) {
    tx.add(
      createAssociatedTokenAccountIx(
        new PublicKey(tokenB.address),
        outTokenAtaUser,
        wallet.publicKey
      )
    );
  }

  try {
    const pairData = await connection.getAccountInfo(pair);
    if (!pairData || !pairData.data)
      throw new Error("Token Pair do not exist yet.");

    const tokenAccountA = await findAssociatedTokenAddress(
      wallet.publicKey,
      new PublicKey(tokenA.address)
    );
    if (
      tokenA.address !== NATIVE_MINT.toBase58() &&
      !(await connection.getParsedAccountInfo(tokenAccountA)).value
    ) {
      tx.add(
        createAssociatedTokenAccountIx(
          new PublicKey(tokenA.address),
          tokenAccountA,
          wallet.publicKey
        )
      );
    }

    const tokenAccountB = await findAssociatedTokenAddress(
      wallet.publicKey,
      new PublicKey(tokenB.address)
    );
    if (
      tokenB.address !== NATIVE_MINT.toBase58() &&
      !(await connection.getParsedAccountInfo(tokenAccountB)).value
    ) {
      tx.add(
        createAssociatedTokenAccountIx(
          new PublicKey(tokenB.address),
          tokenAccountB,
          wallet.publicKey
        )
      );
    }

    const decoded = PAIR_LAYOUT.decode(pairData.data);
    const { oracles, nOracle, feeCollector } = decoded;
    const n = Number(nOracle.toString());
    const remainingAccounts = [];
    for (const oracle of oracles.slice(0, n)) {
      for (const elem of oracle.elements.slice(0, Number(oracle.n))) {
        remainingAccounts.push({
          isSigner: false,
          isWritable: false,
          pubkey: elem.address,
        });
      }
    }
    const collector = feeCollector;

    const accounts = {
      controller: poolController,
      pair,
      sslIn: sslIn[0],
      sslOut: sslOut[0],
      mintIn: new PublicKey(tokenA.address),
      mintOut: new PublicKey(tokenB.address),
      vaultIn,
      vaultOut,
      userWallet: wallet.publicKey,
      userInAta: inTokenAtaUser,
      userOutAta: outTokenAtaUser,
      instructions: new PublicKey(
        "Sysvar1nstructions1111111111111111111111111"
      ),
      feeCollectorAta: await findAssociatedTokenAddress(
        new PublicKey(collector),
        new PublicKey(tokenA.address)
      ),
      feeCollector: collector,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
      systemProgram: SYSTEM,
      rent: SYSVAR_RENT_PUBKEY,
    };

    tx.add(
      await inst.rebalanceSwap(amountIn, minimumAmountOut, {
        accounts,
        remainingAccounts,
      })
    );
    tx.add(await inst.preSwap({ accounts, remainingAccounts }));
    tx.add(await inst.swap({ accounts, remainingAccounts }));
  } catch (error) {
    console.log(error);
  }

  return tx;
};

export const swapToken = async (
  firstToken: string,
  secondToken: string,
  inTokenAmount: number,
  slippage: number,
  wallet: any,
  connection: Connection
): Promise<TransactionSignature | null | undefined> => {
  try {
    const [tokenA, tokenB] = [tokens[firstToken], tokens[secondToken]];
    const outTokenAmount = await getQuote(
      firstToken,
      secondToken,
      inTokenAmount,
      connection
    );

    let preTx = new Transaction();
    if (tokenA.address === NATIVE_MINT.toBase58()) {
      const txn = await wrapSolToken(wallet, inTokenAmount * LAMPORTS_PER_SOL);
      if (txn) {
        preTx = txn;
      }
    }

    const tx = await createSwapInstruction(
      tokenA,
      tokenB,
      inTokenAmount,
      outTokenAmount,
      slippage,
      wallet,
      connection,
      preTx
    );

    // unwrapping sol if tokenB is sol
    if (tokenB.address === NATIVE_MINT.toBase58()) {
      try {
        const associatedTokenAccount = await getAssociatedTokenAddress(
          NATIVE_MINT,
          wallet.publicKey
        );
        if (associatedTokenAccount) {
          const tr = createCloseAccountInstruction(
            associatedTokenAccount,
            wallet.publicKey,
            wallet.publicKey
          );
          tx.add(tr);
        }
      } catch (e) {
        console.log(e);
      }
    }

    const finalResult = await signAndSendRawTransaction(connection, tx, wallet);
    if (finalResult) {
      let result = await connection.confirmTransaction(finalResult);

      if (!result?.value?.err) {
        return finalResult;
      } else {
        return null;
      }
    }
  } catch {
    return null;
  }
};

export const signAndSendRawTransaction = async (
  connection: Connection,
  transaction: Transaction,
  wallet: any,
  ...signers: Array<Signer>
) => {
  try {
    transaction.feePayer = wallet.publicKey;
    transaction.recentBlockhash = (
      await connection.getRecentBlockhash("max")
    ).blockhash;

    signers.forEach((signer) => transaction.partialSign(signer));

    transaction = await wallet.signTransaction(transaction);
    const tx = await connection.sendRawTransaction(transaction!.serialize());
    if (tx) {
      return tx;
    }
    return null;
  } catch (e) {
    console.log(e);
    return null;
  }
};
