use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::error::Error;
use std::io;

use terminus_store::layer::{
    IdTriple, Layer, ObjectType, PredicateObjectPairsForSubject, StringTriple,
};
use terminus_store::storage::*;
use terminus_store::storage::{
    DirectoryLabelStore, DirectoryLayerStore, LabelStore, LayerStore, MemoryLabelStore,
    MemoryLayerStore,
};
use terminus_store::sync::store::*;

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

fn error_to_cstring(error: io::Error) -> CString {
    CString::new(format!("{}", error)).unwrap()
}

#[no_mangle]
pub extern "C" fn create_database(
    name: *const c_char,
    store_ptr: *mut c_void,
    err: *mut *const c_char,
) -> *const SyncDatabase<DirectoryLabelStore, DirectoryLayerStore> {
    let store = store_ptr as *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>;
    let store_box = unsafe { Box::from_raw(store) };
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = unsafe { CStr::from_ptr(name) };
    let db_name = db_name_cstr.to_str().unwrap();

    let result = store_box.create(db_name);
    std::mem::forget(store_box);
    // Safe because we expect the swipl pointers to be decent
    match result {
        Ok(database) => {
            Box::into_raw(Box::new(database))
        }
        Err(e) => {
            unsafe {
                *err = error_to_cstring(e).into_raw();
            }
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_directory_store(store: *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>) {
    unsafe { Box::from_raw(store) };
}

#[no_mangle]
pub extern "C" fn cleanup_db(db: *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>) {
    unsafe { Box::from_raw(db) };
}

#[no_mangle]
pub extern "C" fn cleanup_layer_builder(layer_builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>) {
    unsafe { Box::from_raw(layer_builder) };
}

#[no_mangle]
pub extern "C" fn cleanup_cstring(cstring_ptr: *mut c_char) {
    unsafe {
        CString::from_raw(cstring_ptr);
    }
}
