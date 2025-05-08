# wasip2でのHTTP proxy world実装のサンプル

環境
```sh
$ rustc --version
rustc 1.84.1 (e71f9a9a9 2025-01-27)
$ wasmtime --version
wasmtime 32.0.0
```

実行
```sh
$ cargo build -r --target wasm32-wasip2
$ wasmtime serve -S cli target/wasm32-wasip2/release/wasip2_http_proxy_sample.wasm
```
