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

## Get Quotes and price Impact

```typescript
import { Connection } from "@solana/web3.js";
import { Swap } from "goosefx-ssl-sdk;

const connection = new Connection(
  "https://api.mainnet-beta.solana.com/",
  "finalized"
);

async function main() {
  const { out: outAmount, impact } = await swap.getQuote(
    new PublicKey("So11111111111111111111111111111111111111112"), //SOL
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), //USD
    1000000n
  );

  console.log(`out: ${outAmount} ${impact}`);
}

main()

```

## Get minimum Amount Out and swap Tokens

```typescript
import { Connection } from "@solana/web3.js";
import { Swap } from "goosefx-ssl-sdk;

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

main()

```

# Technical Notes

**Stability of the Functions**

We hope you find the tools we used to build our API useful in the ts/src folder. Due to our on-going development of the GooseFX platform api, we cannot guarantee the stability of the SDK swap beyond 1 SOL. The SDK will be updated in later version to increase the compute units.

# Support

**Integration Questions**

Have problems integrating with the SDK? Pop by over to our [Discord](https://discord.gg/PAVyv4A2C5) #general channel and chat with one of our engineers.

**Issues / Bugs**

If you found a bug, open up an issue on github with the prefix [ISSUE](https://github.com/GooseFX1/gfx-ssl-sdk/issues). To help us be more effective in resolving the problem, be specific in the steps it took to reproduce the problem (ex. when did the issue occur, code samples, debug logs etc).

**Feedback**

Got ideas on how to improve the system? Open up an issue on github with the prefix [FEEDBACK](https://github.com/GooseFX1/gfx-ssl-sdk/issues) and let's brainstorm more about it together!

# License

[MIT](https://choosealicense.com/licenses/mit/)
