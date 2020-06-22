
use std::ffi::CString;
use std::path::Path;
use std::str;

mod datetime;
use datetime::DateTime;

mod chan;
use chan::H5Chan;

mod param;
use param::Parameters;

mod hdf5;
use hdf5::*;

macro_rules! cc {
    ($txt:ident) => { $txt.as_ptr() as *const _};
}

pub enum OpenMode {
    Read, Write, ReadWrite
}

pub struct H5File {
    fid: hid_t,
}

static NAME_CHANS: &[u8] = b"/Channels\0";
static NAME_CONFIG: &[u8] = b"/Configuration\0";
static NAME_TIMESTAMP: &[u8] = b"Timestamp\0";
static NAME_PARAMETERS: &[u8] = b"/Configuration/Parameters\0";

#[derive(Debug)]
pub enum H5Error {
    NotCreated,
    NotFound,
    InvalidPath,
}

impl H5File {
    pub fn open<P: AsRef<Path>>(path: P, mode: OpenMode) -> Result<Self, H5Error> {
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
                    return Ok(H5File { fid });
                } else {
                    let err = match mode {
                        OpenMode::Write => H5Error::NotCreated,
                        _ => H5Error::NotFound,
                    };
                    return Err(err);
                }
            }
        }
        Err(H5Error::InvalidPath)
    }

    pub fn init(&self) {
        unsafe {
            // let config = H5Gcreate2(self.fid, b"/Configuration\0".as_ptr() as *const _, H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
            let config = H5Gcreate2(self.fid, cc!(NAME_CONFIG), H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
            if config != H5I_INVALID_HID {
                H5Gclose(config);
            }
            let chans = H5Gcreate2(self.fid, cc!(NAME_CHANS), H5P_DEFAULT, H5P_DEFAULT, H5P_DEFAULT);
            if chans != H5I_INVALID_HID {
                H5Gclose(chans);
            }
        }
    }

    pub fn set_timestamp(&self, ts: DateTime) {
        let sts = ts.to_string();
        unsafe {
            let sid = H5Screate(H5S_class_t::H5S_SCALAR);
            let tid = H5Tcopy(*__imp_H5T_C_S1_g);
            H5Tset_size(tid, sts.len());
            let aid = H5Acreate2(self.fid, cc!(NAME_TIMESTAMP), tid, sid, H5P_DEFAULT, H5P_DEFAULT);
            H5Awrite(aid, tid, sts.as_ptr() as *const _);
            H5Aclose(aid);
            H5Tclose(tid);
            H5Sclose(sid);
        }
    }

    pub fn get_timestamp(&self) -> Option<DateTime> {
        let mut buffer = [0u8; 24];
        unsafe {
            if H5Aexists(self.fid, cc!(NAME_TIMESTAMP)) > 0 {
                let aid = H5Aopen(self.fid, cc!(NAME_TIMESTAMP), H5P_DEFAULT);
                let tid = H5Aget_type(aid);
                let size = H5Tget_size(tid);
                H5Aread(aid, tid, buffer.as_mut_ptr() as *mut _);
                H5Tclose(tid);
                H5Aclose(aid);
    
                if let Ok(s) = str::from_utf8(&buffer[..size]) {
                    if let Ok(dt) = DateTime::parse(s) {
                        return Some(dt);
                    }
                }
            }
        }
        None
    }

    pub fn set_params(&self, params: Parameters) {
        params.save(self.fid, NAME_PARAMETERS.as_ptr());
    }

    pub fn get_params(&self) -> Option<Parameters> {
        Parameters::load(self.fid, NAME_PARAMETERS.as_ptr())
    }

    pub fn create_channel(&self, number: u8, sample_rate: u16) -> H5Chan {
        H5Chan::create(self.fid, number, sample_rate)
    }

    pub fn open_channel(&self, number: u8) -> Option<H5Chan> {
        H5Chan::open(self.fid, number)
    }
}

impl Drop for H5File {
    fn drop(&mut self) {
        unsafe {
            H5Fclose(self.fid);
        }
    }
}

// cargo test -- --nocapture
// cargo test -- --test-threads=1

#[cfg(test)]
mod tests {

    use super::{H5File, OpenMode, DateTime, Parameters};

    #[test]
    fn open_close() {
        {
            let f = H5File::open("test.h5", OpenMode::Write).unwrap();
            f.init();
        }
        {
            let _ = H5File::open("test.h5", OpenMode::Read).unwrap();
        }
    }

    #[test]
    fn read_write_timestamp() {
        {
            let f = H5File::open("test_ts.h5", OpenMode::Write).unwrap();
            f.set_timestamp(DateTime::new(2020, 6, 19, 7, 17, 0));
        }
        {
            let f = H5File::open("test_ts.h5", OpenMode::Read).unwrap();
            let _ = f.get_timestamp().unwrap();
        }
    }

    #[test]
    fn read_write_params() {
        {
            let f = H5File::open("test_params.h5", OpenMode::Write).unwrap();
            f.init();
            let mut params = Parameters::new();
            params.add("PumpsOn", 5000.0);
            params.add("PumpsOff", 4000.0);
            f.set_params(params);
        }
        {
            let f = H5File::open("test_params.h5", OpenMode::Read).unwrap();
            let mut params = f.get_params().unwrap();
            params.map(|name, val| {
                println!("Name: >{}<, Value: {}", name, val);
                val + 1.0
            });
        }
    }

    #[test]
    fn read_write_chan() {
        {
            let f = H5File::open("test_chan.h5", OpenMode::Write).unwrap();
            f.init();
            let _ = f.create_channel(0, 256);
        }
        {
            let f = H5File::open("test_chan.h5", OpenMode::Read).unwrap();
            let c = f.open_channel(0);
            println!("{:?}", c);
        }
    }
}
