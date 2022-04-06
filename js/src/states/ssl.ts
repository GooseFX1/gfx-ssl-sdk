import { publicKeyLayout, u64 } from "../layout";
import * as lo from "buffer-layout";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
export interface SSL {
    controller: PublicKey,
    mint: PublicKey,
    decimals: number,
    bump: number,
    pt_bump: number,
    suspended: number,
    cranker: PublicKey,
    weight: BN,
    liablity: BN,
    swapped_liability: BN,
    total_share: BN,
}

export const SSL_LAYOUT = lo.struct<SSL>([
    lo.blob(8, "sighash"),
    publicKeyLayout('controller'),
    publicKeyLayout('mint'),
    lo.u8("decimals"),
    lo.u8("bump"),
    lo.u8("pt_bump"),
    lo.u8("suspended"),
    publicKeyLayout('cranker'),
    lo.blob(4), // padding
    u64("weight"),
    u64("liability"),
    u64("swapped_liability"),
    u64("total_share"),

    lo.blob(256, "padding")
]);

if (SSL_LAYOUT.span != 392 + 8) {
    throw new Error("SSL LAYOUT SIZE MISMATCH");
}