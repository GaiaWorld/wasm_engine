set "cfgPath=../temp/cfg.txt"
call cfg.bat
cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=-Zlocation-detail=none
@REM 微信小游戏不支持-reference-types,-sign-ext优化，  参数参考https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features.html
set RUST_LOG=info
wasm-pack build --release  --target web --out-dir pkg --out-name wasm_engine
node build/build_wasm.js pkg wasm_engine
pause;