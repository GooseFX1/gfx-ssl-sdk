import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

test("should swap", () => {
  const swap = new Swap(connection);

  const { out: outAmount, impact } = await swap.getQuote(
    new solana.PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new solana.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    //@ts-ignore
    new BigInt(1000000)
  );

  console.log(`out: ${outAmount} ${impact}`);

  //   expect(quote.getExpectedOutputAmount()).toEqual(
  //     new OrcaU64(new u64("241755364"), params.outputToken.scale)
  //   );
});
