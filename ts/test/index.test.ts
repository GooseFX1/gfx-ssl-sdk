import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  ComputeBudgetProgram,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { ADDRESSES, SSL, Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

const TOKENS_TO_TEST = [
  {
    mint1: "So11111111111111111111111111111111111111112",
    name1: "SOL",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    amount_1_2: 10_000_000_000n,
    amount_2_1: 10_000_000n,
  },
];

test("should swap", async () => {
  jest.useFakeTimers("legacy");
  jest.setTimeout(60000);

  const swap = new Swap(connection);

  for (let i = 0; i < TOKENS_TO_TEST.length; i++) {
    const { amountOut: outAmount1, impact: impact1 } = await swap.getQuote(
      new PublicKey(TOKENS_TO_TEST[i].mint1), //SOL
      new PublicKey(TOKENS_TO_TEST[i].mint2), //USD
      TOKENS_TO_TEST[i].amount_1_2
    );

    console.log(
      TOKENS_TO_TEST[i].name1 + " --> " + TOKENS_TO_TEST[i].name2,
      `out: ${outAmount1} ${impact1}`
    );

    const { amountOut: outAmount2, impact: impact2 } = await swap.getQuote(
      new PublicKey(TOKENS_TO_TEST[i].mint2), //SOL
      new PublicKey(TOKENS_TO_TEST[i].mint1), //USD
      TOKENS_TO_TEST[i].amount_2_1
    );

    console.log(
      TOKENS_TO_TEST[i].name2 + " --> " + TOKENS_TO_TEST[i].name1,
      `out: ${outAmount2} ${impact2}`
    );
  }
});
