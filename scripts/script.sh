# This script takes care of testing the crate.

set -ex

main() {
  cross build --target $TARGET
  cross build --target $TARGET --release

  if [ ! -z $DISABLE_TESTS ]; then
    return
  fi

  cargo fmt -- --check
  cargo +nightly clippy

  cross test --target $TARGET
  cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
  main
fi
