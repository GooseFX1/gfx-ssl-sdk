

import BN from 'bn.js';
import * as lo from '@solana/buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";

export interface Controller {
    seed: Buffer,
    bump: number,
    admin: PublicKey,
    suspended: number,
    decimals: number,
    mint: PublicKey,
    dailyReward: BN,
    totalStakingShare: BN,
    stakingBalance: BN,
    lastDistributionTime: BN,
}

export const CONTROLLER_LAYOUT = lo.struct<Controller>([
    lo.blob(8, 'sighash'),
    lo.blob(32, "seed"),
    lo.u8("bump"),
    publicKey("admin"),
    lo.u8("suspended"),
    lo.u8("decimals"),
    publicKey("mint"),
    lo.blob(5, "padding"),
    u64("dailyReward"),
    u64('totalStakingShare'),
    u64('stakingBalance'),
    u64('lastDistributionTime'),
    lo.blob(256, "padding")
]);

if (CONTROLLER_LAYOUT.span != 392 + 8) {
    throw new Error("CONTROLLER LAYOUT SIZE MISMATCH");
}