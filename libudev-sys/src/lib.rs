#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use libc::{c_char, c_int, c_void, dev_t};

pub type udev = c_void;
pub type udev_device = c_void;
pub type udev_enumerate = c_void;
pub type udev_list_entry = c_void;
pub type udev_monitor = c_void;
pub type udev_queue = c_void;
pub type udev_hwdb = c_void;

#[no_mangle] pub unsafe extern "C" fn udev_new() -> *mut udev { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_unref(_: *mut udev) -> *mut udev { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_ref(_: *mut udev) -> *mut udev { std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn udev_device_new_from_syspath(_: *mut udev, _: *const c_char) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_new_from_devnum(_: *mut udev, _: c_char, _: dev_t) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_new_from_subsystem_sysname(_: *mut udev, _: *const c_char, _: *const c_char) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_new_from_device_id(_: *mut udev, _: *const c_char) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_unref(_: *mut udev_device) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_ref(_: *mut udev_device) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_parent(_: *mut udev_device) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_parent_with_subsystem_devtype(_: *mut udev_device, _: *const c_char, _: *const c_char) -> *mut udev_device { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_devnode(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_is_initialized(_: *mut udev_device) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_devnum(_: *mut udev_device) -> dev_t { 0 }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_devpath(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_devtype(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_subsystem(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_syspath(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_sysname(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_sysnum(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_driver(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_action(_: *mut udev_device) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_seqnum(_: *mut udev_device) -> u64 { 0 }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_property_value(_: *mut udev_device, _: *const c_char) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_sysattr_value(_: *mut udev_device, _: *const c_char) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_device_set_sysattr_value(_: *mut udev_device, _: *const c_char, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_properties_list_entry(_: *mut udev_device) -> *mut udev_list_entry { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_device_get_sysattr_list_entry(_: *mut udev_device) -> *mut udev_list_entry { std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn udev_enumerate_new(_: *mut udev) -> *mut udev_enumerate { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_unref(_: *mut udev_enumerate) -> *mut udev_enumerate { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_ref(_: *mut udev_enumerate) -> *mut udev_enumerate { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_subsystem(_: *mut udev_enumerate, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_is_initialized(_: *mut udev_enumerate) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_sysattr(_: *mut udev_enumerate, _: *const c_char, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_sysname(_: *mut udev_enumerate, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_property(_: *mut udev_enumerate, _: *const c_char, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_tag(_: *mut udev_enumerate, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_match_parent(_: *mut udev_enumerate, _: *mut udev_device) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_nomatch_subsystem(_: *mut udev_enumerate, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_nomatch_sysattr(_: *mut udev_enumerate, _: *const c_char, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_add_syspath(_: *mut udev_enumerate, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_scan_devices(_: *mut udev_enumerate) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_scan_subsystems(_: *mut udev_enumerate) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_enumerate_get_list_entry(_: *mut udev_enumerate) -> *mut udev_list_entry { std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn udev_list_entry_get_next(_: *mut udev_list_entry) -> *mut udev_list_entry { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_list_entry_get_by_name(_: *mut udev_list_entry, _: *const c_char) -> *mut udev_list_entry { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_list_entry_get_name(_: *mut udev_list_entry) -> *const c_char { std::ptr::null() }
#[no_mangle] pub unsafe extern "C" fn udev_list_entry_get_value(_: *mut udev_list_entry) -> *const c_char { std::ptr::null() }

#[no_mangle] pub unsafe extern "C" fn udev_monitor_new_from_netlink(_: *mut udev, _: *const c_char) -> *mut udev_monitor { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_unref(_: *mut udev_monitor) -> *mut udev_monitor { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_ref(_: *mut udev_monitor) -> *mut udev_monitor { std::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_filter_add_match_subsystem_devtype(_: *mut udev_monitor, _: *const c_char, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_filter_add_match_tag(_: *mut udev_monitor, _: *const c_char) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_filter_remove(_: *mut udev_monitor) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_enable_receiving(_: *mut udev_monitor) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_set_receive_buffer_size(_: *mut udev_monitor, _: c_int) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_get_fd(_: *mut udev_monitor) -> c_int { -1 }
#[no_mangle] pub unsafe extern "C" fn udev_monitor_receive_device(_: *mut udev_monitor) -> *mut udev_device { std::ptr::null_mut() }
