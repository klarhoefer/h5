
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_uint, c_int, c_ulonglong, c_longlong};


#[cfg(hid_t_64)]
pub type hid_t = i64;

#[cfg(not(hid_t_64))]
pub type hid_t = c_int;


pub type herr_t = c_int;
pub type htri_t = c_int;
pub type hsize_t = c_ulonglong;
pub type hssize_t = c_longlong;


pub const H5P_DEFAULT: hid_t = 0;
pub const H5I_INVALID_HID: hid_t = -1;

pub const H5F_ACC_RDONLY: c_uint = 0x0000;
pub const H5F_ACC_RDWR: c_uint = 0x0001;
pub const H5F_ACC_TRUNC: c_uint = 0x0002;
pub const H5F_ACC_EXCL: c_uint = 0x0004;
pub const H5F_ACC_CREAT: c_uint = 0x0010;


extern {
    pub fn H5Fcreate(name: *const c_char, flags: c_uint, fcpl_id: hid_t, fapl_id: hid_t) -> hid_t;
    pub fn H5Fopen(name: *const c_char, flags: c_uint, fapl_id: hid_t) -> hid_t;
    pub fn H5Fclose(file_id: hid_t) -> herr_t;
}
