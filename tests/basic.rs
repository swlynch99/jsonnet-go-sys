use std::ffi::CStr;

use jsonnet_go_sys::*;

const JSONNET: &str = "{ field: std.base64('abcd') }\0";

#[derive(Debug, serde::Deserialize)]
struct Basic {
    field: String,
}

#[test]
fn smoketest() {
    let filename = CStr::from_bytes_with_nul(b"basic.jsonnet\0").unwrap();
    let jsonnet = CStr::from_bytes_with_nul(JSONNET.as_bytes()).unwrap();

    let vm = unsafe { jsonnet_make() };

    let mut error = 0;
    let messageptr = unsafe {
        jsonnet_evaluate_snippet(
            vm,
            filename.as_ptr() as _,
            jsonnet.as_ptr() as _,
            &mut error,
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
}
