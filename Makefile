
all: build

install-dependencies:
	cargo install --root .cargo wasm-server-runner
	cargo install --root .cargo wasm-bindgen-cli

labyrinth-build-wasm32:
	cd labyrinth && make build-wasm32

build: labyrinth-build-wasm32
	mkdir -p pages
	cp -r web/* pages/
	cp -r labyrinth/assets pages/
	.cargo/bin/wasm-bindgen --out-dir ./pages/wasm32/ --target web ./labyrinth/target/wasm32-unknown-unknown/release/labyrinth-game.wasm

clean:
	rm -rf pages

web-dev:
	python3 -m http.server --directory pages
