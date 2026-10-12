.PHONY: all build clean

WASM_SRC := target/wasm32-wasip2/release/wasm_raytracer.wasm
WASM_DST := dist/wasm-raytracer.wasm
HTML_SRC := web/index.html
HTML_DST := dist/index.html
JS_SRC := web/main.js
JS_DST := dist/main.js
CSS_SRC := web/style.css
CSS_DST := dist/style.css
SHIM_SRC := web/shim
SHIM_DST := dist/shim
RAYTRACER_SRC := web/raytracer
RAYTRACER_DST := dist/raytracer

.PHONY: $(SHIM_DST) $(RAYTRACER_DST)

all: build jco $(WASM_DST) $(HTML_DST) $(JS_DST) $(CSS_DST) $(SHIM_DST) $(RAYTRACER_DST)

build:
	cargo build --target wasm32-wasip2 --release

jco:
	npx jco transpile target/wasm32-wasip2/release/wasm_raytracer.wasm -o web/raytracer

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

$(SHIM_DST): dist
	rm -rf $(SHIM_DST)
	cp -r $(SHIM_SRC) $(SHIM_DST)

$(RAYTRACER_DST): $(RAYTRACER_SRC)/wasm_raytracer.js $(RAYTRACER_SRC)/wasm_raytracer.core.wasm $(RAYTRACER_SRC)/wasm_raytracer.d.ts dist
	mkdir -p $(RAYTRACER_DST)
	cp $(RAYTRACER_SRC)/wasm_raytracer.js $(RAYTRACER_SRC)/wasm_raytracer.core.wasm $(RAYTRACER_SRC)/wasm_raytracer.d.ts $(RAYTRACER_DST)

clean:
	rm -rf dist
