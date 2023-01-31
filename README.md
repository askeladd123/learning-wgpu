# Pathfinding
I just started this project. Not much to see.

You can try this application online by just clicking the link below.

---
## run web
Click this [link](https://askeladd123.github.io/pathfinding-wgpu/).

## run native
maybe I will provide binaries one day, who knows...

---
## build native
You can simply do `cargo run --release`.
> tested on windows and linux

You will need the [rust toolkit](https://www.rust-lang.org/tools/install).

## build web
To build web ready files run `build_web.py`. You will need [python](https://www.python.org/downloads/) to run the script. The files will be in the new `dist` folder.

### alternatives
You can build manually with:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen --target web --out-dir dist --no-typescript target/wasm32-unknown-unknown/debug/pathfinding_wgpu_web.wasm`

You will need:
- [rust toolkit](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target
- wasm-bindgen cli

I use npm's http-server to test it. Python http server had problems with wasm / MIME-type.
