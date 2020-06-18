
fn main() {
    println!("cargo:rustc-link-lib={}", "hdf5");
    println!("cargo:rustc-link-search={}", r"C:\Program Files\HDF_Group\HDF5\1.8.17\lib");
    // println!("cargo:rustc-cfg={}", "hid_t_64");
}
