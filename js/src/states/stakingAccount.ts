
import BN from 'bn.js';
import * as lo from '@solana/buffer-layout';
import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";

export interface StakingAccount {
    controller: PublicKey,
    bump: number,
    share: BN,
    amount_staked: BN,
}

export const STAKING_ACCOUNT_LAYOUT = lo.struct<StakingAccount>([
    lo.blob(8, 'sighash'),
    publicKey("controller"),
    lo.u8("bump"),
    lo.blob(7),
    u64("share"),
    u64('amountStaked'),
    lo.blob(256, "padding")
]);


if (STAKING_ACCOUNT_LAYOUT.span != 312 + 8) {
    throw new Error("STAKING_ACCOUNT LAYOUT SIZE MISMATCH");
}