import { Program, Provider, Idl, } from "@project-serum/anchor";
import { PAIR_LAYOUT } from "../states";
import {
  TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token-sdk";
import {
  Connection,
  PublicKey,
  SYSVAR_RENT_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { Buffer } from "buffer";
import {
  POOL_ADDRESS,
  POOL_CONTROLLER,
  SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
  SYSTEM,
} from "../constants";
import SwapIDL from "../idl/gfx_ssl_idl.json";
import { findAssociatedTokenAddress } from "./utils";

export interface Quote {
  out: BigInt;
  impact: number;
}

export class Swap {
  private connection: Connection;
  private wasm: any;

  constructor(connection: Connection) {
    this.connection = connection;
    this.wasm = null;
  }

  public async getWasm() {
    if (this.wasm === null) {
      this.wasm = await import("gfx-ssl-wasm");
    }
    return this.wasm;
  }

  public getPairAddress = (tokenA: PublicKey, tokenB: PublicKey) => {
    const addresses = [tokenA.toBuffer(), tokenB.toBuffer()].sort(Buffer.compare);

    const pairArr = PublicKey.findProgramAddressSync(
      [
        Buffer.from("GFX-SSL-Pair", "utf-8"),
        POOL_CONTROLLER.toBuffer(),
        addresses[0],
        addresses[1],
      ],
      POOL_ADDRESS
    );

    return pairArr[0];
  };

  public getSSLAddress = (address: PublicKey) => {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("GFX-SSL", "utf-8"),
        POOL_CONTROLLER.toBuffer(),
        address.toBuffer(),
      ],
      POOL_ADDRESS
    )[0];
  };

  public createAssociatedTokenAccountIx = (
    mint: PublicKey,
    associatedAccount: PublicKey,
    owner: PublicKey
  ) => createAssociatedTokenAccountInstruction(
    owner,
    associatedAccount,
    owner,
    mint
  );

  public getQuote = async (
    tokenA: PublicKey,
    tokenB: PublicKey,
    inTokenAmount: BigInt
  ): Promise<Quote> => {
    let wasm = await this.getWasm();

    const swapWASM = wasm.swap;
    const OracleRegistry = this.wasm.OracleRegistry;
    if (inTokenAmount === 0n)
      return { impact: 0, out: 0n };

    const pair = this.getPairAddress(tokenA, tokenB);
    const pairData = await this.connection.getAccountInfo(pair);
    const sslIn = this.getSSLAddress(tokenA);
    const sslOut = this.getSSLAddress(tokenB);
    const tokenASSLData = await this.connection.getAccountInfo(sslIn);
    const tokenBSSLData = await this.connection.getAccountInfo(sslOut);

    if (!tokenASSLData) throw "Cannot get SSL for tokenA";
    if (!tokenBSSLData) throw "Cannot get SSL for tokenB";
    if (!pairData) throw "Cannot get Pair";

    const decoded = PAIR_LAYOUT.decode(pairData.data);
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

    const out = swapWASM(
      tokenASSLData.data,
      tokenBSSLData.data,
      pairData.data,
      registry,
      inTokenAmount,
      BigInt(0)
    );

    const finalResult: Quote = {
      out: out.out,
      impact: out.price_impact,
    };

    return finalResult;
  };

  public getMinimumQuote = async (
    tokenA: PublicKey,
    tokenB: PublicKey,
    inTokenAmount: BigInt,
    slippage: number,
  ) => {
    const result = await this.getQuote(tokenA, tokenB, inTokenAmount);
    const minAmountOut = result.out as bigint * (10000n - BigInt(slippage * 10000)) / 10000n;
    return minAmountOut;
  };

  public createSwapIx = async (
    tokenA: PublicKey,
    tokenB: PublicKey,
    inTokenAmount: BigInt,
    minOut: BigInt,
    wallet: PublicKey,
  ): Promise<Array<TransactionInstruction>> => {
    let ixs = [];

    const program = new Program(
      SwapIDL as Idl,
      POOL_ADDRESS.toBase58(),
      new Provider(this.connection, wallet as any, {
        commitment: "processed",
      })
    );
    const inst: any = program.instruction;
    const pair = this.getPairAddress(tokenA, tokenB);

    const [inTokenAtaUser, outTokenAtaUser] = [
      findAssociatedTokenAddress(wallet, tokenA),
      findAssociatedTokenAddress(wallet, tokenB),
    ];

    const sslIn = this.getSSLAddress(tokenA);
    const sslOut = this.getSSLAddress(tokenB);
    const vaultIn = findAssociatedTokenAddress(sslIn, tokenA);
    const vaultOut = findAssociatedTokenAddress(sslOut, tokenB);

    if (!(await this.connection.getAccountInfo(outTokenAtaUser))) {
      ixs.push(
        this.createAssociatedTokenAccountIx(tokenB, outTokenAtaUser, wallet)
      );
    }
    const pairData = await this.connection.getAccountInfo(pair);
    if (!pairData || !pairData.data)
      throw new Error("Token Pair do not exist yet.");

    const tokenAccountB = findAssociatedTokenAddress(wallet, tokenB);
    if (
      tokenB.toBase58() !== NATIVE_MINT.toBase58() &&
      !(await this.connection.getParsedAccountInfo(tokenAccountB)).value
    ) {
      ixs.push(
        this.createAssociatedTokenAccountIx(tokenB, tokenAccountB, wallet)
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
      sslIn: sslIn,
      sslOut: sslOut,
      mintIn: tokenA,
      mintOut: tokenB,
      vaultIn,
      vaultOut,
      userWallet: wallet,
      userInAta: inTokenAtaUser,
      userOutAta: outTokenAtaUser,
      instructions: new PublicKey(
        "Sysvar1nstructions1111111111111111111111111"
      ),
      feeCollectorAta: findAssociatedTokenAddress(collector, tokenA),
      feeCollector: collector,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
      systemProgram: SYSTEM,
      rent: SYSVAR_RENT_PUBKEY,
    };

    ixs.push(
      await inst.rebalanceSwap(inTokenAmount, minOut, {
        accounts,
        remainingAccounts,
      })
    );
    ixs.push(await inst.preSwap({ accounts, remainingAccounts }));
    ixs.push(await inst.swap({ accounts, remainingAccounts }));

    return ixs;
  };
}
