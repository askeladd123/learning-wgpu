# Learning wgpu
This is a program that visualizes different searching algrithms. You can try it by clicking [here](https://askeladd123.github.io/learning-wgpu/).

I made this for learning graphics programming, and the [webgpu standard](https://www.w3.org/TR/webgpu/).

## build from source
The program runs both on the web, and as a native application. Clone the repo, and make sure you have the [rust toolkit](https://www.rust-lang.org/tools/install) installed.

### native
To build and run as a native application simply do `cargo run`.

## web
To build web ready files, you can run `cargo build-web`. 

The built files will be in the *dist* folder, ready to be used. This command is possible because of an alias in *.cargo/config.toml*. 

To run on the web see *use built files* below.

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
