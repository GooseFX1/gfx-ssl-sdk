import * as lo from "@solana/buffer-layout";
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64, bool } from "@solana/buffer-layout-utils";

export interface SSLLayout {
  sighash: Uint8Array;
  controller: PublicKey;
  mint: PublicKey;
  decimals: number;
  bump: number;
  ptBump: number;
  suspended: boolean;
  cranker: PublicKey;
  weight: BigInt;
  swappedLiabilityNative: BigInt;
  totalShare: BigInt;
}

export const SSL_LAYOUT = lo.struct<SSLLayout>([
  lo.blob(8, "sighash"),
  publicKey("controller"),
  publicKey("mint"),
  lo.u8("decimals"),
  lo.u8("bump"),
  lo.u8("ptBump"),
  bool("suspended"),
  publicKey("cranker"),
  lo.blob(4), // padding
  u64("weight"),
  lo.blob(8),
  u64("swappedLiabilityNative"),
  u64("totalShare"),

  lo.blob(256, "padding"),
]);

if (SSL_LAYOUT.span != 392 + 8) {
  throw new Error(`SSL LAYOUT SIZE MISMATCH", ${SSL_LAYOUT.span}, ${SSL_LAYOUT.span - 8}`);
}
