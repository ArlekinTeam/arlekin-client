# Arlekin client

## Dependencies

- Rust 1.81.0

## Development

### Installation

To compile and run the browser version you need to have `Rust` tools installed (https://www.rust-lang.org/tools/install).

Then you need to install WebAssembly using the following command:

```
rustup target add wasm32-unknown-unknown
```

And then, you need to install Trunk using the following command:

On Linux and MacOS:

```
cargo install --locked trunk
```

On Windows:

```
cargo install --locked --no-default-features --features update_check,rustls trunk
```

### Compiling and running

Next in the downloaded repository, go to the `crates/client` directory and run the following command:

```
trunk serve
```

Done! Once everything compiles, you should see the Arlekin client at: http://localhost:8080/.
