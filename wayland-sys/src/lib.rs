use libc::{c_char, c_int, c_void};
use libloading::{Library, Symbol};
use std::sync::OnceLock;

pub type wl_display = c_void;
pub type wl_registry = c_void;
pub type wl_proxy = c_void;

type DisplayConnectFn = unsafe extern "C" fn(*const c_char) -> *mut wl_display;
type DisplayDisconnectFn = unsafe extern "C" fn(*mut wl_display);
type DisplayDispatchFn = unsafe extern "C" fn(*mut wl_display) -> c_int;
type DisplayFlushFn = unsafe extern "C" fn(*mut wl_display) -> c_int;
type DisplayRoundtripFn = unsafe extern "C" fn(*mut wl_display) -> c_int;
type ProxyMarshalFlagsFn =
    unsafe extern "C" fn(*mut wl_proxy, u32, *const c_void, *const c_void, c_int) -> c_int;
type ProxyDestroyFn = unsafe extern "C" fn(*mut wl_proxy);

struct WaylandLib {
    _lib: Library,
    wl_display_connect: Symbol<'static, DisplayConnectFn>,
    wl_display_disconnect: Symbol<'static, DisplayDisconnectFn>,
    wl_display_dispatch: Symbol<'static, DisplayDispatchFn>,
    wl_display_flush: Symbol<'static, DisplayFlushFn>,
    wl_display_roundtrip: Symbol<'static, DisplayRoundtripFn>,
    wl_proxy_marshal_flags: Symbol<'static, ProxyMarshalFlagsFn>,
    wl_proxy_destroy: Symbol<'static, ProxyDestroyFn>,
}

static LIB: OnceLock<Option<WaylandLib>> = OnceLock::new();

fn load() -> Option<&'static WaylandLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libwayland-client.so.0")
                .or_else(|_| Library::new("libwayland-client.so"))
                .ok()?
        };
        Some(unsafe {
            WaylandLib {
                wl_display_connect: lib.get(b"wl_display_connect").ok()?,
                wl_display_disconnect: lib.get(b"wl_display_disconnect").ok()?,
                wl_display_dispatch: lib.get(b"wl_display_dispatch").ok()?,
                wl_display_flush: lib.get(b"wl_display_flush").ok()?,
                wl_display_roundtrip: lib.get(b"wl_display_roundtrip").ok()?,
                wl_proxy_marshal_flags: lib.get(b"wl_proxy_marshal_flags").ok()?,
                wl_proxy_destroy: lib.get(b"wl_proxy_destroy").ok()?,
                _lib: lib,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn wl_display_connect(name: *const c_char) -> *mut wl_display {
    load().map(|l| (l.wl_display_connect)(name)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn wl_display_disconnect(display: *mut wl_display) {
    if let Some(l) = load() { (l.wl_display_disconnect)(display) }
}

#[no_mangle]
pub unsafe extern "C" fn wl_display_dispatch(display: *mut wl_display) -> c_int {
    load().map(|l| (l.wl_display_dispatch)(display)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn wl_display_flush(display: *mut wl_display) -> c_int {
    load().map(|l| (l.wl_display_flush)(display)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn wl_display_roundtrip(display: *mut wl_display) -> c_int {
    load().map(|l| (l.wl_display_roundtrip)(display)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn wl_proxy_marshal_flags(
    proxy: *mut wl_proxy,
    opcode: u32,
    interface: *const c_void,
    version: u32,
    flags: u32,
    _varargs: *const c_void,
) -> *mut wl_proxy {
    let _ = version;
    let _ = flags;
    load().map(|l| (l.wl_proxy_marshal_flags)(proxy, opcode, interface, _varargs, 0))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn wl_proxy_destroy(proxy: *mut wl_proxy) {
    if let Some(l) = load() { (l.wl_proxy_destroy)(proxy) }
}
