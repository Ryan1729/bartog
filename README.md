# Bartog

This is a single player only implementation of Bartog, A.K.A. Bartok, A.K.A. Warthog.

See [the included rules folder](./design/rules/README.md) for more info on the game.

Because of the extremely flexible nature of Bartog, in some sense this project may never be truly complete. It is presented here in it's current form, and I make no promises about where it will eventually get.

### Building (using Rust's native WebAssembly backend)

1. Install newest nightly Rust:

       $ curl https://sh.rustup.rs -sSf | sh

2. Install WebAssembly target:

       $ rustup target add wasm32-unknown-unknown

3. Install [cargo-web]:

       $ cargo install -f cargo-web

4. Build it:

       $ cargo web start --target=wasm32-unknown-unknown --release

5. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

### Building for other backends

Replace `--target=wasm32-unknown-unknown` with `--target=wasm32-unknown-emscripten` or `--target=asmjs-unknown-emscripten`
if you want to build it using another backend. You will also have to install the
corresponding targets with `rustup` - `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten`
respectively.

### Extra build options

Extra invariant checking can be activated with by building with the following command

```
       $ cargo web start --target=wasm32-unknown-unknown --release --features="invariant-checking"
```

With this enabled violations of certain invariants will result in a panic. These checks are disabled in default mode since (presumably) a player would prefer the game doing something weird to outright crashing.
___

licensed under Apache or MIT, at your option.
