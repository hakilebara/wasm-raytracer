# wasm-raytracer
a raytracer written in Rust compiled to a WebAssembly Component

## Prerequisites

- [Rust](https://rustup.rs) with the `wasm32-wasip2` target: `rustup target add wasm32-wasip2`
- [Node.js](https://nodejs.org) 22+ (with `npm`)
- `make`

## Run locally

```
git clone https://github.com/hakilebara/wasm-raytracer
cd wasm-raytracer
cd web
npm ci
cp node_modules/@bytecodealliance/preview2-shim/dist/browser/{cli,clocks,common,config,environment,io}.js shim/
cd ..
make
cd dist
python3 -m http.server
```

## Development
In a separate terminal, I use [`entr`](https://github.com/eradman/entr) to automatically rebuild the wasm module.
```
ls web/*.{html,css,js}  src/*.rs | entr make

```
