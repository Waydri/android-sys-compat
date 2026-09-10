use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type gbm_device = c_void;
pub type gbm_bo = c_void;
pub type gbm_surface = c_void;

type GbmCreateDeviceFn = unsafe extern "C" fn(c_int) -> *mut gbm_device;
type GbmDeviceDestroyFn = unsafe extern "C" fn(*mut gbm_device);
type GbmCreateBufferObjectFn =
    unsafe extern "C" fn(*mut gbm_device, u32, u32, u32, u32) -> *mut gbm_bo;
type GbmBoDestroyFn = unsafe extern "C" fn(*mut gbm_bo);
type GbmCreateSurfaceFn =
    unsafe extern "C" fn(*mut gbm_device, u32, u32, u32, u32) -> *mut gbm_surface;
type GbmSurfaceDestroyFn = unsafe extern "C" fn(*mut gbm_surface);

struct GbmLib {
    gbm_create_device: GbmCreateDeviceFn,
    gbm_device_destroy: GbmDeviceDestroyFn,
    gbm_bo_create: GbmCreateBufferObjectFn,
    gbm_bo_destroy: GbmBoDestroyFn,
    gbm_surface_create: GbmCreateSurfaceFn,
    gbm_surface_destroy: GbmSurfaceDestroyFn,
}

static LIB: OnceLock<Option<GbmLib>> = OnceLock::new();

fn load() -> Option<&'static GbmLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libgbm.so.1")
                .or_else(|_| Library::new("libgbm.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            GbmLib {
                gbm_create_device: *lib.get(b"gbm_create_device").ok()?,
                gbm_device_destroy: *lib.get(b"gbm_device_destroy").ok()?,
                gbm_bo_create: *lib.get(b"gbm_bo_create").ok()?,
                gbm_bo_destroy: *lib.get(b"gbm_bo_destroy").ok()?,
                gbm_surface_create: *lib.get(b"gbm_surface_create").ok()?,
                gbm_surface_destroy: *lib.get(b"gbm_surface_destroy").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn gbm_create_device(fd: c_int) -> *mut gbm_device {
    load().map(|l| (l.gbm_create_device)(fd)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn gbm_device_destroy(dev: *mut gbm_device) {
    if let Some(l) = load() { (l.gbm_device_destroy)(dev) }
}

#[no_mangle]
pub unsafe extern "C" fn gbm_bo_create(
    dev: *mut gbm_device,
    width: u32,
    height: u32,
    format: u32,
    flags: u32,
) -> *mut gbm_bo {
    load().map(|l| (l.gbm_bo_create)(dev, width, height, format, flags))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn gbm_bo_destroy(bo: *mut gbm_bo) {
    if let Some(l) = load() { (l.gbm_bo_destroy)(bo) }
}

#[no_mangle]
pub unsafe extern "C" fn gbm_surface_create(
    dev: *mut gbm_device,
    width: u32,
    height: u32,
    format: u32,
    flags: u32,
) -> *mut gbm_surface {
    load().map(|l| (l.gbm_surface_create)(dev, width, height, format, flags))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn gbm_surface_destroy(surf: *mut gbm_surface) {
    if let Some(l) = load() { (l.gbm_surface_destroy)(surf) }
}
