# Pathfinding

## run web

---

## build windows


## build linux


## build other

## build web
To build web ready files run `build.py`. You will need:
- [python](https://www.python.org/downloads/) for `build.py`

### alternatives
You can build manually with:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen --target web --out-dir dist --no-typescript target/wasm32-unknown-unknown/debug/pathfinding_wgpu.wasm`
- copy `index.html` to the new `dist` folder

You will need:
- [rust toolkit](https://www.rust-lang.org/tools/install)
- wasm32-unknown-unknown target
- wasm-bindgen cli
