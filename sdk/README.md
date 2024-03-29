### GooseFX SSL Rust SDK

Provides high level APIs to facilitate the development of
client-side applications that interface with GooseFX `gfx-ssl` and `gfx-controller` programs.

There are functions that allow developers to easily:
- Create program instructions.
- Read blockchain state.

#### Jupiter Integration
There is a feature-gated (on by default) struct for Jupiter integration
that implements `Amm` trait from [this repo](https://github.com/jup-ag/rust-amm-implementation).
See their repo's docs for more details.

When using this crate with the `jupiter` feature,
developers must set their `LD_LIBRARY_PATH` (`DYLD_LIBRARY_PATH`) to this repo's `lib/<system>/<arch>` directory
or install the lib corresponding to their system and arch into system's search path (e.g. `/usr/local/lib`).
This allows binaries to link to a necessary dylib.
[See the Cargo guide](https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths) for more details on how Cargo looks for dynamic libraries.

For example, you can run the Jupiter example from the root of this repo on linux like so:
```commandline
LD_LIBRARY_PATH="./lib/linux/x86_64" cargo build --example jupiter -p gfx-ssl-sdk
```

### Testing
From inside this same directory where this Readme is located:
```
LD_LIBRARY_PATH="./lib/linux/x86_64" cargo test --test all_pairs -- --nocapture
```
