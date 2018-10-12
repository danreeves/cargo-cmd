# This script takes care of testing the crate.

set -ex

main() {
  cargo build --target $TARGET
  cargo build --target $TARGET --release

  if [ ! -z $DISABLE_TESTS ]; then
    return
  fi

  cargo fmt -- --check
  cargo +nightly clippy

  cargo test --target $TARGET
  cargo test --target $TARGET --release
}

if [ $TRAVIS_OS_NAME = linux ]; then
  alias cargo=cross
else
  alias cargo=cargo
fi

cargo --version

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
  main
fi
