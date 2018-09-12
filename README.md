# cargo-cmd

[![crates.io version][1]][2]
[![build status][3]][4]
[![downloads][5]][6]
[![docs.rs docs][7]][8]
[![license][9]][10]

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



[1]: https://img.shields.io/crates/v/cargo-cmd.svg?style=flat-square
[2]: https://crates.io/crates/cargo-cmd
[3]: https://img.shields.io/travis/danreeves/cargo-cmd.svg?style=flat-square
[4]: https://travis-ci.org/danreeves/cargo-cmd
[5]: https://img.shields.io/crates/d/cargo-cmd.svg?style=flat-square
[6]: https://crates.io/crates/cargo-cmd
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/cargo-cmd
[9]: https://img.shields.io/crates/l/cargo-cmd.svg?style=flat-square
[10]: ./LICENSE

