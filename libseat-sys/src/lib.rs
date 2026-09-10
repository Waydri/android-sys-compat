use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type seat = c_void;

type SeatOpenSeatFn = unsafe extern "C" fn(
    *mut *mut seat,
    *const c_void,
    *mut c_void,
) -> c_int;
type SeatCloseSeatFn = unsafe extern "C" fn(*mut seat) -> c_int;
type SeatOpenDeviceFn = unsafe extern "C" fn(*mut seat, *const c_char) -> c_int;
type SeatCloseDeviceFn = unsafe extern "C" fn(*mut seat, c_int) -> c_int;
type SeatDispatchFn = unsafe extern "C" fn(*mut seat, c_int) -> c_int;

struct SeatLib {
    libseat_open_seat: SeatOpenSeatFn,
    libseat_close_seat: SeatCloseSeatFn,
    libseat_open_device: SeatOpenDeviceFn,
    libseat_close_device: SeatCloseDeviceFn,
    libseat_dispatch: SeatDispatchFn,
}

static LIB: OnceLock<Option<SeatLib>> = OnceLock::new();

fn load() -> Option<&'static SeatLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libseat.so.1")
                .or_else(|_| Library::new("libseat.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            SeatLib {
                libseat_open_seat: *lib.get(b"libseat_open_seat").ok()?,
                libseat_close_seat: *lib.get(b"libseat_close_seat").ok()?,
                libseat_open_device: *lib.get(b"libseat_open_device").ok()?,
                libseat_close_device: *lib.get(b"libseat_close_device").ok()?,
                libseat_dispatch: *lib.get(b"libseat_dispatch").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn libseat_open_seat(
    seat_out: *mut *mut seat,
    interface: *const c_void,
    user_data: *mut c_void,
) -> c_int {
    load().map(|l| (l.libseat_open_seat)(seat_out, interface, user_data)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libseat_close_seat(s: *mut seat) -> c_int {
    load().map(|l| (l.libseat_close_seat)(s)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libseat_open_device(s: *mut seat, path: *const c_char) -> c_int {
    load().map(|l| (l.libseat_open_device)(s, path)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libseat_close_device(s: *mut seat, fd: c_int) -> c_int {
    load().map(|l| (l.libseat_close_device)(s, fd)).unwrap_or(-1)
}

#[no_mangle]
pub unsafe extern "C" fn libseat_dispatch(s: *mut seat, timeout: c_int) -> c_int {
    load().map(|l| (l.libseat_dispatch)(s, timeout)).unwrap_or(-1)
}
