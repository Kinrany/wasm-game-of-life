# wasm-game-of-life

Reading [Rust 🦀 and WebAssembly 🕸][book]

Status: finished section 5/10 "Testing Life"

## How to

### Build

1. `wasm-pack build` to build the package
2. `cd pkg && npm link` to add the package to the global `node_modules`

### Test

1. `wasm-pack test --chrome --headless`
1. `wasm-pack test --firefox --headless`

### See the output

1. `cd wasm-app`
2. `npm link wasm-game-of-life` to add a link to the `wasm-game-of-life` package
3. `npm install`
4. `npm run start` and open `localhost:8080` to see the output

## Reference

1. The book: https://rustwasm.github.io/docs/book/introduction.html
2. `wasm-pack-template`: https://github.com/rustwasm/wasm-pack-template
3. `create-wasm-app`: https://github.com/rustwasm/create-wasm-app

[book]: https://rustwasm.github.io/docs/book/introduction.html
