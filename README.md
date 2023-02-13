# Learning wgpu
I just started this project. Not much to see.

You can try this application online by just clicking the link below.

---
## run web
Click this [link](https://askeladd123.github.io/learning-wgpu/).

## run native
maybe I will provide binaries one day, who knows...

---
## build native
You can simply do `cargo run --release`.
> tested on windows and linux

You will need the [rust toolkit](https://www.rust-lang.org/tools/install).

## build web
To build web ready files run `cargo build-web --release`. The new files will be in the new `dist` folder.
> this is possible because of an alias inside `.cargo` that secretly runs another crate `build-web`

### alternatives
You can build manually with:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/debug/pathfinding_wgpu_web.wasm --target web --out-dir dist --no-typescript`

You will need:
- [rust toolkit](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target
- wasm-bindgen cli

I use npm's http-server to test it. Python http server had problems with wasm / MIME-type.
