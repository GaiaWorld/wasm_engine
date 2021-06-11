# astc_decoder_web

## 利用 wasm-pack 打包

``` bat 

wasm-pack build --target no-modules

wasm-pack build --target no-modules -- --features wee_alloc

```

## emcc工具 将 wasm 变成：asm.js

``` bat

.\wasm2js --emscripten pkg/res_mgr_bg.wasm -o pkg/res_mgr_bg1.js

```