set "cfgPath=../temp/cfg.txt"
call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM 微信小游戏不支持-reference-types,-sign-ext优化，  参数参考https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features.html
set RUST_LOG=info
wasm-pack build --profiling  --target nodejs --out-dir pkg_profiling_node --out-name wasm_engine
pause;


