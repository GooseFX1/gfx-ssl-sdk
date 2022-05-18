const x = require("./dist/index.js");
console.log(Object.keys(x.default));
const { Swap } = require("./dist/index.js");

const { Connection, Keypair, PublicKey, Transaction } = require("@solana/web3.js");


// import { aaa } from "./dist/index.js";


const connection = new Connection(
    "https://api.mainnet-beta.solana.com/",
    "finalized"
);

async function main(wallet) {
    //const wallet = new Keypair();
    console.log(Swap);
    const swap = new Swap(connection);

    // wrap SOL token if Sol is first token pair
    const ixs = await swap.createSwapIx(
        new PublicKey("So11111111111111111111111111111111111111112"),
        new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        //@ts-ignore
        new BigInt(100000), // 0.0001 SOL
        //@ts-ignore
        new BigInt(100), // 0.0001 USDC
        wallet.publicKey
    );

    let tx = new Transaction();
    for (const ix of ixs) {
        tx.add(ix);
    }

    //add unwrap sol token txn if sol is present as resulting token pair

    // Send out the tx use browser wallet or keypair
    return tx;
}

main();

// export default main;

