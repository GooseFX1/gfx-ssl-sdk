import * as lo from "@solana/buffer-layout";
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64, u128, bool } from "@solana/buffer-layout-utils";
import { ISwapToken } from ".";

interface Oracle {
  elements: Array<{ address: PublicKey; inverse: boolean }>;
  n: bigint;
  padding: Uint8Array;
}

interface OracleComponent {
  address: PublicKey;
  inverse: boolean;
  padding: Uint8Array;
}

export const ORACLE_ELEMENT_LAYOUT = lo.struct<OracleComponent>([
  publicKey("address"),
  bool("inverse"),
  lo.blob(1, "padding"), // padding
]);

export const ORACLE_LAYOUT = lo.struct<Oracle>([
  lo.seq(ORACLE_ELEMENT_LAYOUT, 4, "elements"),
  u64("n"),
  lo.blob(8 * 8), // padding
]);

export interface Tokens {
  [So11111111111111111111111111111111111111112: string]: ISwapToken;
  EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v: ISwapToken;
}
export interface Pair {
  sighash: Uint8Array;
  controller: PublicKey;
  mints: Array<PublicKey>;
  oracles: Array<Oracle>;
  nOracle: BigInt;
  fee_collector: PublicKey;
  A: number;
  feeRates: Array<number>;
  maxDelay: BigInt;
  confidence: BigInt;
  balancer: PublicKey;
  excessiveConfiscateRate: number;
  feeCollector: PublicKey;
  platformFeeRate: Array<number>;
  rebalanceRebates: Array<number>;
  surpluses: Array<BigInt>;
  volumes: Array<BigInt>;
  enableRebalanceSwap: boolean;
}

export const PAIR_LAYOUT = lo.struct<Pair>([
  lo.blob(8, "sighash"),
  publicKey("controller"),
  lo.seq(publicKey("mint"), 2, "mints"),
  lo.blob(8), // padding for alignment
  lo.seq(ORACLE_LAYOUT, 5, "oracles"),
  u64("nOracle"),
  lo.u8("A"),
  lo.seq(lo.u8(), 2, "feeRates"),
  lo.blob(5), // padding
  u64("maxDelay"),
  u64("confidence"),
  publicKey("balancer"),
  lo.u16("excessiveConfiscateRate"),
  publicKey("feeCollector"),
  lo.seq(lo.u16(), 2, "platformFeeRate"),
  lo.seq(lo.u8(), 2, "rebalanceRebates"),
  lo.seq(u64("surplus"), 2, "surpluses"),
  lo.seq(u128("volumes"), 2, "volumes"),
  bool("enableRebalanceSwap"),
  lo.blob(80, "swapCache"),
  lo.blob(151, "padding"),
]);

if (PAIR_LAYOUT.span != 1528 + 8) {
  throw new Error("PAIR LAYOUT SIZE MISMATCH");
}
