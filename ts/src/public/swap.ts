import { Program, Provider, Idl, BN } from "@project-serum/anchor";
import { PAIR_LAYOUT } from "../layouts";
import {
  TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import {
  Connection,
  ComputeBudgetProgram,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import { Buffer } from "buffer";
import { Network, ADDRESSES } from "../constants";
import * as SwapIDL from "../idl/gfx_ssl_idl.json";
import { findAssociatedTokenAddress } from "./utils";
import wasmData from "../wasm/gfx_ssl_wasm_data";
import init, * as wasm from "../wasm/gfx_ssl_wasm";
import { getAccount } from "@solana/spl-token";
import { SSL } from "./ssl";

let wasmInited = false;

export interface Quote {
  out: BigInt;
  impact: number;
}

export class Swap {
  constructor(
    public connection: Connection,
    public controller: PublicKey = ADDRESSES["MAINNET"].GFX_CONTROLLER,
    public programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID
  ) { }

  public async getWasm() {
    if (!wasmInited) {
      await init(Buffer.from(wasmData, "base64"));
      wasmInited = true;
    }
    return wasm;
  }

  public getPairAddress = (tokenA: PublicKey, tokenB: PublicKey) => {
    const addresses = [tokenA.toBuffer(), tokenB.toBuffer()].sort(
      Buffer.compare
    );

    const pairArr = PublicKey.findProgramAddressSync(
      [
        Buffer.from("GFX-SSL-Pair", "utf-8"),
        this.controller.toBuffer(),
        addresses[0],
        addresses[1],
      ],
      this.programId
    );

    return pairArr[0];
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

  public getQuote = async (
    tokenIn: PublicKey,
    tokenOut: PublicKey,
    inTokenAmount: BigInt
  ): Promise<Quote> => {
    let wasm = await this.getWasm();

    const swapWASM = wasm.swap;
    const OracleRegistry = wasm.OracleRegistry;
    if (inTokenAmount === 0n) return { impact: 0, out: 0n };

    const pair = this.getPairAddress(tokenIn, tokenOut);
    const pairData = await this.connection.getAccountInfo(pair);
    const sslIn = SSL.findAddress(this.controller, tokenIn, this.programId);
    const sslOut = SSL.findAddress(this.controller, tokenOut, this.programId);
    const tokenASSLData = await this.connection.getAccountInfo(sslIn);
    const tokenBSSLData = await this.connection.getAccountInfo(sslOut);

    const liabilityVaultIn = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslIn, tokenIn)
    );
    const swappedLiabilityVaultIn = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslIn, tokenOut)
    );
    const liabilityVaultOut = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslOut, tokenOut)
    );
    const swappedLiabilityVaultOut = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslOut, tokenIn)
    );

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
      liabilityVaultIn.amount,
      liabilityVaultOut.amount,
      swappedLiabilityVaultIn.amount,
      swappedLiabilityVaultOut.amount,
      registry,
      inTokenAmount
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
    slippage: number
  ) => {
    const result = await this.getQuote(tokenA, tokenB, inTokenAmount);
    const minAmountOut =
      //@ts-ignore
      ((result.out as bigint) * (10000n - BigInt(slippage * 10000))) / 10000n;
    return minAmountOut;
  };

  public createSwapIx = async (
    tokenA: PublicKey,
    tokenB: PublicKey,
    inTokenAmount: BigInt,
    minOut: BigInt,
    wallet: PublicKey,
    referrerTokenAccount: PublicKey | undefined, // referrerTokenAccount in TokenA
  ): Promise<Array<TransactionInstruction>> => {
    let ixs = [];

    const addedComputeBudgetIX: TransactionInstruction =
      ComputeBudgetProgram.requestUnits({
        units: 1000000,
        additionalFee: 0,
      });

    ixs.push(addedComputeBudgetIX);

    const program = new Program(
      SwapIDL as Idl,
      this.programId.toBase58(),
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

    const sslIn = SSL.findAddress(this.controller, tokenA, this.programId);
    const sslOut = SSL.findAddress(this.controller, tokenB, this.programId);

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
    if (referrerTokenAccount !== undefined) {
      remainingAccounts.push({ isSigner: false, isWritable: true, pubkey: referrerTokenAccount });
    }
    for (const oracle of oracles.slice(0, n)) {
      for (const elem of oracle.elements.slice(0, Number(oracle.n))) {
        remainingAccounts.push({
          isSigner: false,
          isWritable: false,
          pubkey: elem.address,
        });
      }
    }

    const accounts = {
      controller: this.controller,
      pair,
      sslIn: sslIn,
      sslOut: sslOut,

      liabilityVaultIn: findAssociatedTokenAddress(sslIn, tokenA),
      liabilityVaultOut: findAssociatedTokenAddress(sslOut, tokenB),

      swappedLiabilityVaultIn: findAssociatedTokenAddress(sslIn, tokenB),
      swappedLiabilityVaultOut: findAssociatedTokenAddress(sslOut, tokenA),

      userInAta: inTokenAtaUser,
      userOutAta: outTokenAtaUser,

      feeCollectorAta: findAssociatedTokenAddress(feeCollector, tokenA),

      userWallet: wallet,
      feeCollector: feeCollector,

      tokenProgram: TOKEN_PROGRAM_ID,
    };

    ixs.push(
      await inst.swap(
        new BN(inTokenAmount.toString()),
        new BN(minOut.toString()),
        { accounts, remainingAccounts }
      )
    );

    return ixs;
  };
}
