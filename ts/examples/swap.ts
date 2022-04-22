import { Connection } from "@solana/web3.js";
import { swapToken, getMinimumQuote } from "../src";
import { useWallet } from "@solana/wallet-adapter-react";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const wallet = useWallet();

  //get quote with slippage
  const outAmount = await getMinimumQuote(
    "So11111111111111111111111111111111111111112", //SOL
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", //USD
    1000000,
    connection,
    0.001
  );

  const swapResult = await swapToken(
    "So11111111111111111111111111111111111111112",
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    100000, //0.0001 SOL
    outAmount,
    0.001,
    wallet,
    connection
  );

  if (swapResult) {
    console.log("TRANSACTION AT " + swapResult);
  } else {
    console.log("TRANSACTION FAILED");
  }
}

main();
