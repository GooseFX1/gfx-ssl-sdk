### GooseFX SSL Rust SDK

Provides high level APIs to facilitate the development of
client-side applications that interface with GooseFX `gfx-ssl` and `gfx-controller` programs.

There are functions that allow developers to easily:
- Create program instructions.
- Read blockchain state.

#### Jupiter Integration
There is a feature-gated (on by default) struct for Jupiter integration
that implements `Amm` trait from [this repo](https://github.com/jup-ag/rust-amm-implementation).
See the crate docs for more details.

When using this crate with the `jupiter` feature,
developers must set their `LD_LIBRARY_PATH` to this repo's `lib` directory
or a folder containing the same contents.
This allows binaries to link to a necessary dylib.

For example, you can run the Jupiter example from the root of this repo like so:
```commandline
LD_LIBRARY_PATH="./lib" cargo build --example jupiter -p gfx-ssl-sdk
```

### Testing
From inside this same directory where this Readme is located:
```
LD_LIBRARY_PATH="../lib" cargo test --test all_pairs
```
