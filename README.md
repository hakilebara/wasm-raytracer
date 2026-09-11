# wasm-raytracer
a raytracer written in Rust compiled to a WebAssembly Component

## Run locally

```
git https://github.com/hakilebara/wasm-raytracer
cd wasm-raytracer
make
cd dist
python3 -m http.server
```

## Development
In a separate terminal, I use [`entr`](https://github.com/eradman/entr) to automatically rebuild the wasm module.
```
ls web/*.{html,css,js}  src/*.rs | entr make

```
