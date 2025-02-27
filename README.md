# 3DS VC RNG Lib

This is a library to predict the RNG in 3DS VC games.

Building:

- lib: `cd vc_rng_lib && wasm-pack build`
- lib with docker: `docker run --rm -it -w /app -v $(pwd)/vc_rng_lib:/app rust /bin/bash`, install wasm-pack with `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` [from here](https://rustwasm.github.io/wasm-pack/installer/), then `wasm-pack build`
- cli: `cargo run`
