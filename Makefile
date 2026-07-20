.PHONY: all build clean

WASM_SRC := target/wasm32-unknown-unknown/release/wasm_raytracer.wasm
WASM_DST := dist/wasm-raytracer.wasm
HTML_SRC := web/index.html
HTML_DST := dist/index.html
JS_SRC := web/main.js
JS_DST := dist/main.js
CSS_SRC := web/style.css
CSS_DST := dist/style.css

all: build $(WASM_DST) $(HTML_DST) $(JS_DST) $(CSS_DST)

build:
	cargo build --target wasm32-unknown-unknown --release

dist:
	mkdir -p dist

$(WASM_DST): build dist
	cp $(WASM_SRC) $(WASM_DST)

$(HTML_DST): $(HTML_SRC) dist
	cp $(HTML_SRC) $(HTML_DST)

$(JS_DST): $(JS_SRC) dist
	cp $(JS_SRC) $(JS_DST)

$(CSS_DST): $(CSS_SRC) dist
	cp $(CSS_SRC) $(CSS_DST)

clean:
	rm -rf dist
