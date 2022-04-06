import { NATIVE_MINT } from "@solana/spl-token";
import { Connection, PublicKey } from "@solana/web3.js";
import { ADDRESSES, SSL, SSL_LAYOUT, Swap } from "../src"; // Change "../src" to "goosefx-ssl-sdk" for using the NPM package

const connection = new Connection(
    "https://api.mainnet-beta.solana.com/",
    "finalized"
);

const getInfo = async () => {
    const ssl = (await SSL.loadByMint(connection, ADDRESSES["MAINNET"].GFX_CONTROLLER, NATIVE_MINT))!;
    const mainVault = (await SSL.liabilityVault(connection, ADDRESSES["MAINNET"].GFX_CONTROLLER, NATIVE_MINT))!;
    const swappedVault = (await SSL.liabilityVault(connection, ADDRESSES["MAINNET"].GFX_CONTROLLER, NATIVE_MINT, new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")))!;

    console.log(`SSL ${NATIVE_MINT} has ${mainVault.amount} SOL deposited plus some SOL swapped into ${swappedVault.amount} USDC`);
    console.log(`SSL ${NATIVE_MINT} has ${ssl.swappedLiabilityNative} equivalent SOL swapped`);
    console.log(`SSL ${NATIVE_MINT} has ${ssl.totalShare} share`);
};

getInfo();
