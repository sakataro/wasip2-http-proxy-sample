# wasip2でのHTTP proxy world実装のサンプル

環境
```sh
$ rustc --version
rustc 1.84.1 (e71f9a9a9 2025-01-27)
$ cargo --version
cargo 1.84.1 (66221abde 2024-11-19)
$ wkg --version
wkg 0.10.0
$ wasmtime --version
wasmtime 32.0.0
```

実行
```sh
$ cargo build -r --target wasm32-wasip2
$ wasmtime serve -S cli target/wasm32-wasip2/release/wasip2_http_proxy_sample.wasm
```
## 実装までにやったこと

```sh
$ mkdir wit
$ cargo add wit-bindgen
$ cargo add wit-bindgen-rt
$ wkg get wasi:http@0.2.0 -o wit/
$ wkg wit fetch #wasi:httpが依存してるwitダウンロード
```
