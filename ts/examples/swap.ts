import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { Swap } from "../src";


const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const wallet = new Keypair();
  const swap = new Swap(connection);

  const ixs = await swap.createSwapIx(
    new PublicKey("So11111111111111111111111111111111111111112"),
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
    100000n, // 0.0001 SOL
    100n, // 0.0001 USDC 
    wallet.publicKey
  );

  let tx = new Transaction();
  for (const ix of ixs) {
    tx.add(ix);
  }

  // Send out the tx use browser wallet or keypair
}

main();
