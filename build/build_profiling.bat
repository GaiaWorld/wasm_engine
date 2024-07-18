cd ../
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name wasm_engine
"C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-df7a1317af78a1c0\\wasm-bindgen.exe" "D:\\work\wasm_engine\\target\\wasm32-unknown-unknown\\release\\wasm_engine.wasm" "--out-dir" "D:\\work\wasm_engine\\pkg_profiling" "--typescript" "--target" "web" "--out-name" "wasm_engine"
node build/build_wasm.js pkg_profiling wasm_engine
pause;
