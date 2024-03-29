import { Program, Provider, Idl, BN } from "@project-serum/anchor";
import { PAIR_LAYOUT, SSL_LAYOUT } from "../layouts";
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
import { ADDRESSES } from "../constants";
import * as SwapIDL from "../idl/gfx_ssl_idl.json";
import { findAssociatedTokenAddress } from "./utils";
import wasmData from "../wasm/gfx_ssl_wasm_data";
import init, * as wasm from "../wasm/gfx_ssl_wasm";
import { getAccount } from "@solana/spl-token";
import { SSL } from "./ssl";
import { parsePriceData, PriceStatus } from "@pythnetwork/client";
export { default as wasmData } from "../wasm/gfx_ssl_wasm_data";

let wasmInited = false;

export interface Quote {
  amountIn: bigint;
  fee: bigint;
  amountOut: bigint;
  impact: number;
  swapPrice: number;
  instantPrice: number;
  oraclePrice: number;
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

  public getQuoter = async (
    tokenIn: PublicKey,
    tokenOut: PublicKey,
  ): Promise<Quoter> => {
    let wasm = await this.getWasm();
    return new Quoter(this.connection, this.programId, this.controller, tokenIn, tokenOut, wasm);
  };

  public getQuote = async (
    tokenIn: PublicKey,
    tokenOut: PublicKey,
    inTokenAmount: bigint,
    silent: boolean = true,
  ): Promise<Quote> => {
    const quoter = await this.getQuoter(tokenIn, tokenOut);
    await quoter.prepare();
    return quoter.quote(inTokenAmount, silent);
  };

  public getMinimumQuote = async (
    tokenA: PublicKey,
    tokenB: PublicKey,
    inTokenAmount: bigint,
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
    inTokenAmount: bigint,
    minOut: bigint,
    wallet: PublicKey,
    referrerTokenAccount?: PublicKey, // referrerTokenAccount in TokenA
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

type Prepared = {
  pairData: Buffer;
  sslInData: Buffer;
  sslOutData: Buffer;
  liabilityIn: bigint;
  swappedLiabilityIn: bigint;
  liabilityOut: bigint;
  swappedLiabilityOut: bigint;
  registry: wasm.OracleRegistry;
  suspended: boolean;
  publishedSlots: Array<bigint>;
  pythStatus: Array<PriceStatus>;
  pythConfs: Array<number>;
  confidence: bigint;
  maxDelay: bigint;
};

class Quoter {
  private prepared: Prepared | undefined = undefined;

  constructor(
    public connection: Connection,
    public programId: PublicKey,
    public controller: PublicKey,
    public tokenIn: PublicKey,
    public tokenOut: PublicKey,
    public wasm: any
  ) { }

  async prepare() {
    const pair = this.getPairAddress(this.tokenIn, this.tokenOut);
    const pairData = await this.connection.getAccountInfo(pair);
    if (!pairData) throw "Cannot get Pair";

    const sslIn = SSL.findAddress(
      this.controller,
      this.tokenIn,
      this.programId
    );
    const sslInData = await this.connection.getAccountInfo(sslIn);
    if (!sslInData) throw "Cannot get SSL for tokenIn";

    const sslOut = SSL.findAddress(
      this.controller,
      this.tokenOut,
      this.programId
    );
    const sslOutData = await this.connection.getAccountInfo(sslOut);
    if (!sslOutData) throw "Cannot get SSL for tokenOut";

    const liabilityVaultIn = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslIn, this.tokenIn)
    );

    const swappedLiabilityVaultIn = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslIn, this.tokenOut)
    );

    const liabilityVaultOut = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslOut, this.tokenOut)
    );

    const swappedLiabilityVaultOut = await getAccount(
      this.connection,
      findAssociatedTokenAddress(sslOut, this.tokenIn)
    );

    const OracleRegistry = wasm.OracleRegistry;
    const decoded = PAIR_LAYOUT.decode(pairData.data);
    const { maxDelay, oracles, nOracle } = decoded;
    const n = Number(nOracle.toString());
    let publishedSlots = [];
    let pythStatus = [];
    let pythConfs = [];
    const registry = new OracleRegistry();
    for (const oracle of oracles.slice(0, n)) {
      const n = Number(oracle.n);

      for (const elem of oracle.elements.slice(0, n)) {
        const acctInfo = await this.connection.getAccountInfo(elem.address);
        if (acctInfo?.data) {
          registry.add_oracle(elem.address.toBuffer(), acctInfo.data);
          let priceData = parsePriceData(acctInfo.data);
          publishedSlots.push(priceData.aggregate.publishSlot);
          pythStatus.push(priceData.aggregate.status);

          let price = priceData.aggregate.price / Math.pow(10, -priceData.exponent);
          let std = priceData.aggregate.confidence / Math.pow(10, -priceData.exponent);
          pythConfs.push(price / std);
        }
      }
    }

    this.prepared = {
      pairData: pairData.data,
      sslInData: sslInData.data,
      sslOutData: sslOutData.data,
      liabilityIn: liabilityVaultIn.amount,
      swappedLiabilityIn: swappedLiabilityVaultIn.amount,
      liabilityOut: liabilityVaultOut.amount,
      swappedLiabilityOut: swappedLiabilityVaultOut.amount,
      registry: registry,
      suspended: (new SSL(sslInData)).isSuspended() || (new SSL(sslOutData)).isSuspended(),
      publishedSlots: publishedSlots.map((val) => BigInt(val)),
      pythStatus,
      pythConfs,
      confidence: decoded.confidence,
      maxDelay: maxDelay
    };
  }

  public isSuspended(currentSlot?: bigint): boolean {
    if (this.prepared === undefined) throw "Run prepare first";
    let suspended = this.prepared.suspended;
    if (currentSlot !== undefined) {
      for (const pubSlot of this.prepared.publishedSlots) {
        suspended ||= pubSlot + this.prepared.maxDelay <= currentSlot;
      }
    }

    for (const pythStatus of this.prepared.pythStatus) {
      suspended ||= pythStatus !== PriceStatus.Trading;
    }

    for (const conf of this.prepared.pythConfs) {
      suspended ||= this.prepared.confidence > conf;
    }

    return suspended;
  }

  public quote(inTokenAmount: bigint, silent: boolean = true, niter: number = 64): Quote {
    const swapWASM = wasm.swap;

    if (inTokenAmount === 0n) return {
      amountIn: 0n,
      fee: 0n,
      amountOut: 0n,
      impact: 0,
      swapPrice: 0,
      instantPrice: 0,
      oraclePrice: 0,
    };

    if (this.prepared === undefined) throw "Run prepare first";
    const prepared = this.prepared;

    let out;
    try {
      out = swapWASM(
        prepared.sslInData.slice(),
        prepared.sslOutData.slice(),
        prepared.pairData.slice(),
        prepared.liabilityIn,
        prepared.liabilityOut,
        prepared.swappedLiabilityIn,
        prepared.swappedLiabilityOut,
        prepared.registry,
        inTokenAmount,
        niter
      );
    } catch (e) {
      if (silent) {
        return {
          amountIn: inTokenAmount,
          fee: 0n,
          amountOut: 0n,
          impact: 1,
          swapPrice: 0,
          instantPrice: 0,
          oraclePrice: 0,
        };
      } else {
        throw e;
      }
    }

    return {
      amountIn: out.amount_in,
      fee: out.fee_paid,
      amountOut: out.amount_out,
      impact: out.price_impact,
      swapPrice: out.swap_price,
      instantPrice: out.insta_price,
      oraclePrice: out.oracle_price,
    };


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
}
