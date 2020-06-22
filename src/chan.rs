
#![allow(dead_code)]

use crate::hdf5::*;

#[derive(Debug)]
pub struct H5Chan {
    dsid: hid_t,
    sample_rate: u16,
    chan_no: u8,
}

impl H5Chan {
    pub fn create(loc: hid_t, number: u8, sample_rate: u16) -> Self {
        let path = format!("/Channels/Channel {}\0", number);
        let dims = [0 as hsize_t];
        let max_dims = [H5S_UNLIMITED];
        let chunk_dims = [256 as hsize_t];
        unsafe {
            let sid = H5Screate_simple(1, dims.as_ptr(), max_dims.as_ptr());
            let pid = H5Pcreate(*__imp_H5P_CLS_DATASET_CREATE_ID_g);
            H5Pset_chunk(pid, 1, chunk_dims.as_ptr());
            H5Pset_deflate(pid, 6);
            let dsid = H5Dcreate2(loc, path.as_str().as_bytes().as_ptr() as *const _, *__imp_H5T_NATIVE_FLOAT_g, sid,
                H5P_DEFAULT, pid, H5P_DEFAULT);
            let asid = H5Screate(H5S_class_t::H5S_SCALAR);
            let aid = H5Acreate2(dsid, b"Sample Rate\0".as_ptr() as *const _, *__imp_H5T_NATIVE_INT_g, asid, H5P_DEFAULT, H5P_DEFAULT);
            let sr = sample_rate as i32;
            H5Awrite(aid,*__imp_H5T_NATIVE_INT_g, &sr as *const _ as *const _);
            H5Aclose(aid);
            H5Sclose(asid);
            H5Pclose(pid);
            H5Sclose(sid);
            H5Chan { dsid, sample_rate, chan_no: number }
        }
    }

    pub fn open(loc: hid_t, number: u8) -> Option<Self> {
        let path = format!("/Channels/Channel {}\0", number);
        let name = path.as_str().as_bytes().as_ptr() as *const _;
        unsafe {
            if H5Lexists(loc, name, H5P_DEFAULT) > 0 {
                let dsid = H5Dopen2(loc, name, H5P_DEFAULT);
                let aid = H5Aopen(dsid, b"Sample Rate\0".as_ptr() as *const _, H5P_DEFAULT);
                let mut sr: i32 = 0;
                H5Aread(aid, *__imp_H5T_NATIVE_INT_g, &mut sr as *mut _ as *mut _);
                H5Aclose(aid);
                Some(H5Chan {dsid, sample_rate: 0, chan_no: number })
            } else {
                None
            }
        }
    }
}

impl Drop for H5Chan {
    fn drop(&mut self) {
        unsafe {
            H5Dclose(self.dsid);
        }
    }
}
