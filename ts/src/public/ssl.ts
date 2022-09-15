import { BN, Program } from "@project-serum/anchor";
import { getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { AccountInfo, Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ADDRESSES, Network } from "../constants";
import { SSL_LAYOUT, SSLLayout as SSLLayout } from "../layouts";
import { findAssociatedTokenAddress } from "./utils";

export class SSL {
    inner: SSLLayout;
    public address: PublicKey;

    static async loadByMint(
        connection: Connection,
        controller: PublicKey,
        mint: PublicKey,
        programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID
    ) {
        return await SSL.loadByAddress(connection, SSL.findAddress(controller, mint, programId));
    }

    static async loadByAddress(
        connection: Connection,
        address: PublicKey,
        programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID
    ) {
        let ai = (await connection.getAccountInfo(address));
        if (ai === undefined) {
            return undefined;
        }

        let ssl = new SSL(ai!, programId);
        return ssl;
    }

    static async liabilityVault(connection: Connection, controller: PublicKey, mint: PublicKey, otherMint?: PublicKey, programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID) {
        if (otherMint === undefined) {
            otherMint = mint;
        }

        let address = findAssociatedTokenAddress(SSL.findAddress(controller, mint, programId), otherMint);
        let account = await getAccount(connection, address);
        return account;
    }

    static findAddress(controller: PublicKey, mint: PublicKey, programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID) {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("GFX-SSL", "utf-8"),
                controller.toBuffer(),
                mint.toBuffer(),
            ],
            programId
        )[0];
    };

    constructor(ai: AccountInfo<Buffer>, programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID) {
        this.inner = SSL_LAYOUT.decode(ai.data);
        this.address = SSL.findAddress(this.controller, this.mint, programId);
    }

    public isSuspended(): boolean {
        return this.inner.suspended;
    }

    async deposit(
        program: Program,
        controller: PublicKey,
        wallet: PublicKey,
        amount: BigInt,
        programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID
    ): Promise<TransactionInstruction> {
        const inst = program.instruction;

        const accounts = {
            controller,
            ssl: this.address,
            liquidityAccount: SSL.getLiquidityAccountAddress(controller, this.mint, wallet, programId),
            rtVault: findAssociatedTokenAddress(this.address, this.mint),
            userRtAta: findAssociatedTokenAddress(wallet, this.mint),
            userWallet: wallet,
            tokenProgram: TOKEN_PROGRAM_ID,
        };

        return await inst.deposit(
            new BN(amount.toString()),
            { accounts }
        );
    }

    public static getLiquidityAccountAddress(controller: PublicKey, mint: PublicKey, wallet: PublicKey, programId: PublicKey): PublicKey {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("GFX-LIQUIDITYACCOUNT", "utf-8"),
                controller.toBuffer(),
                mint.toBuffer(),
                wallet.toBuffer()
            ],
            programId
        )[0];
    }

    public get controller() {
        return this.inner.controller;
    }

    public get mint() {
        return this.inner.mint;
    }

    public get decimals() {
        return this.inner.decimals;
    }

    public poolTokenMint(programId: PublicKey = ADDRESSES["MAINNET"].SSL_PROGRAM_ID) {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from("GFX-SSL-PTMINT", "utf-8"),
                this.controller.toBuffer(),
                this.mint.toBuffer(),
            ],
            programId
        )[0];
    }

    public get suspended() {
        return this.inner.suspended;
    }

    public get weight() {
        return this.inner.weight;
    }

    public get swappedLiabilityNative() {
        return this.inner.swappedLiabilityNative;
    }

    public get totalShare() {
        return this.inner.totalShare;
    }
}