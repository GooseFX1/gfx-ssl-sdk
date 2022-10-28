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
import { assert } from "console";
import { ADDRESSES, mergeu64, splitu64, SSL, Swap } from "../src";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);


test("SOL pool is not suspended", async () => {
  let ssl = await SSL.loadByMint(
    connection,
    ADDRESSES["MAINNET"].GFX_CONTROLLER,
    new PublicKey("So11111111111111111111111111111111111111112")
  );

  const suspended = ssl!.isSuspended();

  assert(!suspended);
});

test("SOL/USDC pair is not suspended", async () => {
  const swap = new Swap(connection);
  const quoter = await swap.getQuoter(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
  );
  await quoter.prepare();
  const suspended = quoter.isSuspended();

  console.log(`SOL/USDC suspended: ${suspended}`);
  assert(!suspended);
});

test("should swap", async () => {
  const swap = new Swap(connection);

  const { amountOut: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );

  console.log(`out: ${outAmount} ${impact}`);

});

test("should swap multiple times", async () => {
  const swap = new Swap(connection);

  const quoter = await swap.getQuoter(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
  );

  await quoter.prepare();

  for (let i = 0; i < 3; i += 1) {
    const { amountOut: outAmount, impact } = quoter.quote(1000000n);
    console.log(`out: ${outAmount} ${impact}`);
  }

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

test("split/merge u64 to/from two numbers", async () => {
  const u32CvtShim = new Uint32Array(2);
  const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

  const number = 588410519551n;
  const [low, high] = splitu64(number);
  uint64CvtShim[0] = number;

  expect(low).toBe(u32CvtShim[0]);
  expect(high).toBe(u32CvtShim[1]);

  const number_ = mergeu64(low, high);

  expect(number_).toBe(number);

});
