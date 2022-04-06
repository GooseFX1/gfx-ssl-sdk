// 1. Compile this repo: cd ts && yarn build
// 2. Use ts-node to run this file: `cd ts && ts-node examples/quote.ts`
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { Swap } from "../src"; // Change "../src" to "goosefx-ssl-sdk" for using the NPM package

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function createTx(wallet: any) {
  //const wallet = new Keypair();
  const swap = new Swap(connection);

  // wrap SOL token if Sol is first token pair
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

  // add unwrap sol token txn if sol is present as resulting token pair

  // Send out the tx use browser wallet or keypair
  return tx;
}

export default createTx;
