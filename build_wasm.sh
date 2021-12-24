
mkdir -p wasm-serve/wasm
cp index.html wasm-serve/


if [ "$1" == "release" ]; then
    echo "Building in release mode"
    cargo build --release --lib --target wasm32-unknown-unknown
    for f in ./target/wasm32-unknown-unknown/release/*.wasm; do
        wasm-bindgen --target web --out-dir "wasm-serve/wasm" $f
    done
else 
    echo "Building in debug mode"
    cargo build --lib --target wasm32-unknown-unknown
    
    for f in ./target/wasm32-unknown-unknown/debug/*.wasm; do
        wasm-bindgen --target web --out-dir "wasm-serve/wasm" $f
    done

    
fi
