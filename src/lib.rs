
use std::ffi::CString;
use std::path::Path;

mod hdf5;
use hdf5::*;

pub enum OpenMode {
    Read, Write, ReadWrite
}

pub struct H5File {
    fid: hid_t,
}

impl H5File {
    pub fn open<P: AsRef<Path>>(path: P, mode: OpenMode) -> Option<Self> {
        if let Some(path) = path.as_ref().to_str() {
            if let Ok(path) = CString::new(path) {
                let name = path.as_ptr();
                let fid = unsafe {
                    match mode {
                        OpenMode::Read => H5Fopen(name, H5F_ACC_RDONLY, H5P_DEFAULT),
                        OpenMode::Write => H5Fcreate(name, H5F_ACC_TRUNC, H5P_DEFAULT, H5P_DEFAULT),
                        OpenMode::ReadWrite => H5Fopen(name, H5F_ACC_RDWR, H5P_DEFAULT),
                    }
                };
                if fid != H5I_INVALID_HID {
                    return Some(H5File { fid });
                }
            }
        }
        None
    }
}

impl Drop for H5File {
    fn drop(&mut self) {
        unsafe {
            H5Fclose(self.fid);
        }
    }
}


#[cfg(test)]
mod tests {

    use super::{H5File, OpenMode};

    #[test]
    fn it_works() {
        {
            let _ = H5File::open("test.h5", OpenMode::Write);
        }
        {
            let _ = H5File::open("test.h5", OpenMode::Read);
        }
    }
}
