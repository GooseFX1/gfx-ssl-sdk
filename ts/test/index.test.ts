import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  ComputeBudgetProgram,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

test("should swap", async () => {
  const swap = new Swap(connection);

  const { out: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );

  console.log(`out: ${outAmount} ${impact}`);

});

test("is adding 1000000 additional ComputeBudget Instruction", async () => {
  const swap = new Swap(connection);
  const wallet = new Keypair();

  const ixs = await swap.createSwapIx(
    new PublicKey("So11111111111111111111111111111111111111112"),
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
    100000n, // 0.0001 SOL
    100n, // 0.0001 USDC
    wallet.publicKey
  );

  const result = JSON.parse(JSON.stringify(ixs));

  expect(result[0].programId).toBe("ComputeBudget111111111111111111111111111111");
  expect(result[0].data[0]).toBe(0);
  expect(JSON.stringify(result[0].data.slice(1, 5))).toBe("[64,66,15,0]");
  expect(JSON.stringify(result[0].data.slice(5))).toBe("[0,0,0,0]");

});
