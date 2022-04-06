
import * as lo from '@solana/buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";

export interface LiquidityAccountLayout {
    sighash: Uint8Array,
    mint: PublicKey,
    bump: number,
    share: BigInt,
    ptMinted: BigInt,
    amountDeposited: BigInt,
}

export const LIQUIDITY_ACCOUNT_LAYOUT = lo.struct<LiquidityAccountLayout>([
    lo.blob(8, "sighash"),
    publicKey("mint"),
    lo.u8("bump"),
    lo.blob(7),
    u64("share"),
    u64("ptMinted"),
    u64("amountDeposited"),
    lo.blob(248, "padding")
]);


if (LIQUIDITY_ACCOUNT_LAYOUT.span != 312 + 8) {
    throw new Error("LIQUIDITY_ACCOUNT LAYOUT SIZE MISMATCH");
}