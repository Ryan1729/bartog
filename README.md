# Bartog

This is a single player only implementation of Bartog, A.K.A. Bartok, A.K.A. Warthog.

See [the included rules folder](./design/rules/README.md) for more info on the game, but essentially it's Crazy Eights but if you win a round, you get to pick a new rule that applies to future rounds.

Because of the extremely flexible nature of Bartog, in some sense this project may never be truly complete. It is presented here in its current form, and I make no promises about where it will eventually get.

You can play the live version of the latest release build [here](https://ryan1729.github.io/bartog/bartog/).

I have also made a version of "plain" Crazy Eights, the live version of which can be played [here](https://ryan1729.github.io/bartog/crazy-eights/).

### Building (using Rust's native WebAssembly backend)

1. Install Rust via [rustup.rs](https://rustup.rs).

2. Install WebAssembly target:

       $ rustup target add wasm32-unknown-unknown

3. Install [cargo-web]:

       $ cargo install -f cargo-web

4. Run it:

       $ ./run-stdweb
    Note: As of this writing, a nightly version from 2019 is required. This should be automatically downloaded because of the included `rust-toolchain.toml` file. See [here](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file) for more details on that file.
    

5. Visit `http://localhost:8000` with your browser.

[cargo-web]: https://github.com/koute/cargo-web

### Building for other backends

Replace `--target=wasm32-unknown-unknown` with `--target=wasm32-unknown-emscripten` or `--target=asmjs-unknown-emscripten`
if you want to build it using another backend. You will also have to install the
corresponding targets with `rustup` - `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten`
respectively.

### Extra build options

These extra features can be adding then to the cargo `features` flag. For instance to activate `invariant-checking` and `logging` you can run:
```
       $ cargo web start --target=wasm32-unknown-unknown --release --features="invariant-checking logging"
```

##### invariant-checking

With this enabled violations of certain invariants will result in a panic. These checks are disabled in default mode since (presumably) a player would prefer the game doing something weird to outright crashing.

##### logging

Enables additional generic logging. With this feature disabled, the logs will be compiled out, leaving no appreciable run-time overhead.
___

licensed under Apache or MIT, at your option.
