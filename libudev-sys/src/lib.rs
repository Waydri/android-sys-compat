use libc::{c_char, c_int, c_void};
use libloading::{Library, Symbol};
use std::sync::OnceLock;

type UdevNewFn = unsafe extern "C" fn() -> *mut c_void;
type UdevUnrefFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type UdevEnumerateNewFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type UdevEnumerateAddMatchSubsystemFn = unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int;
type UdevEnumerateScanDevicesFn = unsafe extern "C" fn(*mut c_void) -> c_int;
type UdevEnumerateGetListEntryFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type UdevEnumerateUnrefFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;
type UdevDeviceNewFromSyspathFn = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;
type UdevDeviceGetDevnodeFn = unsafe extern "C" fn(*mut c_void) -> *const c_char;
type UdevDeviceUnrefFn = unsafe extern "C" fn(*mut c_void) -> *mut c_void;

struct UdevLib {
    _lib: Library,
    udev_new: Symbol<'static, UdevNewFn>,
    udev_unref: Symbol<'static, UdevUnrefFn>,
    udev_enumerate_new: Symbol<'static, UdevEnumerateNewFn>,
    udev_enumerate_add_match_subsystem: Symbol<'static, UdevEnumerateAddMatchSubsystemFn>,
    udev_enumerate_scan_devices: Symbol<'static, UdevEnumerateScanDevicesFn>,
    udev_enumerate_get_list_entry: Symbol<'static, UdevEnumerateGetListEntryFn>,
    udev_enumerate_unref: Symbol<'static, UdevEnumerateUnrefFn>,
    udev_device_new_from_syspath: Symbol<'static, UdevDeviceNewFromSyspathFn>,
    udev_device_get_devnode: Symbol<'static, UdevDeviceGetDevnodeFn>,
    udev_device_unref: Symbol<'static, UdevDeviceUnrefFn>,
}

static UDEV: OnceLock<Option<UdevLib>> = OnceLock::new();

fn load() -> Option<&'static UdevLib> {
    UDEV.get_or_init(|| {
        let lib = unsafe {
            Library::new("libudev.so.1")
                .or_else(|_| Library::new("libudev.so"))
                .ok()?
        };
        Some(unsafe {
            UdevLib {
                udev_new: lib.get(b"udev_new").ok()?,
                udev_unref: lib.get(b"udev_unref").ok()?,
                udev_enumerate_new: lib.get(b"udev_enumerate_new").ok()?,
                udev_enumerate_add_match_subsystem: lib.get(b"udev_enumerate_add_match_subsystem").ok()?,
                udev_enumerate_scan_devices: lib.get(b"udev_enumerate_scan_devices").ok()?,
                udev_enumerate_get_list_entry: lib.get(b"udev_enumerate_get_list_entry").ok()?,
                udev_enumerate_unref: lib.get(b"udev_enumerate_unref").ok()?,
                udev_device_new_from_syspath: lib.get(b"udev_device_new_from_syspath").ok()?,
                udev_device_get_devnode: lib.get(b"udev_device_get_devnode").ok()?,
                udev_device_unref: lib.get(b"udev_device_unref").ok()?,
                _lib: lib,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn udev_new() -> *mut c_void {
    load().map(|l| (l.udev_new)()).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_unref(udev: *mut c_void) -> *mut c_void {
    load().map(|l| (l.udev_unref)(udev)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_enumerate_new(udev: *mut c_void) -> *mut c_void {
    load().map(|l| (l.udev_enumerate_new)(udev)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_enumerate_add_match_subsystem(
    enumerate: *mut c_void,
    subsystem: *const c_char,
) -> c_int {
    load().map(|l| (l.udev_enumerate_add_match_subsystem)(enumerate, subsystem)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn udev_enumerate_scan_devices(enumerate: *mut c_void) -> c_int {
    load().map(|l| (l.udev_enumerate_scan_devices)(enumerate)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn udev_enumerate_get_list_entry(enumerate: *mut c_void) -> *mut c_void {
    load().map(|l| (l.udev_enumerate_get_list_entry)(enumerate)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_enumerate_unref(enumerate: *mut c_void) -> *mut c_void {
    load().map(|l| (l.udev_enumerate_unref)(enumerate)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_device_new_from_syspath(
    udev: *mut c_void,
    syspath: *const c_char,
) -> *mut c_void {
    load().map(|l| (l.udev_device_new_from_syspath)(udev, syspath)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udev_device_get_devnode(device: *mut c_void) -> *const c_char {
    load().map(|l| (l.udev_device_get_devnode)(device)).unwrap_or(std::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn udev_device_unref(device: *mut c_void) -> *mut c_void {
    load().map(|l| (l.udev_device_unref)(device)).unwrap_or(std::ptr::null_mut())
}
