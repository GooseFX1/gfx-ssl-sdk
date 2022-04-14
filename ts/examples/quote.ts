import { swap, OracleRegistry } from '../src/wasm/gfx_ssl_wasm';
import { Connection, PublicKey } from "@solana/web3.js";
import { PAIR_LAYOUT } from "../src/index";

const connection = new Connection("https://api.mainnet-beta.solana.com/", "finalized");

async function main() {
    let SOLSSLKey = new PublicKey("5i9k43oKJWwTcG84zv6B4Mmejf6R2mgY8dvTGXfBaaZq");
    let USDSSLKey = new PublicKey("AkCAPJHqYU1JaTqVDJvrkV3Qonx6NyZFAC3z2t24eQTU");
    let pairKey = new PublicKey("CpfpL9PXt88u3kPQ6fuD6WqQpQ8c5UEftxsop9rm1ATM");

    let SOLSSLData = (await connection.getAccountInfo(SOLSSLKey)).data;
    let USDSSLData = (await connection.getAccountInfo(USDSSLKey)).data;
    let pairData = (await connection.getAccountInfo(pairKey)).data;


    const decoded = PAIR_LAYOUT.decode(pairData);
    const { oracles, nOracle: nOracleN } = decoded;
    const nOracle = Number(nOracleN);

    let registry = new OracleRegistry();

    for (const oracle of oracles.slice(0, nOracle)) {
        const n = Number(oracle.n);

        for (const elem of oracle.elements.slice(0, n)) {
            registry.add_oracle(elem.address.toBuffer(), (await connection.getAccountInfo(elem.address)).data);
        }
    }

    let { out, price_impact } = swap(SOLSSLData, USDSSLData, pairData, registry, BigInt(1) * BigInt(10000000), BigInt(0));

    console.log(`out: ${out} ${price_impact}`);
}

main();