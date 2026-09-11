#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use libc::{c_char, c_int, c_uint, c_void};

pub type dbus_bool_t = c_uint;
pub type dbus_int32_t = c_int;
pub type dbus_uint32_t = c_uint;
pub type dbus_int64_t = i64;
pub type dbus_uint64_t = u64;

pub mod DBusBusType {
    use libc::c_uint;
    pub type Type = c_uint;
    pub const Session: Type = 0;
    pub const System: Type = 1;
    pub const Starter: Type = 2;
}

pub mod DBusRequestNameReply {
    use libc::c_uint;
    pub type Type = c_uint;
    pub const PrimaryOwner: Type = 1;
    pub const InQueue: Type = 2;
    pub const Exists: Type = 3;
    pub const AlreadyOwner: Type = 4;
}

pub mod DBusReleaseNameReply {
    use libc::c_uint;
    pub type Type = c_uint;
    pub const Released: Type = 1;
    pub const NonExistent: Type = 2;
    pub const NotOwner: Type = 3;
}

pub mod DBusHandlerResult {
    use libc::c_uint;
    pub type Type = c_uint;
    pub const Handled: Type = 0;
    pub const NotYetHandled: Type = 1;
    pub const NeedMemory: Type = 2;
}

pub mod DBusDispatchStatus {
    use libc::c_uint;
    pub type Type = c_uint;
    pub const DataRemains: Type = 0;
    pub const Complete: Type = 1;
    pub const NeedMemory: Type = 2;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DBusError {
    pub name: *const c_char,
    pub message: *const c_char,
    pub dummy: c_uint,
    pub padding1: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DBusObjectPathVTable {
    pub unregister_function: Option<unsafe extern "C" fn(*mut DBusConnection, *mut c_void)>,
    pub message_function: Option<
        unsafe extern "C" fn(*mut DBusConnection, *mut DBusMessage, *mut c_void) -> c_uint,
    >,
    pub dbus_internal_pad1: Option<unsafe extern "C" fn(*mut c_void)>,
    pub dbus_internal_pad2: Option<unsafe extern "C" fn(*mut c_void)>,
    pub dbus_internal_pad3: Option<unsafe extern "C" fn(*mut c_void)>,
    pub dbus_internal_pad4: Option<unsafe extern "C" fn(*mut c_void)>,
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

pub const DBUS_NAME_FLAG_ALLOW_REPLACEMENT: c_int = 0x1;
pub const DBUS_NAME_FLAG_REPLACE_EXISTING: c_int = 0x2;
pub const DBUS_NAME_FLAG_DO_NOT_QUEUE: c_int = 0x4;

pub const DBUS_SEND_MESSAGE_FLAG_NONE: c_int = 0x0;
pub const DBUS_SEND_MESSAGE_FLAG_NO_REPLY_EXPECTED: c_int = 0x1;
pub const DBUS_SEND_MESSAGE_FLAG_NO_AUTO_START: c_int = 0x2;
pub const DBUS_SEND_MESSAGE_FLAG_ALLOW_INTERACTIVE_AUTHORIZATION: c_int = 0x4;

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

pub const DBUS_WATCH_READABLE: c_uint = 1;
pub const DBUS_WATCH_WRITABLE: c_uint = 2;
pub const DBUS_WATCH_ERROR: c_uint = 4;
pub const DBUS_WATCH_HANGUP: c_uint = 8;

#[no_mangle] pub unsafe extern "C" fn dbus_error_init(e: *mut DBusError) { if !e.is_null() { (*e).name = std::ptr::null(); (*e).message = std::ptr::null(); } }
#[no_mangle] pub unsafe extern "C" fn dbus_error_is_set(_e: *const DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_error_free(_e: *mut DBusError) {}
#[no_mangle] pub unsafe extern "C" fn dbus_error_has_name(_e: *const DBusError, _n: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_set_error(_e: *mut DBusError, _n: *const c_char, _m: *const c_char) {}
#[no_mangle] pub unsafe extern "C" fn dbus_set_error_from_message(_e: *mut DBusError, _m: *mut DBusMessage) -> dbus_bool_t { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_bus_get(_t: DBusBusType::Type, _e: *mut DBusError) -> *mut DBusConnection { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_get_private(_t: DBusBusType::Type, _e: *mut DBusError) -> *mut DBusConnection { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_bus_get_unique_name(_c: *mut DBusConnection) -> *const c_char { std::ptr::null() }
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
#[no_mangle] pub unsafe extern "C" fn dbus_connection_get_dispatch_status(_c: *mut DBusConnection) -> DBusDispatchStatus::Type { DBusDispatchStatus::Complete }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_dispatch(_c: *mut DBusConnection) -> DBusHandlerResult::Type { DBusHandlerResult::NotYetHandled }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_get_is_connected(_c: *mut DBusConnection) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_set_exit_on_disconnect(_c: *mut DBusConnection, _b: dbus_bool_t) {}
#[no_mangle] pub unsafe extern "C" fn dbus_connection_pop_message(_c: *mut DBusConnection) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send(_c: *mut DBusConnection, _m: *mut DBusMessage, _s: *mut dbus_uint32_t) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send_with_reply(_c: *mut DBusConnection, _m: *mut DBusMessage, _p: *mut *mut DBusPendingCall, _t: c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_send_with_reply_and_block(_c: *mut DBusConnection, _m: *mut DBusMessage, _t: c_int, _e: *mut DBusError) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_open_private(_a: *const c_char, _e: *mut DBusError) -> *mut DBusConnection { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_has_messages_to_send(_c: *mut DBusConnection) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_add_filter(
    _c: *mut DBusConnection,
    _f: Option<unsafe extern "C" fn(*mut DBusConnection, *mut DBusMessage, *mut c_void) -> c_uint>,
    _u: *mut c_void,
    _fr: Option<unsafe extern "C" fn(*mut c_void)>,
) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_try_register_object_path(
    _c: *mut DBusConnection, _p: *const c_char, _v: *const DBusObjectPathVTable, _u: *mut c_void, _e: *mut DBusError,
) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_unregister_object_path(_c: *mut DBusConnection, _p: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_list_registered(_c: *mut DBusConnection, _p: *const c_char, _o: *mut *mut *mut c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_connection_set_watch_functions(
    _c: *mut DBusConnection,
    _add: Option<unsafe extern "C" fn(*mut DBusWatch, *mut c_void) -> dbus_bool_t>,
    _remove: Option<unsafe extern "C" fn(*mut DBusWatch, *mut c_void)>,
    _toggled: Option<unsafe extern "C" fn(*mut DBusWatch, *mut c_void)>,
    _u: *mut c_void, _fr: Option<unsafe extern "C" fn(*mut c_void)>,
) -> dbus_bool_t { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_message_new_method_call(_d: *const c_char, _p: *const c_char, _i: *const c_char, _m: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_signal(_p: *const c_char, _i: *const c_char, _n: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_method_return(_m: *mut DBusMessage) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_new_error(_m: *mut DBusMessage, _n: *const c_char, _msg: *const c_char) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_copy(_m: *const DBusMessage) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_ref(m: *mut DBusMessage) -> *mut DBusMessage { m }
#[no_mangle] pub unsafe extern "C" fn dbus_message_unref(_m: *mut DBusMessage) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_type(_m: *mut DBusMessage) -> c_int { DBUS_MESSAGE_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_path(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_interface(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_member(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_sender(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_sender(_m: *mut DBusMessage, _s: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_destination(_m: *mut DBusMessage) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_destination(_m: *mut DBusMessage, _d: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_path(_m: *mut DBusMessage, _p: *const c_char) -> bool { false }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_signature(_m: *mut DBusMessage, _c: dbus_bool_t) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_no_reply(_m: *mut DBusMessage) -> dbus_bool_t { 1 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_no_reply(_m: *mut DBusMessage, _b: dbus_bool_t) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_auto_start(_m: *mut DBusMessage) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_auto_start(_m: *mut DBusMessage, _b: dbus_bool_t) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_serial(_m: *mut DBusMessage) -> dbus_uint32_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_get_reply_serial(_m: *mut DBusMessage) -> dbus_uint32_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_set_serial(_m: *mut DBusMessage, _s: dbus_uint32_t) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_method_call(_m: *mut DBusMessage, _i: *const c_char, _meth: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_signal(_m: *mut DBusMessage, _i: *const c_char, _sig: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_is_error(_m: *mut DBusMessage, _n: *const c_char) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_marshal(_m: *mut DBusMessage, _d: *mut *mut c_char, _l: *mut c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_demarshal(_d: *const c_char, _l: c_int, _e: *mut DBusError) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_demarshal_bytes_needed(_d: *const c_char, _l: c_int) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_init(_m: *mut DBusMessage, _i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_init_append(_m: *mut DBusMessage, _i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_append_basic(_i: *mut DBusMessageIter, _t: c_int, _v: *const c_void) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_append_fixed_array(_i: *mut DBusMessageIter, _t: c_int, _v: *const c_void, _n: c_int) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_open_container(_i: *mut DBusMessageIter, _t: c_int, _s: *const c_char, _c: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_close_container(_i: *mut DBusMessageIter, _c: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_abandon_container(_i: *mut DBusMessageIter, _c: *mut DBusMessageIter) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_arg_type(_i: *mut DBusMessageIter) -> c_int { DBUS_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_basic(_i: *mut DBusMessageIter, _v: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_element_type(_i: *mut DBusMessageIter) -> c_int { DBUS_TYPE_INVALID }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_fixed_array(_i: *mut DBusMessageIter, _v: *mut c_void, _n: *mut c_int) {}
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_get_signature(_i: *mut DBusMessageIter) -> *mut c_char { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_next(_i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_has_next(_i: *mut DBusMessageIter) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_message_iter_recurse(_i: *mut DBusMessageIter, _s: *mut DBusMessageIter) {}

#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_ref(p: *mut DBusPendingCall) -> *mut DBusPendingCall { p }
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_unref(_p: *mut DBusPendingCall) {}
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_steal_reply(_p: *mut DBusPendingCall) -> *mut DBusMessage { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_block(_p: *mut DBusPendingCall) {}
#[no_mangle] pub unsafe extern "C" fn dbus_pending_call_get_completed(_p: *mut DBusPendingCall) -> dbus_bool_t { 1 }

#[no_mangle] pub unsafe extern "C" fn dbus_watch_handle(_w: *mut DBusWatch, _f: c_uint) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_watch_get_unix_fd(_w: *mut DBusWatch) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn dbus_watch_get_enabled(_w: *mut DBusWatch) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_watch_get_flags(_w: *mut DBusWatch) -> c_uint { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_get_local_machine_id() -> *mut c_char { std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn dbus_signature_validate_single(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_validate_path(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_validate_member(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_validate_interface(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_validate_bus_name(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }
#[no_mangle] pub unsafe extern "C" fn dbus_validate_error_name(_s: *const c_char, _e: *mut DBusError) -> dbus_bool_t { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_threads_init_default() -> dbus_bool_t { 0 }

#[no_mangle] pub unsafe extern "C" fn dbus_free(_p: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dbus_free_string_array(_p: *mut *mut c_char) {}
