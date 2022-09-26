haskellHop

inspiration from tsoding (betterttv emote) and andriamanitra (haskelHop haskellHop) and belzile (rust wasm repo)

## Build for the Web
### Prerequisites

```sh
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

```sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/haskell_hop.wasm
npx serve .
```
