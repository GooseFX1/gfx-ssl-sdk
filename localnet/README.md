## GFX Localnet

This is a CLI that spins up a localnet configured with preloaded
with the necessary mints, token accounts, GFX program accounts, and programs
required for local testing.

### Setup
From the root of the repo:
```commandline
solana-keygen new -o localnet_wallet.json
./dump_programs
cargo run -p gfx-localnet
```
This first generates a new keypair that will serve as a mock
program admin / user.
Then it dumps the GFX programs from mainnet.
Finally, it executes `gfx-localnet` with its default subcommand of `build`,
which builds all the necessary test files.

### Usage
From the root of the repo:
```commandline
cargo run -p gfx-localnet -- from-test-config --skip-project-programs tests/Test.toml
```

The above command is complicated, so some explanation is in order.

- The `--` tells cargo that subsequent args are to be parsed by `gfx-localnet`,
rather than cargo itself.
- `from-test-config` is a subcommand that spins up a localnet from a specific `Test.toml`,
of which there is only one, located at `tests/Test.toml`.
- `--skip-project-programs` is a flag that turns off the loading of any programs in this repo.
We need this flag because we're already loading the real SSL and Controller programs at the
same addresses as the programs in this repo which are "interface-only" versions 
of the SSL and Controller programs.