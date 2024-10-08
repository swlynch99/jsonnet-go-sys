/* automatically generated by rust-bindgen 0.70.1 */

#[doc = " Callback used to load imports.\n\n The returned char* should be allocated with jsonnet_realloc.  It will be cleaned up by\n libjsonnet when no-longer needed.\n\n \\param ctx User pointer, given in jsonnet_import_callback.\n \\param base The directory containing the code that did the import.\n \\param rel The path imported by the code.\n \\param found_here Set this byref param to path to the file, absolute or relative to the\n     process's CWD.  This is necessary so that imports from the content of the imported file can\n     be resolved correctly.  Allocate memory with jsonnet_realloc.  Only use when *success = 1.\n \\param success Set this byref param to 1 to indicate success and 0 for failure.\n \\param buf Set this byref param to the content of the imported file, or an error message.  Allocate memory with jsonnet_realloc.  Do not include a null terminator byte.\n \\param buflen Set this byref param to the length of the data returned in buf.\n \\returns 0 to indicate success and 1 for failure.  On success, the content is in *buf.  On failure, an error message is in *buf."]
pub type JsonnetImportCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        base: *const ::std::os::raw::c_char,
        rel: *const ::std::os::raw::c_char,
        found_here: *mut *mut ::std::os::raw::c_char,
        buf: *mut *mut ::std::os::raw::c_char,
        buflen: *mut usize,
    ) -> ::std::os::raw::c_int,
>;
#[doc = " Callback to provide native extensions to Jsonnet.\n\n The returned JsonnetJsonValue* should be allocated with jsonnet_realloc.  It will be cleaned up\n along with the objects rooted at argv by libjsonnet when no-longer needed.  Return a string upon\n failure, which will appear in Jsonnet as an error.  The argv pointer is an array whose size\n matches the array of parameters supplied when the native callback was originally registered.\n\n \\param ctx User pointer, given in jsonnet_native_callback.\n \\param argv Array of arguments from Jsonnet code.\n \\param success Set this byref param to 1 to indicate success and 0 for failure.\n \\returns The content of the imported file, or an error message."]
pub type JsonnetNativeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut ::std::os::raw::c_void,
        argv: *const *const JsonnetJsonValue,
        success: *mut ::std::os::raw::c_int,
    ) -> *mut JsonnetJsonValue,
>;
#[doc = " Jsonnet virtual machine context."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JsonnetVm {
    _unused: [u8; 0],
}
#[doc = " An opaque type which can only be utilized via the jsonnet_json_* family of functions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JsonnetJsonValue {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Return the version string of the Jsonnet interpreter.  Conforms to semantic versioning\n https://semver.org/ If this does not match LIB_JSONNET_VERSION then there is a mismatch between\n header and compiled library."]
    pub fn jsonnet_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Create a new Jsonnet virtual machine."]
    pub fn jsonnet_make() -> *mut JsonnetVm;
}
extern "C" {
    #[doc = " Set the maximum stack depth."]
    pub fn jsonnet_max_stack(vm: *mut JsonnetVm, v: ::std::os::raw::c_uint);
}
extern "C" {
    #[doc = " Set the number of objects required before a garbage collection cycle is allowed."]
    pub fn jsonnet_gc_min_objects(vm: *mut JsonnetVm, v: ::std::os::raw::c_uint);
}
extern "C" {
    #[doc = " Run the garbage collector after this amount of growth in the number of objects."]
    pub fn jsonnet_gc_growth_trigger(vm: *mut JsonnetVm, v: f64);
}
extern "C" {
    #[doc = " Expect a string as output and don't JSON encode it."]
    pub fn jsonnet_string_output(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " If the value is a string, return it as UTF8 otherwise return NULL."]
    pub fn jsonnet_json_extract_string(
        vm: *mut JsonnetVm,
        v: *const JsonnetJsonValue,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " If the value is a number, return 1 and store the number in out, otherwise return 0."]
    pub fn jsonnet_json_extract_number(
        vm: *mut JsonnetVm,
        v: *const JsonnetJsonValue,
        out: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return 0 if the value is false, 1 if it is true, and 2 if it is not a bool."]
    pub fn jsonnet_json_extract_bool(
        vm: *mut JsonnetVm,
        v: *const JsonnetJsonValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return 1 if the value is null, else 0."]
    pub fn jsonnet_json_extract_null(
        vm: *mut JsonnetVm,
        v: *const JsonnetJsonValue,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Convert the given UTF8 string to a JsonnetJsonValue."]
    pub fn jsonnet_json_make_string(
        vm: *mut JsonnetVm,
        v: *const ::std::os::raw::c_char,
    ) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Convert the given double to a JsonnetJsonValue."]
    pub fn jsonnet_json_make_number(vm: *mut JsonnetVm, v: f64) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Convert the given bool (1 or 0) to a JsonnetJsonValue."]
    pub fn jsonnet_json_make_bool(
        vm: *mut JsonnetVm,
        v: ::std::os::raw::c_int,
    ) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Make a JsonnetJsonValue representing null."]
    pub fn jsonnet_json_make_null(vm: *mut JsonnetVm) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Make a JsonnetJsonValue representing an array.\n\n Assign elements with jsonnet_json_array_append."]
    pub fn jsonnet_json_make_array(vm: *mut JsonnetVm) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Add v to the end of the array."]
    pub fn jsonnet_json_array_append(
        vm: *mut JsonnetVm,
        arr: *mut JsonnetJsonValue,
        v: *mut JsonnetJsonValue,
    );
}
extern "C" {
    #[doc = " Make a JsonnetJsonValue representing an object with the given number of fields.\n\n Every index of the array must have a unique value assigned with jsonnet_json_array_element."]
    pub fn jsonnet_json_make_object(vm: *mut JsonnetVm) -> *mut JsonnetJsonValue;
}
extern "C" {
    #[doc = " Add the field f to the object, bound to v.\n\n This replaces any previous binding of the field."]
    pub fn jsonnet_json_object_append(
        vm: *mut JsonnetVm,
        obj: *mut JsonnetJsonValue,
        f: *const ::std::os::raw::c_char,
        v: *mut JsonnetJsonValue,
    );
}
extern "C" {
    #[doc = " Clean up a JSON subtree.\n\n This is useful if you want to abort with an error mid-way through building a complex value."]
    pub fn jsonnet_json_destroy(vm: *mut JsonnetVm, v: *mut JsonnetJsonValue);
}
extern "C" {
    #[doc = " Allocate, resize, or free a buffer.  This will abort if the memory cannot be allocated.  It will\n only return NULL if sz was zero.\n\n \\param buf If NULL, allocate a new buffer.  If an previously allocated buffer, resize it.\n \\param sz The size of the buffer to return.  If zero, frees the buffer.\n \\returns The new buffer."]
    pub fn jsonnet_realloc(
        vm: *mut JsonnetVm,
        buf: *mut ::std::os::raw::c_char,
        sz: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Override the callback used to locate imports."]
    pub fn jsonnet_import_callback(
        vm: *mut JsonnetVm,
        cb: JsonnetImportCallback,
        ctx: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Register a native extension.\n\n This will appear in Jsonnet as a function type and can be accessed from std.nativeExt(\"foo\").\n\n DO NOT register native callbacks with side-effects!  Jsonnet is a lazy functional language and\n will call your function when you least expect it, more times than you expect, or not at all.\n\n \\param vm The vm.\n \\param name The name of the function as visible to Jsonnet code, e.g. \"foo\".\n \\param cb The PURE function that implements the behavior you want.\n \\param ctx User pointer, stash non-global state you need here.\n \\param params NULL-terminated array of the names of the params.  Must be valid identifiers."]
    pub fn jsonnet_native_callback(
        vm: *mut JsonnetVm,
        name: *const ::std::os::raw::c_char,
        cb: JsonnetNativeCallback,
        ctx: *mut ::std::os::raw::c_void,
        params: *const *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Bind a Jsonnet external var to the given string.\n\n Argument values are copied so memory should be managed by caller."]
    pub fn jsonnet_ext_var(
        vm: *mut JsonnetVm,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Bind a Jsonnet external var to the given code.\n\n Argument values are copied so memory should be managed by caller."]
    pub fn jsonnet_ext_code(
        vm: *mut JsonnetVm,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Bind a string top-level argument for a top-level parameter.\n\n Argument values are copied so memory should be managed by caller."]
    pub fn jsonnet_tla_var(
        vm: *mut JsonnetVm,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Bind a code top-level argument for a top-level parameter.\n\n Argument values are copied so memory should be managed by caller."]
    pub fn jsonnet_tla_code(
        vm: *mut JsonnetVm,
        key: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[doc = " Set the number of lines of stack trace to display (0 for all of them)."]
    pub fn jsonnet_max_trace(vm: *mut JsonnetVm, v: ::std::os::raw::c_uint);
}
extern "C" {
    #[doc = " Add to the default import callback's library search path.\n\n The search order is last to first, so more recently appended paths take precedence."]
    pub fn jsonnet_jpath_add(vm: *mut JsonnetVm, v: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Evaluate a file containing Jsonnet code, return a JSON string.\n\n The returned string should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param error Return by reference whether or not there was an error.\n \\returns Either JSON or the error message."]
    pub fn jsonnet_evaluate_file(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Evaluate a string containing Jsonnet code, return a JSON string.\n\n The returned string should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file (used in error messages).\n \\param snippet Jsonnet code to execute.\n \\param error Return by reference whether or not there was an error.\n \\returns Either JSON or the error message."]
    pub fn jsonnet_evaluate_snippet(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        snippet: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Evaluate a file containing Jsonnet code, return a number of named JSON files.\n\n The returned character buffer contains an even number of strings, the filename and JSON for each\n JSON file interleaved.  It should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param error Return by reference whether or not there was an error.\n \\returns Either the error, or a sequence of strings separated by \\0, terminated with \\0\\0."]
    pub fn jsonnet_evaluate_file_multi(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Evaluate a string containing Jsonnet code, return a number of named JSON files.\n\n The returned character buffer contains an even number of strings, the filename and JSON for each\n JSON file interleaved.  It should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param snippet Jsonnet code to execute.\n \\param error Return by reference whether or not there was an error.\n \\returns Either the error, or a sequence of strings separated by \\0, terminated with \\0\\0."]
    pub fn jsonnet_evaluate_snippet_multi(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        snippet: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Evaluate a file containing Jsonnet code, return a number of JSON files.\n\n The returned character buffer contains several strings.  It should be cleaned up with\n jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param error Return by reference whether or not there was an error.\n \\returns Either the error, or a sequence of strings separated by \\0, terminated with \\0\\0."]
    pub fn jsonnet_evaluate_file_stream(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Evaluate a string containing Jsonnet code, return a number of JSON files.\n\n The returned character buffer contains several strings.  It should be cleaned up with\n jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param snippet Jsonnet code to execute.\n \\param error Return by reference whether or not there was an error.\n \\returns Either the error, or a sequence of strings separated by \\0, terminated with \\0\\0."]
    pub fn jsonnet_evaluate_snippet_stream(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        snippet: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Complement of \\see jsonnet_vm_make."]
    pub fn jsonnet_destroy(vm: *mut JsonnetVm);
}
extern "C" {
    #[doc = " Indentation level when reformatting (number of spaeces).\n\n \\param n Number of spaces, must be > 0."]
    pub fn jsonnet_fmt_indent(vm: *mut JsonnetVm, n: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Indentation level when reformatting (number of spaeces).\n\n \\param n Number of spaces, must be > 0."]
    pub fn jsonnet_fmt_max_blank_lines(vm: *mut JsonnetVm, n: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Preferred style for string literals (\"\" or '').\n\n \\param c String style as a char ('d', 's', or 'l' (leave))."]
    pub fn jsonnet_fmt_string(vm: *mut JsonnetVm, c: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Preferred style for line comments (# or //).\n\n \\param c Comment style as a char ('h', 's', or 'l' (leave))."]
    pub fn jsonnet_fmt_comment(vm: *mut JsonnetVm, c: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Whether to add an extra space on the inside of arrays."]
    pub fn jsonnet_fmt_pad_arrays(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Whether to add an extra space on the inside of objects."]
    pub fn jsonnet_fmt_pad_objects(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Use syntax sugar where possible with field names."]
    pub fn jsonnet_fmt_pretty_field_names(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Sort top-level imports in alphabetical order"]
    pub fn jsonnet_fmt_sort_imports(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " If set to 1, will reformat the Jsonnet input after desugaring."]
    pub fn jsonnet_fmt_debug_desugaring(vm: *mut JsonnetVm, v: ::std::os::raw::c_int);
}
extern "C" {
    #[doc = " Reformat a file containing Jsonnet code, return a Jsonnet string.\n\n The returned string should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file containing Jsonnet code.\n \\param error Return by reference whether or not there was an error.\n \\returns Either Jsonnet code or the error message."]
    pub fn jsonnet_fmt_file(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Reformat a string containing Jsonnet code, return a Jsonnet string.\n\n The returned string should be cleaned up with jsonnet_realloc.\n\n \\param filename Path to a file (used in error messages).\n \\param snippet Jsonnet code to execute.\n \\param error Return by reference whether or not there was an error.\n \\returns Either Jsonnet code or the error message."]
    pub fn jsonnet_fmt_snippet(
        vm: *mut JsonnetVm,
        filename: *const ::std::os::raw::c_char,
        snippet: *const ::std::os::raw::c_char,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
