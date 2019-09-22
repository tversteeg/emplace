# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd)

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin emplace --target $TARGET --release -- -C lto

    cp target/$TARGET/release/emplace $src/
}

main
