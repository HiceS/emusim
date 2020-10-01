# emusim

### Intention

Build a simulation loop that can emulate motor movements given some form of input to be added to a simulation stack to better simulation robot articulations within a realtime environment.

### Building

#### Steps

1. clone the repo
2. wasm-pack if you don't have it (wasm-pack)[https://rustwasm.github.io/wasm-pack/installer/]
3. cargo build --lib
4. wasm-pack build