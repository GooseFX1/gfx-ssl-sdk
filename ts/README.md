<div align="center">
  <img height="142" src="https://github.com/GooseFX1/gfx-web-app/blob/dev/public/img/assets/gfx_logo_gradient_lite.svg" />
  <h3>GooseFX Typescript SDK</h3>
  <p>The GooseFX SDK contains a set of simple to use APIs to allow developers to integrate with the GooseFX platform.</p>
  <h4>
    <a href="https://goosefx.io">Website</a>
    <span> | </span>
    <a href="https://docs.goosefx.io">Docs</a>
    <span> | </span>
    <a href="https://discord.com/channels/833693973687173121/833742620371058688">Discord</a>
    <span> | </span>
    <a href="https://www.t.me/goosefx">Telegram</a>
    <span> | </span>
    <a href="https://medium.com/goosefx">Medium</a>
  </h4>
  <br />
  <br />
</div>

### Contents

- `/ts` : contains typescript `goosefx-ssl-sdk` which creates npm package - [npmjs.com/package/goosefx-ssl-sdk](https://www.npmjs.com/package/goosefx-ssl-sdk)

### Trading GooseFX Liquidity Pools

- Get detailed quotes and make swaps between trading pairs in a GooseFx Pool
- Check your GooseFX Pool LP token balance and total supply

# Installation

Use your environment's package manager to install `goosefx-ssl-sdk` and other related packages into your project.

```bash
yarn add goosefx-ssl-sdk
```

```bash
npm install goosefx-ssl-sdk
```

# Usage

## Get Quotes and Price Impact

```typescript
import { Connection } from "@solana/web3.js";
import { Swap } from "goosefx-ssl-sdk";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

const quote = async () => {
  const swap = new Swap(connection);
  const { out: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );
  console.log(`out: ${outAmount} ${impact}`);
  return { outAmount, impact };
};
quote();
```

## Get Minimum Amount Out and Swap Tokens

```typescript
import { Connection } from "@solana/web3.js";
import { Swap } from "goosefx-ssl-sdk";

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
```

## Get Status of the SSL Pool

```typescript
import { Connection } from "@solana/web3.js";
import { SSL, ADDRESSES } from "goosefx-ssl-sdk";
import { NATIVE_MINT } from "@solana/spl-token";

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

//get the pool status for SOl native token
async function getSolPoolStatus() {
  const ssl = (await SSL.loadByMint(
    connection,
    ADDRESSES["MAINNET"].GFX_CONTROLLER,
    NATIVE_MINT
  ))!;
  const isSuspended = ssl.suspended;
  return isSuspended;
  // interprete the meaning from this: if true, the pool have been suspended and all txns to it will fail, if false the pool is active
}

getSolPoolStatus();
```

# Build

`yarn build`: this will output the bundled js in the `dist` folder.

# Technical Notes

**Stability of the Functions**

We hope you find the tools we used to build our API useful in the ts/src folder. Due to our on-going development of the GooseFX platform api, we cannot guarantee the stability of the SDK swap beyond 1 SOL. The SDK will be updated in later version to increase the compute units.

# Support

**Integration Questions**

Have problems integrating with the SDK? Pop by over to our [Discord](https://discord.gg/PAVyv4A2C5) #general channel and chat with one of our engineers.

**Issues / Bugs**

If you found a bug, open up an issue on github with the prefix [ISSUE](https://github.com/GooseFX1/gfx-ssl-sdk/issues). To help us be more effective in resolving the problem, be specific in the steps it took to reproduce the problem (ex. when did the issue occur, code samples, debug logs etc).

**Feedback**

Got ideas on how to improve the system? Open up an issue on github with the prefix [FEEDBACK] and let's brainstorm more about it together!

# Addresses

## Devnet

```
CONTROLLER_PROGRAM=3Gwyhoudx8XgYry8dzKQ2GGsofkUdm7VZUvddHxchL3x
SSL_PROGRAM=JYe7AcuQ7CqhkGvchJGvSKF8ei41FuDKb1h47qkbFNf
CONTROLLER=ApkmzBaTPUAeVj3QuqDcz6iLE6xZSLd29nke4McqrKw5
```

## Mainnet

```
CONTROLLER_PROGRAM=8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx
SSL_PROGRAM=7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5
CONTROLLER=8CxKnuJeoeQXFwiG6XiGY2akBjvJA5k3bE52BfnuEmNQ
```

# License

[MIT](https://choosealicense.com/licenses/mit/)
