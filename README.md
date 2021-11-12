idl2json
========

Reads [Candid](https://github.com/dfinity/candid) textual values from `stdin` and emits them as json values on `stdout`.

This command is useful for pipeing the output of Candid value producing tools (such as dfx) into json tools.

```
dfx canister call governance get_proposal_info 1 | idl2json
```

There is sample output to play with without dfx:

```
<samples/proposal.idl ./target/debug/idl2json
```

# Build

Build with any of the following methods.  The binary executable will be at `idl2json/target/{debug,release}/idl2json` (depending on whether `cargo build --release` is used).

## With nix:
```
nix-shell --command 'cargo build'
```

## Build OSX without nix

```
brew install libiconv
export LIBRARY_PATH=/usr/local/Cellar/libiconv/1.16/lib/
cargo build
```
