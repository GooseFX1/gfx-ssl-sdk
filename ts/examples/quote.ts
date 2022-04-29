import { Connection } from "@solana/web3.js";
import { Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const { getQuote, getPriceImpact } = new Swap(connection);
  const outAmount = await getQuote(
    "So11111111111111111111111111111111111111112", //SOL
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", //USD
    1000000
  );

  const priceImpact = await getPriceImpact(
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    1000000
  );

  console.log(`out: ${outAmount} ${priceImpact}`);
}

main();
