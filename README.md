# wasm-game-of-life

Reading [Rust 🦀 and WebAssembly 🕸][book]

Status: finished section 4/10 "Implementing Life"

## How to

1. `wasm-pack build` to build the package
2. `npm link` to add the package to the global `node_modules`
3. `cd wasm-app`
4. `npm link wasm-game-of-life`
5. `npm install`
6. `npm run start` and open `localhost:8080` to see the output

## Reference

1. The book: https://rustwasm.github.io/docs/book/introduction.html
2. `wasm-pack-template`: https://github.com/rustwasm/wasm-pack-template
3. `create-wasm-app`: https://github.com/rustwasm/create-wasm-app

[book]: https://rustwasm.github.io/docs/book/introduction.html
