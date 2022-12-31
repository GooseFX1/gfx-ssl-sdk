// 1. Compile this repo: cd ts && yarn build
// 2. Use ts-node to run this file: `cd ts && ts-node examples/quote.ts`
import { PublicKey, Connection } from "@solana/web3.js";
import { Swap } from "../src"; // Change "../src" to "goosefx-ssl-sdk" for using the NPM package

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

const quote = async (input: bigint) => {
  const swap = new Swap(connection);
  const quoter = await swap.getQuoter(
    new PublicKey("So11111111111111111111111111111111111111112"), // SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") // USDC
  );
  await quoter.prepare();
  const quote = quoter.quote(input, false);
  console.log("quote:", quote);
  console.log("suspended:", quoter.isSuspended());
};

quote(BigInt(process.argv[2]));