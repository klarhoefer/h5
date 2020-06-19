
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_uint, c_int, c_ulonglong, c_longlong, c_void};


#[cfg(hid_t_64)]
pub type hid_t = i64;

#[cfg(not(hid_t_64))]
pub type hid_t = c_int;


pub type herr_t = c_int;
pub type htri_t = c_int;
pub type hsize_t = c_ulonglong;
pub type hssize_t = c_longlong;

pub type size_t = usize;


pub const H5P_DEFAULT: hid_t = 0;
pub const H5I_INVALID_HID: hid_t = -1;

pub const H5F_ACC_RDONLY: c_uint = 0x0000;
pub const H5F_ACC_RDWR: c_uint = 0x0001;
pub const H5F_ACC_TRUNC: c_uint = 0x0002;
pub const H5F_ACC_EXCL: c_uint = 0x0004;
pub const H5F_ACC_CREAT: c_uint = 0x0010;


#[repr(C)]
pub enum H5S_class_t {
    H5S_NO_CLASS = -1,
    H5S_SCALAR = 0,
    H5S_SIMPLE = 1,
    H5S_NULL = 2,
}


extern {
    pub static __imp_H5T_C_S1_g: *const hid_t;

    pub fn H5Fcreate(filename: *const c_char, flags: c_uint, create_plist: hid_t, access_plist: hid_t) -> hid_t;
    pub fn H5Fopen(filename: *const c_char, flags: c_uint, access_plist: hid_t) -> hid_t;
    pub fn H5Fclose(file_id: hid_t) -> herr_t;

    pub fn H5Gcreate2(loc_id: hid_t, name: *const c_char, lcpl_id: hid_t, gcpl_id: hid_t, gapl_id: hid_t) -> hid_t;
    pub fn H5Gopen2(loc_id: hid_t, name: *const c_char, gapl_id: hid_t) -> hid_t;
    pub fn H5Gclose(group_id: hid_t) -> herr_t;


    pub fn H5Screate(type_: H5S_class_t) -> hid_t;
    pub fn H5Screate_simple(rank: c_int, dims: *const hsize_t, maxdims: *const hsize_t) -> hid_t;
    pub fn H5Sclose(space_id: hid_t) -> herr_t;


    pub fn H5Tcopy(type_id: hid_t) -> hid_t;
    pub fn H5Tclose(type_id: hid_t) -> herr_t;
    pub fn H5Tset_size(type_id: hid_t, size: size_t) -> herr_t;
    pub fn H5Tget_size(type_id: hid_t) -> size_t;

    pub fn H5Acreate2(loc_id: hid_t, attr_name: *const c_char, type_id: hid_t, space_id: hid_t, acpl_id: hid_t, aapl_id: hid_t) -> hid_t;
    pub fn H5Aopen(obj_id: hid_t, attr_name: *const c_char, aapl_id: hid_t) -> hid_t;
    pub fn H5Aclose(attr_id: hid_t) -> herr_t;
    pub fn H5Awrite(attr_id: hid_t, type_id: hid_t, buf: *const c_void) -> herr_t;
    pub fn H5Aread(attr_id: hid_t, type_id: hid_t, buf: *mut c_void) -> herr_t;
    // pub fn H5Aget_space(attr_id: hid_t) -> hid_t;
    pub fn H5Aget_type(attr_id: hid_t) -> hid_t;


}

// pub static H5T_C_S1: *const hid_t = unsafe { __imp_H5T_C_S1_g };
