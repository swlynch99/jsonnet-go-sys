#!/bin/bash

set -euo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR"

mkdir -p target/bindings

cd go-jsonnet
go build -buildmode=c-archive -o ../target/bindings/libjsonnet.a c-bindings/*.go
cd ..

CLANG_ARGS=(
    -Igo-jsonnet/c-bindings
)

BINDGEN_ARGS=(
    --sort-semantically
    --allowlist-type 'Jsonnet.*'
    --allowlist-item 'jsonnet_.*'
)

clang -E target/bindings/libjsonnet.h   \
    -o target/bindings/libjsonnet.i     \
    "${CLANG_ARGS[@]}"

bindgen target/bindings/libjsonnet.h    \
    -o target/bindings/bindings.rs      \
    "${BINDGEN_ARGS[@]}"                \
    --                                  \
    "${CLANG_ARGS[@]}"

cp target/bindings/bindings.rs src/bindings.rs

