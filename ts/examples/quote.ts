// 1. Compile this repo: cd ts && yarn build
// 2. Use ts-node to run this file: `cd ts && ts-node examples/quote.ts`
import { PublicKey, Connection } from "@solana/web3.js";
import { gfxSSL } from "../dist";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

const quote = async () => {
  const swap = new gfxSSL.Swap(connection);
  const { out: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );
  console.log(`out: ${outAmount} ${impact}`);
  return { outAmount, impact };
};

quote();
