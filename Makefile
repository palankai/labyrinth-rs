install-dependencies:
	cargo install --root .cargo wasm-server-runner
	cargo install --root .cargo wasm-bindgen-cli

build-wasm:
	cargo build --release --target wasm32-unknown-unknown

build-pages: build-wasm
	mkdir -p pages
	cp -r web/* pages/
	cp -r assets pages/
	.cargo/bin/wasm-bindgen --out-dir ./pages/wasm32/ --target web ./target/wasm32-unknown-unknown/release/labyrinth-game.wasm

clean-pages:
	rm -rf pages
