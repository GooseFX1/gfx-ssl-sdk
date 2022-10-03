import * as lo from '@solana/buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";

export interface ControllerLayout {
    sighash: Uint8Array,
    seed: Buffer,
    bump: number,
    admin: PublicKey,
    suspended: number,
    decimals: number,
    mint: PublicKey,
    dailyReward: bigint,
    totalStakingShare: bigint,
    stakingBalance: bigint,
    lastDistributionTime: bigint,
    withdrawFee: number,
}

export const CONTROLLER_LAYOUT = lo.struct<ControllerLayout>([
    lo.blob(8, "sighash"),
    lo.blob(32, "seed"),
    lo.u8("bump"),
    publicKey("admin"),
    lo.u8("suspended"),
    lo.u8("decimals"),
    publicKey("mint"),
    lo.blob(5, "padding"),
    u64("dailyReward"),
    u64("totalStakingShare"),
    u64("stakingBalance"),
    u64("lastDistributionTime"),
    lo.u16("withdrawFee"),
    lo.blob(254, "padding")
]);

if (CONTROLLER_LAYOUT.span != 392 + 8) {
    throw new Error("CONTROLLER LAYOUT SIZE MISMATCH");
}