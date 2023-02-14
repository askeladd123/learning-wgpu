# Learning wgpu
I just started this project. Not much to see.

You can try this application online by just clicking the link below.

---
## run web
Click this [link](https://askeladd123.github.io/learning-wgpu/).

---
## run native
You can simply do `cargo run --release`.
> tested on windows and linux

You will need the [rust toolkit](https://www.rust-lang.org/tools/install).

## build web
To build web ready files, you can use a build script *build-wasm*. 
 - run `git submodule update --init --recursive`, beacause *git submodules* is not cool
 - run `cargo build-web --release`. The new files will be in the new `dist` folder.
    - this is possible because of a cargo alias

### alternatives
You can build manually with:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/debug/learn_wgpu.wasm --target web --out-dir dist --no-typescript`

You will need:
- [rust toolkit](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target
- wasm-bindgen cli

To run the built web files I recommend these for hosting http server:
- [npm http-server](https://www.npmjs.com/package/http-server)
- [python http.server](https://docs.python.org/3/library/http.server.html)
- [vscode Live Server extension](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer)