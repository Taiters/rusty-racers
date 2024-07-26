rustup toolchain install stable
cargo install wasm-pack
cd salesman && wasm-pack build --target web --release && cd ..
cd salesman-web && npm install && npm run build
