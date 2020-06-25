
#![allow(dead_code)]

use std::str;

use crate::hdf5::*;

#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
struct Param {
    name: [u8; 16],
    value: f64,
}

fn copy_to_bytes(src: &str, dst: &mut[u8]) {
    let bytes = src.as_bytes();
    let len = dst.len();
    for (i, &b) in bytes.iter().enumerate() {
        if i == len {
            break;
        }
        dst[i] = b;
    }
}

impl Param {
    fn new(name: &str, value: f64) -> Self {
        let mut buffer = [0u8; 16];
        copy_to_bytes(name, &mut buffer);
        Param { name: buffer, value }
    }

    fn has_name(&self, name: &[u8]) -> bool {
        for (i, &b) in self.name.iter().enumerate() {
            if b != name[i] {
                return false;
            }
        }
        true
    }
}


pub struct Parameters {
    params: Vec<Param>
}

impl Parameters {
    pub fn new() -> Self {
        Parameters { params: Vec::new() }
    }

    pub fn add(&mut self, name: &str, value: f64) {
        self.params.push(Param::new(name, value));
    }

    pub fn save(&self, loc: hid_t, name: *const u8) {
        let dims = [self.params.len() as hsize_t];
        unsafe {
            let s16id = H5Tcopy(H5T_C_S1);
            H5Tset_size(s16id, 16);
            let tid = H5Tcreate(H5T_class_t::H5T_COMPOUND, 24);
            H5Tinsert(tid, b"Name\0".as_ptr() as *const _, 0, s16id);
            H5Tinsert(tid, b"Value\0".as_ptr() as *const _, 16, H5T_NATIVE_DOUBLE);
            let sid = H5Screate_simple(1, dims.as_ptr(), 0 as *const hsize_t);
            let dsid = H5Dcreate2(loc, name as *const _, tid, sid, H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
            H5Dwrite(dsid, tid, H5S_ALL, H5S_ALL, H5P_DEFAULT, self.params.as_ptr() as *const _);
            H5Dclose(dsid);
            H5Sclose(sid);
            H5Tclose(tid);
            H5Tclose(s16id);
        }
    }

    pub fn load(loc: hid_t, name: *const u8) -> Option<Self> {
        unsafe {
            let dsid = H5Dopen2(loc, name as *const _, H5P_DEFAULT);
            let sid = H5Dget_space(dsid);
            let n = H5Sget_select_npoints(sid);
            let tid = H5Dget_type(dsid);
            let mut params = vec![Param::default(); n as usize];
            H5Dread(dsid, tid, H5S_ALL, H5S_ALL, H5P_DEFAULT, params.as_mut_ptr() as *mut _);
            H5Tclose(tid);
            H5Sclose(sid);
            H5Dclose(dsid);
            Some(Parameters { params })
        }
    }

    pub fn map(&mut self, f: impl Fn(&str, f64)->f64) {
        for param in &mut self.params {
            let mut pos = 0;
            while pos < param.name.len() && param.name[pos] != 0 {
                pos += 1;
            }
            if let Ok(name) = str::from_utf8(&param.name[..pos]) {
                let val = f(name, param.value);
                param.value = val;
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        let mut buffer = [0u8; 16];
        copy_to_bytes(name, &mut buffer);
        for param in &self.params {
            if param.has_name(&buffer) {
                return Some(param.value);
            }
        }
        None
    }
}
