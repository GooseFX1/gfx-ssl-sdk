#!/bin/sh

# Clones both the GFX Controller and GFX SSL programs from mainnet,
# so that they can be used in localnet.

# To test new and undeployed version of these programs, you can copy the .so
# build artifacts from the target/deploy folder of the repo that contains
# the actual program code.

solana -um -k ~/.config/solana/id.json program dump 8KJx48PYGHVC9fxzRRtYp4x4CM2HyYCm2EjVuAP4vvrx dump/controller.so
solana -um -k ~/.config/solana/id.json program dump 7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5 dump/ssl.so
