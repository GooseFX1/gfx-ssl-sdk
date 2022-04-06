import { publicKeyLayout, u64, u128 } from "../layout";
import * as lo from "buffer-layout";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

export const Oracle = lo.struct([
    lo.seq(
        lo.struct([
            publicKeyLayout("address"),
            lo.u8("inverse"),
            lo.u8(), // padding
        ]),
        4,
        "elements"
    ),
    u64("n"),
    lo.blob(8 * 8), // padding
]);

export interface Pair {
    controller: PublicKey,
    mints: Array<PublicKey>,
    oracles: Array<{ elements: Array<{ address: PublicKey, inverse: number; }>, n: BN; }>,
    nOracle: BN,
    fee_collector: PublicKey;
}

export const PAIR_LAYOUT = lo.struct<Pair>([
    lo.blob(8, "sighash"),
    publicKeyLayout('controller'),
    lo.seq(publicKeyLayout("mint"), 2, 'mints'),
    lo.blob(8),// padding for alignment
    lo.seq(Oracle, 5, "oracles"),
    u64("nOracle"),
    lo.u8("A"),
    lo.seq(lo.u8(), 2, 'feeRates'),
    lo.blob(5), // padding
    u64("maxDelay"),
    u64("confidence"),
    publicKeyLayout('balancer'),
    lo.u16("excessiveConfiscateRate"),
    publicKeyLayout('feeCollector'),
    lo.seq(lo.u16(), 2, 'platformFeeRate'),
    lo.seq(lo.u8(), 2, 'rebalanceRebates'),
    lo.seq(u64("surplus"), 2, 'surpluses'),
    lo.seq(u128("volumes"), 2, 'volumes'),
    lo.u8("enableRebalanceSwap"),
    lo.blob(80, "swapCache"),
    lo.blob(151, "padding")
]);

if (PAIR_LAYOUT.span != 1528 + 8) {
    throw new Error("PAIR LAYOUT SIZE MISMATCH");
}