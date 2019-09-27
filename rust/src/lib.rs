use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

use terminus_store::storage::*;
use terminus_store::sync::store::*;

#[no_mangle]
pub static STORE_SIZE: usize =
    std::mem::size_of::<SyncStore<DirectoryLabelStore, DirectoryLayerStore>>();

#[no_mangle]
pub extern "C" fn open_directory_store(
    dir: *const c_char,
) -> *const SyncStore<DirectoryLabelStore, DirectoryLayerStore> {
    // Safe because swipl will always return a null-terminated string
    let dir_name_cstr = unsafe { CStr::from_ptr(dir) };
    let dir_name = dir_name_cstr.to_str().unwrap();
    let store = open_sync_directory_store(dir_name);
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub extern "C" fn cleanup_store(store_ptr: *mut c_void) {
    let store = store_ptr as *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>;
    unsafe { Box::from_raw(store) };
}
