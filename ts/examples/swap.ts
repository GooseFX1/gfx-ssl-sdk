import { Connection } from "@solana/web3.js";
import { Swap } from "../src";
import { useWallet } from "@solana/wallet-adapter-react";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const wallet = useWallet();
  const { swapToken } = new Swap(connection);

  const swapResult = await swapToken(
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    100000, //0.0001 SOL
    0.001,
    wallet
  );

  if (swapResult) {
    console.log("TRANSACTION AT " + swapResult);
  } else {
    console.log("TRANSACTION FAILED");
  }
}

main();
