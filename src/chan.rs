
#![allow(dead_code)]

use crate::hdf5::*;

pub struct H5Chan {
    dsid: hid_t,
    sample_rate: u16,
    chan_no: u8,
}

impl H5Chan {
    pub fn create(loc: hid_t, number: u8, sample_rate: u16) -> Self {
        H5Chan { dsid: loc, sample_rate, chan_no: number }
    }
}
