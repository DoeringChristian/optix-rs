use cuda_rs::*;
use std::env;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

use crate::*;

#[cfg(target_os = "windows")]
fn find_cuda_lib_dirs() -> PathBuf {
    todo!()
}
#[cfg(not(target_os = "windows"))]
fn find_optix_lib_dirs() -> PathBuf {
    let globs = ["/usr/lib64/libnvoptix.so.1"];

    let paths = globs
        .iter()
        .map(|pat| glob::glob(pat).unwrap().map(|g| g.unwrap()))
        .flatten()
        .collect::<Vec<_>>();

    if paths.len() == 1 {
        paths[0].clone()
    } else if paths.len() > 1 {
        paths[0].clone()
    } else {
        panic!("libcuda.so not found!")
    }
}

const OPTIX_ABI_VERSION: i32 = 55;
impl OptixApi {
    pub unsafe fn find_and_load() -> Result<Self, OptixError> {
        let path = find_optix_lib_dirs();
        Self::new(path)
    }
}
