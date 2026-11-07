#!/usr/bin/env sh
set -eu

course_dir=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)

if [ -f index.rs ] && [ -f test.rs ]; then
    module=$(basename "$(dirname "$PWD")")
    lesson=$(basename "$PWD")
    exec cargo test --manifest-path "$course_dir/Cargo.toml" --bin "$module-$lesson"
fi

exec cargo test --manifest-path "$course_dir/Cargo.toml" --all-targets
