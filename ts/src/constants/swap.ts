import { PublicKey } from "@solana/web3.js";

export type Network = "MAINNET" | "DEVNET";

export const ADDRESSES = {
  MAINNET: {
    CONTROLLER_PROGRAM_ID: new PublicKey("8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx"),
    SSL_PROGRAM_ID: new PublicKey("7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5"),
    GFX_CONTROLLER: new PublicKey("8CxKnuJeoeQXFwiG6XiGY2akBjvJA5k3bE52BfnuEmNQ"),
  },
  DEVNET: {
    CONTROLLER_PROGRAM_ID: new PublicKey("3Gwyhoudx8XgYry8dzKQ2GGsofkUdm7VZUvddHxchL3x"),
    SSL_PROGRAM_ID: new PublicKey("JYe7AcuQ7CqhkGvchJGvSKF8ei41FuDKb1h47qkbFNf"),
    GFX_CONTROLLER: new PublicKey("483AtY5eistVBBcXr9Tq2XH6MTrxCWfFRingputiZC2B"),
  }
};

