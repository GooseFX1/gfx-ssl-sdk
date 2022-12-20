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
import { ADDRESSES, SSL, Swap } from "../src";

const connection = new Connection(
  "https://solana-api.syndica.io/access-token/0gySHOxquJsGffID7CEWJRa03x53taOZIFcULMNPmd5aDJ1gs3Hd2zISJumMMdf8/rpc",
  "finalized"
);

const TOKENS_TO_TEST = [
  {
    mint1: "So11111111111111111111111111111111111111112",
    name1: "SOL",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 9n,
    decimal2: 6n,
  },
  {
    mint1: "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs",
    name1: "ETH",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 8n,
    decimal2: 6n,
  },
  {
    mint1: "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
    name1: "MSOL",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 9n,
    decimal2: 6n,
  },
  {
    mint1: "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt",
    name1: "SRM",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 6n,
    decimal2: 6n,
  },
  {
    mint1: "7i5KKsX2weiTkry7jA4ZwSuXGhs5eJBEjY8vVxR4pfRx",
    name1: "GMT",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 9n,
    decimal2: 6n,
  },
  {
    mint1: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    name1: "USDT",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 6n,
    decimal2: 6n,
  },
  {
    mint1: "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE",
    name1: "ORCA",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 6n,
    decimal2: 6n,
  },
  {
    mint1: "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj",
    name1: "stSOL",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 9n,
    decimal2: 6n,
  },
  {
    mint1: "6LNeTYMqtNm1pBFN8PfhQaoLyegAH8GD32WmHU9erXKN",
    name1: "APT",
    mint2: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    name2: "USDC",
    decimal1: 8n,
    decimal2: 6n,
  },
];

const displayPrice = (price: bigint, decimal: bigint) => {
  if (price.toString().length > Number(decimal))
    return (
      price.toString().substring(0, price.toString().length - Number(decimal)) +
      "." +
      price
        .toString()
        .substring(
          price.toString().length - Number(decimal),
          price.toString().length
        )
    );
  if (price.toString().length === Number(decimal)) return "0." + price;
  let strToReturn = "0.";
  for (let i = 0; i < Number(decimal) - price.toString().length; i++)
    strToReturn += "0";
  strToReturn += price;
  return strToReturn;
};

test(
  "SOL pool is not suspended",
  async () => {
    let ssl = await SSL.loadByMint(
      connection,
      ADDRESSES["MAINNET"].GFX_CONTROLLER,
      new PublicKey("So11111111111111111111111111111111111111112")
    );

    const suspended = ssl!.isSuspended();

    expect(suspended).toBe(false);
  },
  10 * 1000
);

test(
  "SOL/USDC pair is not suspended",
  async () => {
    const swap = new Swap(connection);
    const quoter = await swap.getQuoter(
      new PublicKey("So11111111111111111111111111111111111111112"), //SOL
      new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") //USD
    );
    await quoter.prepare();
    const suspended = quoter.isSuspended();

    console.log(`SOL/USDC suspended: ${suspended}`);
    expect(suspended).toBe(false);
  },
  10 * 1000
);

test(
  "SOL/USDC pair is not suspended with latest oracle update",
  async () => {
    const swap = new Swap(connection);
    const quoter = await swap.getQuoter(
      new PublicKey("So11111111111111111111111111111111111111112"), //SOL
      new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") //USD
    );
    await quoter.prepare();
    let slot = await connection.getSlot();
    const suspended = quoter.isSuspended(BigInt(slot));

    console.log(`SOL/USDC suspended: ${suspended}`);
    expect(suspended).toBe(false);
  },
  10 * 1000
);

test(
  "SOL/USDC pair is suspended with 100 slot oracle lag",
  async () => {
    const swap = new Swap(connection);
    const quoter = await swap.getQuoter(
      new PublicKey("So11111111111111111111111111111111111111112"), //SOL
      new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") //USD
    );
    await quoter.prepare();
    let slot = await connection.getSlot();
    const suspended = quoter.isSuspended(BigInt(slot) + 100n);

    console.log(`SOL/USDC suspended: ${suspended}`);
    expect(suspended).toBe(true);
  },
  10 * 1000
);

test.only(
  "should swap",
  async () => {
    const swap = new Swap(connection);

    for (let i = 0; i < TOKENS_TO_TEST.length; i++) {
      const {
        amountOut: outAmount1,
        impact: impact1,
        oraclePrice: oraclePrice1,
      } = await swap.getQuote(
        new PublicKey(TOKENS_TO_TEST[i].mint1),
        new PublicKey(TOKENS_TO_TEST[i].mint2),
        10n ** (TOKENS_TO_TEST[i].decimal1 + 1n)
      );

      console.log(
        "10 " + TOKENS_TO_TEST[i].name1 + " --> " + TOKENS_TO_TEST[i].name2,
        `out: ${displayPrice(
          outAmount1,
          TOKENS_TO_TEST[i].decimal2
        )} \n oracle price: ${oraclePrice1} \n impact: ${impact1}`
      );

      const {
        amountOut: outAmount2,
        impact: impact2,
        oraclePrice: oraclePrice2,
      } = await swap.getQuote(
        new PublicKey(TOKENS_TO_TEST[i].mint2),
        new PublicKey(TOKENS_TO_TEST[i].mint1),
        10n ** (TOKENS_TO_TEST[i].decimal2 + 1n)
      );

      console.log(
        "10 " + TOKENS_TO_TEST[i].name2 + " --> " + TOKENS_TO_TEST[i].name1,
        `out: ${displayPrice(
          outAmount2,
          TOKENS_TO_TEST[i].decimal1
        )} \n impact: ${impact2}`
      );
    }
  },
  100 * 1000
);

test(
  "should swap multiple times",
  async () => {
    const swap = new Swap(connection);

    const quoter = await swap.getQuoter(
      new PublicKey("So11111111111111111111111111111111111111112"), //SOL
      new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") //USD
    );

    await quoter.prepare();

    for (let i = 0; i < 3; i += 1) {
      const { amountOut: outAmount, impact } = quoter.quote(1000000n);
      console.log(`out: ${outAmount} ${impact}`);
    }
  },
  10 * 1000
);

test(
  "is adding 1000000 additional ComputeBudget Instruction",
  async () => {
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

    expect(result[0].programId).toBe(
      "ComputeBudget111111111111111111111111111111"
    );
    expect(result[0].data[0]).toBe(0);
    expect(JSON.stringify(result[0].data.slice(1, 5))).toBe("[64,66,15,0]");
    expect(JSON.stringify(result[0].data.slice(5))).toBe("[0,0,0,0]");
  },
  10 * 1000
);
