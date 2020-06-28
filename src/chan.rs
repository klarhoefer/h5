
#![allow(dead_code)]

use crate::hdf5::*;

#[derive(Debug)]
pub struct H5Chan {
    dsid: hid_t,
    sample_rate: u16,
    chan_no: u8,
    count: usize,
}

impl H5Chan {
    pub fn create(loc: hid_t, number: u8, sample_rate: u16) -> Self {
        let path = format!("/Channels/Channel {}\0", number);
        let dims = [0 as hsize_t];
        let max_dims = [H5S_UNLIMITED];
        let chunk_dims = [256 as hsize_t];
        unsafe {
            let sid = H5Screate_simple(1, dims.as_ptr(), max_dims.as_ptr());
            let pid = H5Pcreate(H5P_CLS_DATASET_CREATE);
            H5Pset_chunk(pid, 1, chunk_dims.as_ptr());
            H5Pset_deflate(pid, 6);
            let dsid = H5Dcreate2(loc, path.as_str().as_bytes().as_ptr() as *const _, H5T_NATIVE_FLOAT, sid,
                H5P_DEFAULT, pid, H5P_DEFAULT);
            let asid = H5Screate(H5S_class_t::H5S_SCALAR);
            let aid = H5Acreate2(dsid, b"Sample Rate\0".as_ptr() as *const _, H5T_NATIVE_INT, asid, H5P_DEFAULT, H5P_DEFAULT);
            let sr = sample_rate as i32;
            H5Awrite(aid, H5T_NATIVE_INT, &sr as *const _ as *const _);
            H5Aclose(aid);
            H5Sclose(asid);
            H5Pclose(pid);
            H5Sclose(sid);
            H5Chan { dsid, sample_rate, chan_no: number, count: 0 }
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
                H5Aread(aid, H5T_NATIVE_INT, &mut sr as *mut _ as *mut _);
                H5Aclose(aid);
                let sid = H5Dget_space(dsid);
                let count = H5Sget_select_npoints(sid) as usize;
                H5Sclose(sid);
                Some(H5Chan {dsid, sample_rate: sr as u16, chan_no: number, count })
            } else {
                None
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn append(&mut self, samples: &[f32]) {
        let mut dims = [0 as hsize_t];
        let mut max_dims = [0 as hsize_t];
        unsafe {
            let sid = H5Dget_space(self.dsid);
            H5Sget_simple_extent_dims(sid, dims.as_mut_ptr(), max_dims.as_mut_ptr());
            H5Sclose(sid);
            let start = [dims[0]];
            let count = [samples.len() as hsize_t];
            dims[0] += samples.len() as hsize_t;
            H5Dset_extent(self.dsid, dims.as_ptr());
            let sid = H5Dget_space(self.dsid);
            H5Sselect_hyperslab(sid, H5S_seloper_t::H5S_SELECT_SET, start.as_ptr(), NULL, count.as_ptr(), NULL);
            let tid = H5Dget_type(self.dsid);
            let mem_sid = H5Screate_simple(1, count.as_ptr(), NULL);
            H5Dwrite(self.dsid, tid, mem_sid, sid, H5P_DEFAULT, samples.as_ptr() as *const _);
            H5Sclose(mem_sid);
            H5Tclose(tid);
            H5Sclose(sid);
        }
        self.count += samples.len();
    }

    pub fn read(&self, offset: usize, samples: &mut[f32]) {
        unsafe {
            let start = [offset as hsize_t];
            let count = [samples.len() as hsize_t];
            let sid = H5Dget_space(self.dsid);
            let tid = H5Dget_type(self.dsid);
            H5Sselect_hyperslab(sid, H5S_seloper_t::H5S_SELECT_SET, start.as_ptr(), NULL, count.as_ptr(), NULL);
            let mem_sid = H5Screate_simple(1, count.as_ptr(), NULL);
            H5Dread(self.dsid, tid, mem_sid, sid, H5P_DEFAULT, samples.as_mut_ptr() as *mut _);
            H5Sclose(mem_sid);
            H5Tclose(tid);
            H5Sclose(sid);
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
