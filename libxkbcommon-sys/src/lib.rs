use libc::{c_char, c_int, c_void};
use libloading::Library;
use std::sync::OnceLock;

pub type xkb_context = c_void;
pub type xkb_keymap = c_void;
pub type xkb_state = c_void;
pub type xkb_rule_names = c_void;

type ContextNewFn = unsafe extern "C" fn(u32) -> *mut xkb_context;
type ContextUnrefFn = unsafe extern "C" fn(*mut xkb_context);
type KeymapNewFromNamesFn =
    unsafe extern "C" fn(*mut xkb_context, *const xkb_rule_names, u32, u32) -> *mut xkb_keymap;
type KeymapNewFromStringFn =
    unsafe extern "C" fn(*mut xkb_context, *const c_char, u32) -> *mut xkb_keymap;
type KeymapUnrefFn = unsafe extern "C" fn(*mut xkb_keymap);
type StateNewFn = unsafe extern "C" fn(*mut xkb_keymap) -> *mut xkb_state;
type StateUnrefFn = unsafe extern "C" fn(*mut xkb_state);
type StateUpdateMaskFn = unsafe extern "C" fn(*mut xkb_state, u32, u32, u32, u32) -> u32;
type StateKeyGetOneSymFn = unsafe extern "C" fn(*mut xkb_state, u32) -> u32;

struct XkbLib {
    _lib: Library,
    xkb_context_new: ContextNewFn,
    xkb_context_unref: ContextUnrefFn,
    xkb_keymap_new_from_names: KeymapNewFromNamesFn,
    xkb_keymap_new_from_string: KeymapNewFromStringFn,
    xkb_keymap_unref: KeymapUnrefFn,
    xkb_state_new: StateNewFn,
    xkb_state_unref: StateUnrefFn,
    xkb_state_update_mask: StateUpdateMaskFn,
    xkb_state_key_get_one_sym: StateKeyGetOneSymFn,
}

static LIB: OnceLock<Option<XkbLib>> = OnceLock::new();

fn load() -> Option<&'static XkbLib> {
    LIB.get_or_init(|| {
        let lib = unsafe {
            Library::new("libxkbcommon.so.0")
                .or_else(|_| Library::new("libxkbcommon.so"))
                .ok()?
        };
        let lib: &'static Library = Box::leak(Box::new(lib));
        Some(unsafe {
            XkbLib {
                xkb_context_new: *lib.get(b"xkb_context_new").ok()?,
                xkb_context_unref: *lib.get(b"xkb_context_unref").ok()?,
                xkb_keymap_new_from_names: *lib.get(b"xkb_keymap_new_from_names").ok()?,
                xkb_keymap_new_from_string: *lib.get(b"xkb_keymap_new_from_string").ok()?,
                xkb_keymap_unref: *lib.get(b"xkb_keymap_unref").ok()?,
                xkb_state_new: *lib.get(b"xkb_state_new").ok()?,
                xkb_state_unref: *lib.get(b"xkb_state_unref").ok()?,
                xkb_state_update_mask: *lib.get(b"xkb_state_update_mask").ok()?,
                xkb_state_key_get_one_sym: *lib.get(b"xkb_state_key_get_one_sym").ok()?,
            }
        })
    }).as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn xkb_context_new(flags: u32) -> *mut xkb_context {
    load().map(|l| (l.xkb_context_new)(flags)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xkb_context_unref(ctx: *mut xkb_context) {
    if let Some(l) = load() { (l.xkb_context_unref)(ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_new_from_names(
    ctx: *mut xkb_context,
    names: *const xkb_rule_names,
    flags: u32,
    format: u32,
) -> *mut xkb_keymap {
    load().map(|l| (l.xkb_keymap_new_from_names)(ctx, names, flags, format))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_new_from_string(
    ctx: *mut xkb_context,
    string: *const c_char,
    format: u32,
    flags: u32,
) -> *mut xkb_keymap {
    load().map(|l| (l.xkb_keymap_new_from_string)(ctx, string, format, flags))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xkb_keymap_unref(map: *mut xkb_keymap) {
    if let Some(l) = load() { (l.xkb_keymap_unref)(map) }
}

#[no_mangle]
pub unsafe extern "C" fn xkb_state_new(map: *mut xkb_keymap) -> *mut xkb_state {
    load().map(|l| (l.xkb_state_new)(map)).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xkb_state_unref(state: *mut xkb_state) {
    if let Some(l) = load() { (l.xkb_state_unref)(state) }
}

#[no_mangle]
pub unsafe extern "C" fn xkb_state_update_mask(
    state: *mut xkb_state,
    depressed: u32,
    latched: u32,
    locked: u32,
    depressed_layout: u32,
    latched_layout: u32,
    locked_layout: u32,
) -> u32 {
    load().map(|l| (l.xkb_state_update_mask)(state, depressed, latched, locked, depressed_layout))
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: u32) -> u32 {
    load().map(|l| (l.xkb_state_key_get_one_sym)(state, key)).unwrap_or(0)
}
