import { Connection, PublicKey } from "@solana/web3.js";
import { Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const swap = new Swap(connection);
  const { out: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );

  console.log(`out: ${outAmount} ${impact}`);
}

main();
