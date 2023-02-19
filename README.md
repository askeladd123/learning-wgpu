# Learning wgpu
I just started this project. Not much to see.

You can try this application online by just clicking the link below.

---
## run web
Click this [link](https://askeladd123.github.io/learning-wgpu/).

---
## run native
You can simply do `cargo run`.
You will need the [rust toolkit](https://www.rust-lang.org/tools/install).
> **Warning**: crashes on some windows devices because of a *wgpu v0.15.1* bug

## build web
To build web ready files, you can run `cargo build-web`. 

The built files will be in the `dist` folder, ready to be used. This command is possible because of a alias in *.cargo/config.toml*. 

### alternatives
You can build manually with:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/debug/learn_wgpu.wasm --target web --out-dir dist --no-typescript`

You will need:
- [rust toolkit](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target
- wasm-bindgen cli

### use built files
To run the built web files I recommend these for hosting http server:
- [npm http-server](https://www.npmjs.com/package/http-server)
- [python http.server](https://docs.python.org/3/library/http.server.html)
- [vscode Live Server extension](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer)
