cd ../
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name wasm_engine
"C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-2b8061563077bfb8\\wasm-bindgen.exe" "D:\\work\\pi_show_wasm_bindgen\\wasm_engine\\target\\wasm32-unknown-unknown\\release\\wasm_engine.wasm" "--out-dir" "D:\\work\\pi_show_wasm_bindgen\\wasm_engine\\pkg_profiling" "--typescript" "--target" "web" "--out-name" "wasm_engine"
node build/build_wasm.js pkg_profiling wasm_engine
pause;
