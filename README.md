# jsonnet-go-sys

This crate provides rust bindings for the C API of the [go-jsonnet] library.

[go-jsonnet]: https://github.com/google/go-jsonnet

# Building
You will need to have go 1.12+ installed in order to build go-jsonnet. This can
be gotten via either,
- your package manager (e.g. apt install golang-go), or,
- the official golang.org install page: <https://go.dev/doc/install>.

# Example
```rust
use std::ffi::CStr;
use jsonnet_go_sys::*;

#[derive(Debug, serde::Deserialize)]
struct Basic {
    field: String,
}

let filename = c"basic.jsonnet";
let jsonnet = c"{ field: std.base64('abcd') }";

let vm = unsafe { jsonnet_make() };

let mut error = 0;
let messageptr = unsafe {
    jsonnet_evaluate_snippet(
        vm,
        filename.as_ptr() as _,
        jsonnet.as_ptr() as _,
        &mut error
    )
};

let message = unsafe { CStr::from_ptr(messageptr) };
let message = message.to_str().unwrap();

if error != 0 {
    panic!("jsonnet evaluation returned an error: {message}");
}

let json: Basic = serde_json::from_str(message).unwrap();
assert_eq!(json.field, "YWJjZA==");

unsafe { jsonnet_realloc(vm, messageptr, 0) };
unsafe { jsonnet_destroy(vm) };
```