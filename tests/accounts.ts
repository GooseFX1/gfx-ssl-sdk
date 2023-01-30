import * as anchor from "@project-serum/anchor";

import * as localnetWalletJson from "./localnet_wallet.json";
export const localnetWallet = new anchor.web3.PublicKey(localnetWalletJson.pubkey);
import * as mintJson from "./mint.json";
export const mint = new anchor.web3.PublicKey(mintJson.pubkey);
import * as testUserTokenActJson from "./test_user_token_act.json";
export const testUserTokenAct = new anchor.web3.PublicKey(testUserTokenActJson.pubkey);