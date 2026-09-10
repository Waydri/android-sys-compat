use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type ei = c_void;
pub type ei_device = c_void;
pub type ei_event = c_void;

type EiNewFn = unsafe extern "C" fn(*const c_void, *const c_void) -> *mut ei;
type EiUnrefFn = unsafe extern "C" fn(*mut ei);
type EiSetupBackendFn = unsafe extern "C" fn(*mut ei, *const c_void, *mut c_void) -> c_int;
type EiDispatchFn = unsafe extern "C" fn(*mut ei) -> c_int;
type EiEventFn = unsafe extern "C" fn(*mut ei, *mut c_void) -> *mut ei_event;
type EiEventUnrefFn = unsafe extern "C" fn(*mut ei_event);

struct EiLib {
    ei_new: EiNewFn,
    ei_unref: EiUnrefFn,
    ei_setup_backend_socket: EiSetupBackendFn,
    ei_dispatch: EiDispatchFn,
    ei_event: EiEventFn,
    ei_event_unref: EiEventUnrefFn,
}

static LIB: OnceLock<Option<EiLib>> = OnceLock::new();

fn load() -> Option<&'static EiLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libei.so.1")
                .or_else(|_| Library::new("libei.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            EiLib {
                ei_new: *lib.get(b"ei_new").ok()?,
                ei_unref: *lib.get(b"ei_unref").ok()?,
                ei_setup_backend_socket: *lib.get(b"ei_setup_backend_socket").ok()?,
                ei_dispatch: *lib.get(b"ei_dispatch").ok()?,
                ei_event: *lib.get(b"ei_event").ok()?,
                ei_event_unref: *lib.get(b"ei_event_unref").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn ei_new(
    interface: *const c_void,
    user_data: *const c_void,
) -> *mut ei {
    load().map(|l| (l.ei_new)(interface, user_data)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn ei_unref(ei: *mut ei) {
    if let Some(l) = load() { (l.ei_unref)(ei) }
}

#[no_mangle]
pub unsafe extern "C" fn ei_setup_backend_socket(
    ei: *mut ei,
    socket: *const c_char,
) -> c_int {
    load().map(|l| (l.ei_setup_backend_socket)(ei, socket as *const c_void, std::ptr::null_mut()))
        .unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn ei_dispatch(ei: *mut ei) -> c_int {
    load().map(|l| (l.ei_dispatch)(ei)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn ei_event(ei: *mut ei, user_data: *mut c_void) -> *mut ei_event {
    load().map(|l| (l.ei_event)(ei, user_data)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn ei_event_unref(event: *mut ei_event) {
    if let Some(l) = load() { (l.ei_event_unref)(event) }
}
