use libc::{c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type di_info = c_void;

type InfoParseFn = unsafe extern "C" fn(*const c_void, usize) -> *mut di_info;
type InfoDestroyFn = unsafe extern "C" fn(*mut di_info);
type InfoGetMakeFn = unsafe extern "C" fn(*const di_info) -> *const c_void;
type InfoGetModelFn = unsafe extern "C" fn(*const di_info) -> *const c_void;

struct DiLib {
    di_info_parse_edid: InfoParseFn,
    di_info_destroy: InfoDestroyFn,
    di_info_get_make: InfoGetMakeFn,
    di_info_get_model: InfoGetModelFn,
}

static LIB: OnceLock<Option<DiLib>> = OnceLock::new();

fn load() -> Option<&'static DiLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libdisplay-info.so.2")
                .or_else(|_| Library::new("libdisplay-info.so.1"))
                .or_else(|_| Library::new("libdisplay-info.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            DiLib {
                di_info_parse_edid: *lib.get(b"di_info_parse_edid").ok()?,
                di_info_destroy: *lib.get(b"di_info_destroy").ok()?,
                di_info_get_make: *lib.get(b"di_info_get_make").ok()?,
                di_info_get_model: *lib.get(b"di_info_get_model").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn di_info_parse_edid(data: *const c_void, size: usize) -> *mut di_info {
    load().map(|l| (l.di_info_parse_edid)(data, size)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn di_info_destroy(info: *mut di_info) {
    if let Some(l) = load() { (l.di_info_destroy)(info) }
}

#[no_mangle]
pub unsafe extern "C" fn di_info_get_make(info: *const di_info) -> *const c_void {
    load().map(|l| (l.di_info_get_make)(info)).unwrap_or(std::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn di_info_get_model(info: *const di_info) -> *const c_void {
    load().map(|l| (l.di_info_get_model)(info)).unwrap_or(std::ptr::null())
}
