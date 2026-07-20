.PHONY: all build clean

WASM_SRC := target/wasm32-unknown-unknown/release/wasm_raytracer.wasm
WASM_DST := dist/wasm-raytracer.wasm
HTML_SRC := index.html
HTML_DST := dist/index.html

all: build $(WASM_DST) $(HTML_DST)

build:
	cargo build --target wasm32-unknown-unknown --release

dist:
	mkdir -p dist

$(WASM_DST): build dist
	cp $(WASM_SRC) $(WASM_DST)

$(HTML_DST): $(HTML_SRC) dist
	cp $(HTML_SRC) $(HTML_DST)

clean:
	rm -rf dist
