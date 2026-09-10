use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type libinput = c_void;
pub type libinput_device = c_void;
pub type libinput_event = c_void;

type LibinputPathCreateContextFn = unsafe extern "C" fn(*const c_char, *mut c_void) -> *mut libinput;
type LibinputUdevCreateContextFn = unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut libinput;
type LibinputUnrefFn = unsafe extern "C" fn(*mut libinput) -> *mut libinput;
type LibinputGetEventsFn = unsafe extern "C" fn(*mut libinput) -> c_int;
type LibinputDispatchFn = unsafe extern "C" fn(*mut libinput) -> c_int;
type LibinputSuspendFn = unsafe extern "C" fn(*mut libinput) -> c_int;
type LibinputResumeFn = unsafe extern "C" fn(*mut libinput) -> c_int;

struct LibinputLib {
    _lib: Library,
    libinput_path_create_context: LibinputPathCreateContextFn,
    libinput_udev_create_context: LibinputUdevCreateContextFn,
    libinput_unref: LibinputUnrefFn,
    libinput_get_fd: Symbol<'static, unsafe extern "C" fn(*mut libinput) -> c_int>,
    libinput_dispatch: LibinputDispatchFn,
    libinput_suspend: LibinputSuspendFn,
    libinput_resume: LibinputResumeFn,
}

static LIB: OnceLock<Option<LibinputLib>> = OnceLock::new();

fn load() -> Option<&'static LibinputLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libinput.so.10")
                .or_else(|_| Library::new("libinput.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            LibinputLib {
                libinput_path_create_context: *lib.get(b"libinput_path_create_context").ok()?,
                libinput_udev_create_context: *lib.get(b"libinput_udev_create_context").ok()?,
                libinput_unref: *lib.get(b"libinput_unref").ok()?,
                libinput_get_fd: *lib.get(b"libinput_get_fd").ok()?,
                libinput_dispatch: *lib.get(b"libinput_dispatch").ok()?,
                libinput_suspend: *lib.get(b"libinput_suspend").ok()?,
                libinput_resume: *lib.get(b"libinput_resume").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn libinput_path_create_context(
    interface: *const c_void,
    user_data: *mut c_void,
) -> *mut libinput {
    load().map(|l| (l.libinput_path_create_context)(interface as *const c_char, user_data))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn libinput_udev_create_context(
    interface: *const c_void,
    user_data: *mut c_void,
    udev: *mut c_void,
) -> *mut libinput {
    let _ = user_data;
    load().map(|l| (l.libinput_udev_create_context)(interface as *const c_char, udev))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn libinput_unref(li: *mut libinput) -> *mut libinput {
    load().map(|l| (l.libinput_unref)(li)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn libinput_get_fd(li: *mut libinput) -> c_int {
    load().map(|l| (l.libinput_get_fd)(li)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libinput_dispatch(li: *mut libinput) -> c_int {
    load().map(|l| (l.libinput_dispatch)(li)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libinput_suspend(li: *mut libinput) -> c_int {
    load().map(|l| (l.libinput_suspend)(li)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libinput_resume(li: *mut libinput) -> c_int {
    load().map(|l| (l.libinput_resume)(li)).unwrap_or(-1)
}
