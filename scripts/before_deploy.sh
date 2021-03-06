# This script takes care of building the crate and packaging it for release.

PKG_NAME="cargo-cmd"

set -ex

main() {
  local src=$(pwd) \
    stage=

  case $TRAVIS_OS_NAME in
    linux)
      stage=$(mktemp -d)
      ;;
    windows)
      stage=$(mktemp -d)
      ;;
    osx)
      stage=$(mktemp -d -t tmp)
      ;;
  esac

  test -f Cargo.lock || cargo generate-lockfile

  local cargo=
  if [ $TRAVIS_OS_NAME = linux ]; then
    cargo='cross'
  else
    cargo='cargo'
  fi

  $cargo rustc --bin $PKG_NAME --target $TARGET --release -- -C lto
  cp target/$TARGET/release/$PKG_NAME $stage/

  cd $stage
  tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
  cd $src

  rm -rf $stage
}

main
