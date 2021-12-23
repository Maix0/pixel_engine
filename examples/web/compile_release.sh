mkdir -p ../../target/wasm-examples/web 
wasm-bindgen --target web --out-dir ../../target/wasm-examples/web ../../target/wasm32-unknown-unknown/release/web.wasm
cp src/index.html ../../target/wasm-examples/web/