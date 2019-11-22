# Wasmuter
This library aims to facilitate WebAssembly code generation, i.e., transforming a compiler syntax tree representation into executable WebAssembly bytecode. It provides light abstractions over the literal bytecode format to make it more straighforward to generate, while still allowing full control over the emitted WebAssembly bytecode.

### Priorities
- [x] Add support for all instructions in specification
- [x] Add support for all module sections in specification
- [ ] Add helpers to define multiple related module sections as one conceptual unit
- [ ] Add structured expression abstraction over raw instruction `Vec`s

### Future
- [ ] Add support for custom module sections
- [ ] Write an example language compiler using this library
- [ ] Consider the utility of a WebAssembly bytecode parser
- [ ] Look into how optimization passes might work
