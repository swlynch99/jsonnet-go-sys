//! Bindings for the C interface of [go-jsonnet].
//!
//! In order to use this library you will first want to create a [`JsonnetVm`]
//! instance via [`jsonnet_make`].
//!
//! See the [C API documentation on jsonnet.org][c-api] for more details.
//!
//! [go-jsonnet]: https://github.com/google/go-jsonnet
//! [c-api]: https://jsonnet.org/ref/bindings.html#c_api
//!
//! # Example
//! ```
//! use std::ffi::CStr;
//! use jsonnet_go_sys::*;
//!
//! #[derive(Debug, serde::Deserialize)]
//! struct Basic {
//!     field: String,
//! }
//!
//! let filename = c"basic.jsonnet";
//! let jsonnet = c"{ field: std.base64('abcd') }";
//!
//! let vm = unsafe { jsonnet_make() };
//!
//! let mut error = 0;
//! let messageptr = unsafe {
//!     jsonnet_evaluate_snippet(
//!         vm,
//!         filename.as_ptr() as _,
//!         jsonnet.as_ptr() as _,
//!         &mut error
//!     )
//! };
//!
//! let message = unsafe { CStr::from_ptr(messageptr) };
//! let message = message.to_str().unwrap();
//!
//! if error != 0 {
//!     panic!("jsonnet evaluation returned an error: {message}");
//! }
//!
//! let json: Basic = serde_json::from_str(message).unwrap();
//! assert_eq!(json.field, "YWJjZA==");
//!
//! unsafe { jsonnet_realloc(vm, messageptr, 0) };
//! unsafe { jsonnet_destroy(vm) };
//! ```

#[allow(non_snake_case, rustdoc::bare_urls)]
mod bindings;

pub use crate::bindings::*;

// Check the README to validate that all the code within compiles and runs.
#[cfg(doc)]
#[doc = include_str!("../README.md")]
mod readme {}
