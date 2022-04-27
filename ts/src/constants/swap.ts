import { PublicKey } from "@solana/web3.js";
import { Tokens } from "../states/pair";

export const poolAddress = new PublicKey(
  "7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5"
);
export const poolController = new PublicKey(
  "8CxKnuJeoeQXFwiG6XiGY2akBjvJA5k3bE52BfnuEmNQ"
);

export const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);

export const SYSTEM = new PublicKey("11111111111111111111111111111111");

export const tokens: Tokens = {
  EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v: {
    address: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    symbol: "USDC",
    decimals: 6,
  },
  So11111111111111111111111111111111111111112: {
    address: "So11111111111111111111111111111111111111112",
    symbol: "SOL",
    decimals: 9,
  },
};
