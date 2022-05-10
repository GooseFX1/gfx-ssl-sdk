const solana = require("@solana/web3.js");
import { Swap } from "../dist";

const connection = new solana.Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

const quote = async () => {
  const swap = new Swap(connection);
  const { out: outAmount, impact } = await swap.getQuote(
    new solana.PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new solana.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    //@ts-ignore
    new BigInt(1000000)
  );
  console.log(`out: ${outAmount} ${impact}`);
  return { outAmount, impact };
};

export default quote;
