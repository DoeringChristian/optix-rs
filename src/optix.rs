use std::ffi::c_void;
use std::path::Path;

use cuda_rs::*;

use crate::OptixError;
include!("autogen_optix.rs");

const OPTIX_ABI_VERSION: i32 = 55;

pub struct OptixApi {
    lib: libloading::Library,
    ftable: OptixFunctionTable,
}

impl OptixApi {
    pub unsafe fn new<P>(path: P) -> Result<Self, OptixError>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let lib = libloading::Library::new(path).unwrap();
        unsafe {
            let mut query_table: libloading::Symbol<OptixQueryFunctionTable_t> =
                lib.get(b"optixQueryFunctionTable").unwrap();
            let mut ftable = OptixFunctionTable::default();
            query_table.unwrap()(
                OPTIX_ABI_VERSION,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut ftable as *mut _ as *mut c_void,
                std::mem::size_of::<OptixFunctionTable>(),
            )
            .check()?;
        }

        Ok(Self {
            lib,
            ftable: todo!(),
        })
    }
}
