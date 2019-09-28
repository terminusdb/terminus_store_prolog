use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

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
pub static STORE_SIZE: usize =
    std::mem::size_of::<SyncStore<DirectoryLabelStore, DirectoryLayerStore>>();

#[no_mangle]
pub static DB_SIZE: usize =
    std::mem::size_of::<SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>>();

#[no_mangle]
pub static LAYER_BUILDER_SIZE: usize =
    std::mem::size_of::<SyncDatabaseLayerBuilder<DirectoryLayerStore>>();

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
pub extern "C" fn create_database(
    name: *const c_char,
    store_ptr: *mut c_void,
    err: *const *const c_char,
) -> *const SyncDatabase<DirectoryLabelStore, DirectoryLayerStore> {
    let store = store_ptr as *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>;
    let store_box = unsafe { Box::from_raw(store) };
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = unsafe { CStr::from_ptr(name) };
    let db_name = db_name_cstr.to_str().unwrap();
    // Safe because we expect the swipl pointers to be decent
    let database = store_box.create(db_name).unwrap();
    Box::into_raw(Box::new(database))
}

#[no_mangle]
pub extern "C" fn cleanup_directory_store(store_ptr: *mut c_void) {
    let store = store_ptr as *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>;
    unsafe { Box::from_raw(store) };
}

#[no_mangle]
pub extern "C" fn cleanup_db(db_ptr: *mut c_void) {
    let db = db_ptr as *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>;
    unsafe { Box::from_raw(db) };
}

#[no_mangle]
pub extern "C" fn cleanup_layer_builder(layer_builder_ptr: *mut c_void) {
    let builder = layer_builder_ptr as *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>;
    unsafe { Box::from_raw(builder) };
}
