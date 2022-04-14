import * as lo from '@solana/buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64, bool } from "@solana/buffer-layout-utils";

export interface SSL {
    sighash: Uint8Array,
    controller: PublicKey,
    mint: PublicKey,
    decimals: number,
    bump: number,
    ptBump: number,
    suspended: boolean,
    cranker: PublicKey,
    weight: BigInt,
    liablity: BigInt,
    swappedLiability: BigInt,
    totalShare: BigInt,
}

export const SSL_LAYOUT = lo.struct<SSL>([
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
    u64("liability"),
    u64("swappedLiability"),
    u64("totalShare"),

    lo.blob(256, "padding")
]);

if (SSL_LAYOUT.span != 392 + 8) {
    throw new Error("SSL LAYOUT SIZE MISMATCH");
}