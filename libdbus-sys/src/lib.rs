#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use libc::{c_char, c_int, c_uint, c_void};

pub type dbus_bool_t = c_uint;
pub type dbus_int32_t = c_int;
pub type dbus_uint32_t = c_uint;
pub type dbus_int64_t = i64;
pub type dbus_uint64_t = u64;

#[repr(C)]
pub struct DBusError {
    pub name: *const c_char,
    pub message: *const c_char,
    pub dummy1: c_uint,
    pub padding1: *mut c_void,
    pub dummy6: c_int,
    pub padding2: *mut c_void,
    pub padding3: *mut c_void,
    pub padding4: *mut c_void,
}

#[repr(C)]
pub struct DBusMessageIter {
    pub dummy1: *mut c_void,
    pub dummy2: *mut c_void,
    pub dummy3: dbus_uint32_t,
    pub dummy4: c_int,
    pub dummy5: c_int,
    pub dummy6: c_int,
    pub dummy7: c_int,
    pub dummy8: c_int,
    pub dummy9: c_int,
    pub dummy10: c_int,
    pub dummy11: c_int,
    pub pad1: c_int,
    pub pad2: c_int,
    pub pad3: *mut c_void,
}

pub type DBusConnection = c_void;
pub type DBusMessage = c_void;
pub type DBusPendingCall = c_void;
pub type DBusWatch = c_void;
pub type DBusTimeout = c_void;

pub const DBUS_BUS_SESSION: c_int = 0;
pub const DBUS_BUS_SYSTEM: c_int = 1;
pub const DBUS_BUS_STARTER: c_int = 2;

pub const DBUS_MESSAGE_TYPE_INVALID: c_int = 0;
pub const DBUS_MESSAGE_TYPE_METHOD_CALL: c_int = 1;
pub const DBUS_MESSAGE_TYPE_METHOD_RETURN: c_int = 2;
pub const DBUS_MESSAGE_TYPE_ERROR: c_int = 3;
pub const DBUS_MESSAGE_TYPE_SIGNAL: c_int = 4;

pub const DBUS_MESSAGE_FLAG_NO_REPLY_EXPECTED: c_int = 0x1;
pub const DBUS_MESSAGE_FLAG_NO_AUTO_START: c_int = 0x2;
pub const DBUS_MESSAGE_FLAG_ALLOW_INTERACTIVE_AUTHORIZATION: c_int = 0x4;

pub const DBUS_SEND_MESSAGE_FLAG_NONE: c_int = 0x0;
pub const DBUS_SEND_MESSAGE_FLAG_NO_REPLY_EXPECTED: c_int = 0x1;
pub const DBUS_SEND_MESSAGE_FLAG_NO_AUTO_START: c_int = 0x2;
pub const DBUS_SEND_MESSAGE_FLAG_ALLOW_INTERACTIVE_AUTHORIZATION: c_int = 0x4;

pub const DBUS_NAME_FLAG_ALLOW_REPLACEMENT: c_int = 0x1;
pub const DBUS_NAME_FLAG_REPLACE_EXISTING: c_int = 0x2;
pub const DBUS_NAME_FLAG_DO_NOT_QUEUE: c_int = 0x4;

pub const DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: c_int = 1;
pub const DBUS_REQUEST_NAME_REPLY_IN_QUEUE: c_int = 2;
pub const DBUS_REQUEST_NAME_REPLY_EXISTS: c_int = 3;
pub const DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: c_int = 4;

pub const DBUS_RELEASE_NAME_REPLY_RELEASED: c_int = 1;
pub const DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: c_int = 2;
pub const DBUS_RELEASE_NAME_REPLY_NOT_OWNER: c_int = 3;

pub const DBUS_HANDLER_RESULT_HANDLED: c_int = 0;
pub const DBUS_HANDLER_RESULT_NOT_YET_HANDLED: c_int = 1;
pub const DBUS_HANDLER_RESULT_NEED_MEMORY: c_int = 2;

pub const DBUS_DISPATCH_DATA_REMAINS: c_int = 0;
pub const DBUS_DISPATCH_COMPLETE: c_int = 1;
pub const DBUS_DISPATCH_NEED_MEMORY: c_int = 2;

pub const DBUS_TYPE_INVALID: c_int = 0;
pub const DBUS_TYPE_BYTE: c_int = b'y' as c_int;
pub const DBUS_TYPE_BOOLEAN: c_int = b'b' as c_int;
pub const DBUS_TYPE_INT16: c_int = b'n' as c_int;
pub const DBUS_TYPE_UINT16: c_int = b'q' as c_int;
pub const DBUS_TYPE_INT32: c_int = b'i' as c_int;
pub const DBUS_TYPE_UINT32: c_int = b'u' as c_int;
pub const DBUS_TYPE_INT64: c_int = b'x' as c_int;
pub const DBUS_TYPE_UINT64: c_int = b't' as c_int;
pub const DBUS_TYPE_DOUBLE: c_int = b'd' as c_int;
pub const DBUS_TYPE_STRING: c_int = b's' as c_int;
pub const DBUS_TYPE_OBJECT_PATH: c_int = b'o' as c_int;
pub const DBUS_TYPE_SIGNATURE: c_int = b'g' as c_int;
pub const DBUS_TYPE_UNIX_FD: c_int = b'h' as c_int;
pub const DBUS_TYPE_ARRAY: c_int = b'a' as c_int;
pub const DBUS_TYPE_VARIANT: c_int = b'v' as c_int;
pub const DBUS_TYPE_STRUCT: c_int = b'r' as c_int;
pub const DBUS_TYPE_DICT_ENTRY: c_int = b'e' as c_int;

#[no_mangle] pub unsafe extern "C" fn dbus_error_init(e: *mut DBusError) { if !e.is_null() { (*e).name = std::ptr::null(); (*e).message = std::ptr::null(); } }
#[no_mangle] pub unsafe extern "C" fn dbus_error_is_set(_e: *const DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_error_free(_e: *mut DBusError) {}
#[no_mangle] pub unsafe extern "C" fn dbus_error_has_name(_e: *const DBusError, _n: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_set_error(_e: *mut DBusError, _n: *const c_char, _m: *const c_char) {}
#[no_mangle] pub unsafe extern "C" fn dbus_set_error_from_message(_e: *mut DBusError, _m: *mut DBusMessage) -> dbus_bool_t { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_bus_get(_t: c_int, _e: *mut DBusError) -> *mut DBusConnection { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_get_private(_t: c_int, _e: *mut DBusError) -> *mut DBusConnection { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_register(_c: *mut DBusConnection, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_request_name(_c: *mut DBusConnection, _n: *const c_char, _f: c_uint, _e: *mut DBusError) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_release_name(_c: *mut DBusConnection, _n: *const c_char, _e: *mut DBusError) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_add_match(_c: *mut DBusConnection, _r: *const c_char, _e: *mut DBusError) {}
#[no_mangle] pub unsafe extern "C" fn dbus_bus_remove_match(_c: *mut DBusConnection, _r: *const c_char, _e: *mut DBusError) {}

#[no_mangle] pub unsafe extern "C" fn dbus_connection_close(_c: *mut DBusConnection) {}
#[no_mangle] pub unsafe extern "C" fn dbus_connection_unref(_c: *mut DBusConnection) {}
#[no_mangle] pub unsafe extern "C" fn dbus_connection_ref(c: *mut DBusConnection) -> *mut DBusConnection { c }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_flush(_c: *mut DBusConnection) {}
#[no_mangle] pub unsafe extern "C" fn dbus_connection_read_write(_c: *mut DBusConnection, _t: c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_read_write_dispatch(_c: *mut DBusConnection, _t: c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_get_dispatch_status(_c: *mut DBusConnection) -> c_int { DBUS_DISPATCH_COMPLETE }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_dispatch(_c: *mut DBusConnection) -> c_int { DBUS_HANDLER_RESULT_NOT_YET_HANDLED }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_get_is_connected(_c: *mut DBusConnection) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_set_exit_on_disconnect(_c: *mut DBusConnection, _b: dbus_bool_t) {}
#[no_mangle] pub unsafe extern "C" fn dbus_connection_pop_message(_c: *mut DBusConnection) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send(_c: *mut DBusConnection, _m: *mut DBusMessage, _s: *mut dbus_uint32_t) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send_with_reply(_c: *mut DBusConnection, _m: *mut DBusMessage, _p: *mut *mut DBusPendingCall, _t: c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send_with_reply_and_block(_c: *mut DBusConnection, _m: *mut DBusMessage, _t: c_int, _e: *mut DBusError) -> *mut DBusMessage { std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn dbus_message_new_method_call(_d: *const c_char, _p: *const c_char, _i: *const c_char, _m: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_signal(_p: *const c_char, _i: *const c_char, _n: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_method_return(_m: *mut DBusMessage) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_error(_m: *mut DBusMessage, _n: *const c_char, _msg: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_ref(m: *mut DBusMessage) -> *mut DBusMessage { m }
#[no_mangle] pub unsafe extern "C" fn dbus_message_unref(_m: *mut DBusMessage) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_type(_m: *mut DBusMessage) -> c_int { DBUS_MESSAGE_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_path(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_interface(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_member(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_sender(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_destination(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_signature(_m: *mut DBusMessage, _c: dbus_bool_t) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_no_reply(_m: *mut DBusMessage) -> dbus_bool_t { 1 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_serial(_m: *mut DBusMessage) -> dbus_uint32_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_serial(_m: *mut DBusMessage, _s: dbus_uint32_t) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_method_call(_m: *mut DBusMessage, _i: *const c_char, _meth: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_signal(_m: *mut DBusMessage, _i: *const c_char, _sig: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_error(_m: *mut DBusMessage, _n: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_init(_m: *mut DBusMessage, _i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_init_append(_m: *mut DBusMessage, _i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_append_basic(_i: *mut DBusMessageIter, _t: c_int, _v: *const c_void) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_open_container(_i: *mut DBusMessageIter, _t: c_int, _s: *const c_char, _c: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_close_container(_i: *mut DBusMessageIter, _c: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_abandon_container(_i: *mut DBusMessageIter, _c: *mut DBusMessageIter) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_arg_type(_i: *mut DBusMessageIter) -> c_int { DBUS_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_basic(_i: *mut DBusMessageIter, _v: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_element_type(_i: *mut DBusMessageIter) -> c_int { DBUS_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_next(_i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_has_next(_i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_recurse(_i: *mut DBusMessageIter, _s: *mut DBusMessageIter) {}

#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_ref(p: *mut DBusPendingCall) -> *mut DBusPendingCall { p }
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_unref(_p: *mut DBusPendingCall) {}
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_steal_reply(_p: *mut DBusPendingCall) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_block(_p: *mut DBusPendingCall) {}
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_get_completed(_p: *mut DBusPendingCall) -> dbus_bool_t { 1 }

#[no_mangle] pub unsafe extern "C" fn dbus_free(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dbus_free_string_array(_p: *mut *mut c_char) {}
