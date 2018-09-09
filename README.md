# cargo-cmd

Like `npm` scripts, but for `cargo`.

## Installation

```sh
cargo install cargo-cmd
```

## Usage

You can define your commands in `Cargo.toml` under the `[package.metadata.commands]` table, like so:

```toml
[package.metadata.commands]
greet = "echo 'Hello, planet!'"
```

Now you can run `cargo cmd greet`:

```sh
$ cargo cmd greet
> echo 'Hello, planet!'
Hello, planet!
```

## License
[MIT © Dan Reeves](./LICENSE)
