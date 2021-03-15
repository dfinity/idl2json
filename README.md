idl2json
========

Parse idl output from dfx.

```
dfx canister call governance get_proposal_info 1 | idl2json
```

There is sample output to play with without dfx:

```
<samples/proposal.idl ./target/debug/idl2json
```

# Installation on OSX

```
brew install libiconv
export LIBRARY_PATH=/usr/local/Cellar/libiconv/1.16/lib/
```

I guess this could be done via nix.
