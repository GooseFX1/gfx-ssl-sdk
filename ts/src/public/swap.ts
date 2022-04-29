import { ISwapToken } from "../states/ssl";
import { BN, Program, Provider, Idl } from "@project-serum/anchor";
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
  POOL_ADDRESS,
  POOL_CONTROLLER,
  TOKENS,
  SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
  SYSTEM,
} from "../constants";
import SwapIDL from "../idl/gfx_ssl_idl.json";
const wasm = import("gfx-ssl-wasm");

export interface Quote {
  preSwapResult: string | undefined;
  impact: number;
}

export interface QuoteObject {
  [inTokenAmount: number]: Quote;
}

export class Swap {
  private connection: Connection;
  private millisecondsToLive: number;
  public quote: QuoteObject;
  private fetchDate: Date;
  private wasm: any;

  constructor(connection: Connection) {
    this.connection = connection;
    this.millisecondsToLive = 1 * 60 * 1000; //1 minute cache time
    this.quote = {};
    this.wasm = null;
    this.fetchDate = new Date();
  }

  public isCacheExpired() {
    return (
      this.fetchDate.getTime() + this.millisecondsToLive < new Date().getTime()
    );
  }

  public resetCache() {
    this.fetchDate = new Date();
    this.quote = {};
  }

  public async initialize() {
    this.wasm = await wasm;
  }

  public getPairDetails = (tokenA: ISwapToken, tokenB: ISwapToken) => {
    const addresses = [
      new PublicKey(tokenA.address).toBuffer(),
      new PublicKey(tokenB.address).toBuffer(),
    ].sort(Buffer.compare);

    const pairArr = PublicKey.findProgramAddressSync(
      [
        new Buffer("GFX-SSL-Pair", "utf-8"),
        POOL_CONTROLLER.toBuffer(),
        addresses[0],
        addresses[1],
      ],
      POOL_ADDRESS
    );

    const pair = pairArr[0];

    return pair;
  };

  public genSSL = (address: string) => {
    return PublicKey.findProgramAddressSync(
      [
        new Buffer("GFX-SSL", "utf-8"),
        POOL_CONTROLLER.toBuffer(),
        new PublicKey(address).toBuffer(),
      ],
      POOL_ADDRESS
    );
  };

  public createAssociatedTokenAccountIx = (
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

  public findAssociatedTokenAddress = (
    walletAddress: PublicKey,
    tokenMintAddress: PublicKey
  ): PublicKey => {
    return PublicKey.findProgramAddressSync(
      [
        walletAddress.toBuffer(),
        TOKEN_PROGRAM_ID.toBuffer(),
        tokenMintAddress.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
    )[0];
  };

  public wrapSolToken = async (wallet: any, amount: number) => {
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

      return tx;
    } catch {
      return tx;
    }
  };

  public preSwapAmount = async (
    tokenA: ISwapToken,
    tokenB: ISwapToken,
    inTokenAmount: number
  ): Promise<Quote> => {
    try {
      if (!this.wasm) {
        await this.initialize();
      }

      if (this.quote[inTokenAmount] && !this.isCacheExpired())
        return Promise.resolve(this.quote[inTokenAmount]);

      const swapWASM = this.wasm.swap;
      const OracleRegistry = this.wasm.OracleRegistry;
      if (!inTokenAmount || inTokenAmount === 0)
        return { impact: 0, preSwapResult: "0" };

      const pair = this.getPairDetails(tokenA, tokenB);
      const pairData = await this.connection.getAccountInfo(pair);
      const sslIn = this.genSSL(tokenA.address);
      const sslOut = this.genSSL(tokenB.address);
      const tokenASSLData = await this.connection.getAccountInfo(
        new PublicKey(sslIn[0])
      );
      const tokenBSSLData = await this.connection.getAccountInfo(
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
          const acctInfo = await this.connection.getAccountInfo(elem.address);
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
      const finalResult: Quote = {
        preSwapResult:
          +(trueValue * 10 ** differenceInDecimals).toFixed(7) + "",
        impact: out.price_impact,
      };

      this.fetchDate = new Date();
      this.quote[inTokenAmount] = finalResult;
      return finalResult;
    } catch (e) {
      console.log(e);
      return {
        preSwapResult: "0",
        impact: 0,
      };
    }
  };

  public getQuote = async (
    firstToken: string,
    secondToken: string,
    inTokenAmount: number,
    connection?: Connection
  ) => {
    if (connection) this.connection = connection;

    const [tokenA, tokenB]: [ISwapToken, ISwapToken] = [
      TOKENS[firstToken],
      TOKENS[secondToken],
    ];
    const result = await this.preSwapAmount(tokenA, tokenB, inTokenAmount);

    return Number(result.preSwapResult);
  };

  public getMinimumQuote = async (
    tokenA: string,
    tokenB: string,
    inTokenAmount: number,
    slippage: number,
    connection?: Connection
  ) => {
    if (connection) this.connection = connection;
    const result = await this.getQuote(tokenA, tokenB, inTokenAmount);
    const minAmountOut = result * (1 - slippage);
    return minAmountOut;
  };

  public getPriceImpact = async (
    firstToken: string,
    secondToken: string,
    inTokenAmount: number,
    connection?: Connection
  ) => {
    if (connection) this.connection = connection;
    const [tokenA, tokenB] = [TOKENS[firstToken], TOKENS[secondToken]];
    const result = await this.preSwapAmount(tokenA, tokenB, inTokenAmount);

    return Number(result.impact);
  };

  public createSwapInstruction = async (
    tokenA: ISwapToken,
    tokenB: ISwapToken,
    inTokenAmount: number,
    outTokenAmount: number,
    slippage: number,
    wallet: any,
    txn?: Transaction
  ): Promise<Transaction> => {
    const tx = txn || new Transaction();
    try {
      if (!wallet.publicKey || !wallet.signTransaction) return tx;

      const program = new Program(
        SwapIDL as Idl,
        POOL_ADDRESS.toBase58(),
        new Provider(this.connection, wallet as any, {
          commitment: "processed",
        })
      );
      const inst: any = program.instruction;
      const amountIn = new BN(inTokenAmount * 10 ** tokenA.decimals);
      const minimumAmountOut = new BN(
        outTokenAmount * 10 ** tokenB.decimals * (1 - slippage)
      );
      const pair = this.getPairDetails(tokenA, tokenB);

      const [inTokenAtaUser, outTokenAtaUser] = [
        this.findAssociatedTokenAddress(
          wallet.publicKey,
          new PublicKey(tokenA.address)
        ),
        this.findAssociatedTokenAddress(
          wallet.publicKey,
          new PublicKey(tokenB.address)
        ),
      ];

      const sslIn = this.genSSL(tokenA.address);
      const sslOut = this.genSSL(tokenB.address);
      const vaultIn = this.findAssociatedTokenAddress(
        sslIn[0],
        new PublicKey(tokenA.address)
      );
      const vaultOut = this.findAssociatedTokenAddress(
        sslOut[0],
        new PublicKey(tokenB.address)
      );

      if (!(await this.connection.getAccountInfo(outTokenAtaUser))) {
        tx.add(
          this.createAssociatedTokenAccountIx(
            new PublicKey(tokenB.address),
            outTokenAtaUser,
            wallet.publicKey
          )
        );
      }
      const pairData = await this.connection.getAccountInfo(pair);
      if (!pairData || !pairData.data)
        throw new Error("Token Pair do not exist yet.");

      const tokenAccountA = this.findAssociatedTokenAddress(
        wallet.publicKey,
        new PublicKey(tokenA.address)
      );
      if (
        tokenA.address !== NATIVE_MINT.toBase58() &&
        !(await this.connection.getParsedAccountInfo(tokenAccountA)).value
      ) {
        tx.add(
          this.createAssociatedTokenAccountIx(
            new PublicKey(tokenA.address),
            tokenAccountA,
            wallet.publicKey
          )
        );
      }

      const tokenAccountB = this.findAssociatedTokenAddress(
        wallet.publicKey,
        new PublicKey(tokenB.address)
      );
      if (
        tokenB.address !== NATIVE_MINT.toBase58() &&
        !(await this.connection.getParsedAccountInfo(tokenAccountB)).value
      ) {
        tx.add(
          this.createAssociatedTokenAccountIx(
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
        controller: POOL_CONTROLLER,
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
        feeCollectorAta: this.findAssociatedTokenAddress(
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

  public swapToken = async (
    firstToken: string,
    secondToken: string,
    inTokenAmount: number,
    slippage: number,
    wallet: any,
    connection?: Connection
  ): Promise<TransactionSignature | null | undefined> => {
    if (connection) this.connection = connection;
    try {
      const [tokenA, tokenB] = [TOKENS[firstToken], TOKENS[secondToken]];
      const outTokenAmount = await this.getQuote(
        firstToken,
        secondToken,
        inTokenAmount
      );

      let preTx = new Transaction();
      if (tokenA.address === NATIVE_MINT.toBase58()) {
        const txn = await this.wrapSolToken(
          wallet,
          inTokenAmount * LAMPORTS_PER_SOL
        );
        if (txn) {
          preTx = txn;
        }
      }

      const tx = await this.createSwapInstruction(
        tokenA,
        tokenB,
        inTokenAmount,
        outTokenAmount,
        slippage,
        wallet,
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

      const finalResult = await this.signAndSendRawTransaction(tx, wallet);
      if (finalResult) {
        let result = await this.connection.confirmTransaction(finalResult);

        if (!result?.value?.err) {
          return finalResult;
        } else {
          return null;
        }
      }
    } catch (e) {
      console.log(e);
      return null;
    }
  };

  private signAndSendRawTransaction = async (
    transaction: Transaction,
    wallet: any,
    ...signers: Array<Signer>
  ) => {
    try {
      transaction.feePayer = wallet.publicKey;
      transaction.recentBlockhash = (
        await this.connection.getRecentBlockhash("max")
      ).blockhash;

      signers.forEach((signer) => transaction.partialSign(signer));

      transaction = await wallet.signTransaction(transaction);
      const tx = await this.connection.sendRawTransaction(
        transaction!.serialize()
      );
      if (tx) {
        return tx;
      }
      return null;
    } catch (e) {
      console.log(e);
      return null;
    }
  };
}

export async function SwapFactory(connection: Connection) {
  const swap = new Swap(connection);
  await swap.initialize();
  return swap;
}
