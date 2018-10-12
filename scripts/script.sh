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

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
  main
fi
