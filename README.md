# cargo-jvm-backend
Compile Rust to JVM classes

## Possible intermediates
- https://github.com/cretz/asmble
- https://github.com/davidar/lljvm
- https://github.com/bytecodealliance/wasmtime/blob/master/crates/lightbeam/src/microwasm.rs + https://asm.ow2.io/
- https://github.com/MaulingMonkey/jni-bindgen/

## Sketch of implementation plan
- hook cargo to get the metadata for current crate (look to wasm-pack et al for inspiration)
- parse metadata for rust <-> jvm imports/exports (via one of {macro, trait, toml file})
- possibly use jni-bindgen for importing class/function signatures, and specify which {.class,.jar} files to look at via the {macro,trait,toml}
- use one of {asmble,lljvm} to get bytecode for the crate, generating glue for the imports
- generate class files for the bytecode based on the export metadata

## manually building the class with asmble
Assumes that `asmble` is on PATH.

```bash
cd examples/hello-world-java
cargo build --release --target=wasm32-unknown-unknown
asmble compile target/wasm32-unknown-unknown/release/hello-world-java.wasm Hello
javap Hello
```
