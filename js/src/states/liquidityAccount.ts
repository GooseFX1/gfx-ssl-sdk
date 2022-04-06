
import BN from 'bn.js';
import * as lo from 'buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKeyLayout, u64 } from "../layout";


export interface LiquidityAccount {
    mint: PublicKey,
    bump: number,
    share: BN,
    ptMinted: BN,
}

export const Liquidity_ACCOUNT_LAYOUT = lo.struct<LiquidityAccount>([
    lo.blob(8, 'sighash'),
    publicKeyLayout("mint"),
    lo.u8("bump"),
    lo.blob(7),
    u64("share"),
    u64('ptMinted'),
    lo.blob(256, "padding")
]);


if (Liquidity_ACCOUNT_LAYOUT.span != 312 + 8) {
    throw new Error("LIQUIDITY_ACCOUNT LAYOUT SIZE MISMATCH");
}