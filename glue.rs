/* automatically generated by rust-bindgen */

import libc::*;

type enum_StubType = c_uint;
const PROPERTY_STUB: u32 = 0_u32;
const STRICT_PROPERTY_STUB: u32 = 1_u32;
const ENUMERATE_STUB: u32 = 2_u32;
const CONVERT_STUB: u32 = 3_u32;
const RESOLVE_STUB: u32 = 4_u32;

#[link_name="jsglue"]
extern mod bindgen {

#[rust_stack]
fn GetJSClassHookStubPointer(++_type: enum_StubType) -> *c_void;

#[rust_stack]
fn RUST_JSVAL_IS_NULL(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_IS_VOID(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_IS_INT(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_TO_INT(++v: jsval) -> int32_t;

#[rust_stack]
fn RUST_INT_TO_JSVAL(++v: int32_t) -> jsval;

#[rust_stack]
fn RUST_JSVAL_IS_DOUBLE(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_TO_DOUBLE(++v: jsval) -> c_double;

#[rust_stack]
fn RUST_DOUBLE_TO_JSVAL(++v: c_double) -> jsval;

#[rust_stack]
fn RUST_UINT_TO_JSVAL(++v: uint32_t) -> jsval;

#[rust_stack]
fn RUST_JSVAL_IS_NUMBER(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_IS_STRING(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_TO_STRING(++v: jsval) -> *JSString;

#[rust_stack]
fn RUST_STRING_TO_JSVAL(++v: *JSString) -> jsval;

#[rust_stack]
fn RUST_JSVAL_TO_OBJECT(++v: jsval) -> *JSObject;

#[rust_stack]
fn RUST_OBJECT_TO_JSVAL(++v: *JSObject) -> jsval;

#[rust_stack]
fn RUST_JSVAL_IS_BOOLEAN(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_TO_BOOLEAN(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_BOOLEAN_TO_JSVAL(++v: JSBool) -> jsval;

#[rust_stack]
fn RUST_JSVAL_IS_PRIMITIVE(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_IS_GCTHING(++v: jsval) -> JSBool;

#[rust_stack]
fn RUST_JSVAL_TO_GCTHING(++v: jsval) -> *c_void;

#[rust_stack]
fn RUST_PRIVATE_TO_JSVAL(++v: *c_void) -> jsval;

#[rust_stack]
fn RUST_JSVAL_TO_PRIVATE(++v: jsval) -> *c_void;

}
