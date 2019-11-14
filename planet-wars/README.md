## Visualizer

### First-time visualizer setup (`visualizer-wasm` directory)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building and running
- Run `wasm-pack build` to compile Rust to webassembly (this creates a npm package in the `pkg` directory)
- Run `npm run start` from the `visualizer` directory, and open `http://localhost:8080/` in your browser

To use the visualizer from the infinibattle website:
- Run `wasm-pack build`
- Run `cd ../../visualizer/ && npm run build && cp dist/* ../../website/src/Infinibattle.Website/wwwroot/lib/visualizer/`

## Runner

Using the runner with Cargo (or use `--help` to see available options):
```
cargo run -- --width 800 --height 600 --seed 42 "bot_1.dll" "path/bot_2.dll"

```

Most arguments are optional and allow shorthand:
```
cargo run -- -s 123 "bot_1.dll" "bot_2.dll"
```
