# cargo-cmd

[![crates.io version][1]][2]
[![build status][3]][4]
[![docs.rs docs][5]][6]
[![license][7]][8]

Alias any shell command in your `Cargo.toml`. It's like `npm` scripts, but for `cargo`.

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

### Advanced use

#### Passing arguments

It's possible to pass arguments into your command by passing them directly to `cargo cmd`.

```toml
[package.metadata.commands]
echo = "echo"
```

```sh
$ cargo cmd echo 'Hello, planet!'
> echo 'Hello, planet!'
Hello, planet!
```

#### Pre and Post commands

You are able to set up commands to run before and after your command by prefixing the name with `pre` or `post` respectively.

```toml
[package.metadata.commands]
pretest = "./setup.sh"
test = "cargo test"
posttest = "./teardown.sh"
```

```sh
$ cargo cmd test

[pretest]
> ./setup.sh
Setting up DB...

[test]
> cargo test
...

[posttest]
> ./teardown.sh
Tearing down DB...
```

## License
[MIT © Dan Reeves](./LICENSE)



[1]: https://img.shields.io/crates/v/cargo-cmd.svg?style=flat-square
[2]: https://crates.io/crates/cargo-cmd
[3]: https://img.shields.io/travis/danreeves/cargo-cmd.svg?style=flat-square
[4]: https://travis-ci.org/danreeves/cargo-cmd
[5]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[6]: https://docs.rs/crate/cargo-cmd
[7]: https://img.shields.io/crates/l/cargo-cmd.svg?style=flat-square
[8]: ./LICENSE

