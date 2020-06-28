
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
pub const H5S_ALL: hid_t = 0;

pub const H5S_UNLIMITED: hsize_t = (-1 as hssize_t) as _;


pub const H5F_ACC_RDONLY: c_uint = 0x0000;
pub const H5F_ACC_RDWR: c_uint = 0x0001;
pub const H5F_ACC_TRUNC: c_uint = 0x0002;
pub const H5F_ACC_EXCL: c_uint = 0x0004;
pub const H5F_ACC_CREAT: c_uint = 0x0010;


#[repr(C)]
pub enum H5S_seloper_t {
    H5S_SELECT_NOOP = -1,
    H5S_SELECT_SET = 0,
    H5S_SELECT_OR = 1,
    H5S_SELECT_AND = 2,
    H5S_SELECT_XOR = 3,
    H5S_SELECT_NOTB = 4,
    H5S_SELECT_NOTA = 5,
    H5S_SELECT_APPEND = 6,
    H5S_SELECT_PREPEND = 7,
    H5S_SELECT_INVALID = 8,
}

#[repr(C)]
pub enum H5S_class_t {
    H5S_NO_CLASS = -1,
    H5S_SCALAR = 0,
    H5S_SIMPLE = 1,
    H5S_NULL = 2,
}

#[repr(C)]
pub enum H5T_class_t {
    H5T_NO_CLASS = -1,
    H5T_INTEGER = 0,
    H5T_FLOAT = 1,
    H5T_TIME = 2,
    H5T_STRING = 3,
    H5T_BITFIELD = 4,
    H5T_OPAQUE = 5,
    H5T_COMPOUND = 6,
    H5T_REFERENCE = 7,
    H5T_ENUM = 8,
    H5T_VLEN = 9,
    H5T_ARRAY = 10,
    H5T_NCLASSES = 11,
}


extern {
    pub static __imp_H5T_C_S1_g: *const hid_t;
    pub static __imp_H5T_NATIVE_DOUBLE_g: *const hid_t;
    pub static __imp_H5T_NATIVE_FLOAT_g: *const hid_t;
    pub static __imp_H5T_NATIVE_INT_g: *const hid_t;

    #[cfg(hid_t_64)]
    pub static __imp_H5P_CLS_DATASET_CREATE_g: *const hid_t;
    
    #[cfg(not(hid_t_64))]
    pub static __imp_H5P_CLS_DATASET_CREATE_ID_g: *const hid_t;


    pub fn H5Fcreate(filename: *const c_char, flags: c_uint, create_plist: hid_t, access_plist: hid_t) -> hid_t;
    pub fn H5Fopen(filename: *const c_char, flags: c_uint, access_plist: hid_t) -> hid_t;
    pub fn H5Fclose(file_id: hid_t) -> herr_t;

    pub fn H5Gcreate2(loc_id: hid_t, name: *const c_char, lcpl_id: hid_t, gcpl_id: hid_t, gapl_id: hid_t) -> hid_t;
    pub fn H5Gopen2(loc_id: hid_t, name: *const c_char, gapl_id: hid_t) -> hid_t;
    pub fn H5Gclose(group_id: hid_t) -> herr_t;

    pub fn H5Dcreate2(loc_id: hid_t, name: *const c_char, type_id: hid_t, space_id: hid_t, lcpl_id: hid_t, dcpl_id: hid_t, dapl_id: hid_t) -> hid_t;
    pub fn H5Dopen2(file_id: hid_t, name: *const c_char, dapl_id: hid_t) -> hid_t;
    pub fn H5Dclose(dset_id: hid_t) -> herr_t;
    pub fn H5Dget_space(dset_id: hid_t) -> hid_t;
    pub fn H5Dget_type(dset_id: hid_t) -> hid_t;
    pub fn H5Dread(dset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t, file_space_id: hid_t, plist_id: hid_t, buf: *mut c_void) -> herr_t;
    pub fn H5Dwrite(dset_id: hid_t, mem_type_id: hid_t, mem_space_id: hid_t, file_space_id: hid_t, plist_id: hid_t, buf: *const c_void) -> herr_t;


    pub fn H5Screate(type_: H5S_class_t) -> hid_t;
    pub fn H5Screate_simple(rank: c_int, dims: *const hsize_t, maxdims: *const hsize_t) -> hid_t;
    pub fn H5Sclose(space_id: hid_t) -> herr_t;
    pub fn H5Sget_select_npoints(spaceid: hid_t) -> hssize_t;
    pub fn H5Sget_simple_extent_dims(space_id: hid_t, dims: *mut hsize_t, maxdims: *mut hsize_t) -> c_int;
    pub fn H5Sset_extent_simple(space_id: hid_t, rank: c_int, dims: *const hsize_t, max: *const hsize_t) -> herr_t;
    pub fn H5Sselect_hyperslab(space_id: hid_t, op: H5S_seloper_t, start: *const hsize_t, _stride: *const hsize_t, count: *const hsize_t, _block: *const hsize_t) -> herr_t;


    pub fn H5Tcreate(type_: H5T_class_t, size: size_t) -> hid_t;
    pub fn H5Tcopy(type_id: hid_t) -> hid_t;
    pub fn H5Tclose(type_id: hid_t) -> herr_t;
    pub fn H5Tset_size(type_id: hid_t, size: size_t) -> herr_t;
    pub fn H5Tget_size(type_id: hid_t) -> size_t;
    pub fn H5Tinsert(parent_id: hid_t, name: *const c_char, offset: size_t, member_id: hid_t) -> herr_t;

    pub fn H5Acreate2(loc_id: hid_t, attr_name: *const c_char, type_id: hid_t, space_id: hid_t, acpl_id: hid_t, aapl_id: hid_t) -> hid_t;
    pub fn H5Aopen(obj_id: hid_t, attr_name: *const c_char, aapl_id: hid_t) -> hid_t;
    pub fn H5Aclose(attr_id: hid_t) -> herr_t;
    pub fn H5Awrite(attr_id: hid_t, type_id: hid_t, buf: *const c_void) -> herr_t;
    pub fn H5Aread(attr_id: hid_t, type_id: hid_t, buf: *mut c_void) -> herr_t;
    // pub fn H5Aget_space(attr_id: hid_t) -> hid_t;
    pub fn H5Aget_type(attr_id: hid_t) -> hid_t;
    pub fn H5Aexists(obj_id: hid_t, attr_name: *const c_char) -> htri_t;

    pub fn H5Pcreate(cls_id: hid_t) -> hid_t;
    pub fn H5Pclose(plist_id: hid_t) -> herr_t;
    pub fn H5Pset_chunk(plist_id: hid_t, ndims: c_int, dim: *const hsize_t) -> herr_t;
    pub fn H5Pset_deflate(plist_id: hid_t, aggression: c_uint) -> herr_t;


    pub fn H5Lexists(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> htri_t;

}

pub static mut H5T_C_S1: hid_t = 0;
pub static mut H5T_NATIVE_DOUBLE: hid_t = 0;
pub static mut H5T_NATIVE_FLOAT: hid_t = 0;
pub static mut H5T_NATIVE_INT: hid_t = 0;

pub static mut H5P_CLS_DATASET_CREATE: hid_t = 0;

pub static mut IS_INITIALIZED: bool = false;

pub const NULL: *const hsize_t = 0 as *const hsize_t;

pub fn init() {
    unsafe {
        H5T_C_S1 = *__imp_H5T_C_S1_g;
        H5T_NATIVE_DOUBLE = *__imp_H5T_NATIVE_DOUBLE_g;
        H5T_NATIVE_FLOAT = *__imp_H5T_NATIVE_FLOAT_g;
        H5T_NATIVE_INT = *__imp_H5T_NATIVE_INT_g;
        
        IS_INITIALIZED = true;
    }
    
    #[cfg(hid_t_64)]
    unsafe {
        H5P_CLS_DATASET_CREATE = *__imp_H5P_CLS_DATASET_CREATE_g;
    }

    #[cfg(not(hid_t_64))]
    unsafe {
        H5P_CLS_DATASET_CREATE = *__imp_H5P_CLS_DATASET_CREATE_ID_g;
    }
}
