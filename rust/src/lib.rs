use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::io;

use terminus_store::storage::{
    DirectoryLabelStore, DirectoryLayerStore,
};
use terminus_store::layer::{
    Layer, StringTriple, IdTriple, ObjectType
};
use terminus_store::sync::store::*;

#[no_mangle]
pub unsafe extern "C" fn open_directory_store(
    dir: *const c_char,
) -> *const SyncStore<DirectoryLabelStore, DirectoryLayerStore> {
    // Safe because swipl will always return a null-terminated string
    let dir_name_cstr = CStr::from_ptr(dir);
    let dir_name = dir_name_cstr.to_str().unwrap();
    let store = open_sync_directory_store(dir_name);
    Box::into_raw(Box::new(store))
}

fn error_to_cstring(error: io::Error) -> CString {
    CString::new(format!("{}", error)).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn create_database(
    store_ptr: *mut c_void,
    name: *const c_char,
    err: *mut *const c_char,
) -> *const SyncDatabase<DirectoryLabelStore, DirectoryLayerStore> {
    let store = store_ptr as *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>;
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = CStr::from_ptr(name);
    let db_name = db_name_cstr.to_str().unwrap();

    // Safe because we expect the swipl pointers to be decent
    match (*store).create(db_name) {
        Ok(database) => {
            Box::into_raw(Box::new(database))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_database(
    store: *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>,
    name: *const c_char,
    err: *mut *const c_char,
) -> *const SyncDatabase<DirectoryLabelStore, DirectoryLayerStore> {
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = CStr::from_ptr(name);
    let db_name = db_name_cstr.to_str().unwrap();

    // Safe because we expect the swipl pointers to be decent
    match (*store).open(db_name) {
        Ok(Some(database)) => {
            *err = std::ptr::null();
            Box::into_raw(Box::new(database))
        }
        Ok(None) => {
            *err = std::ptr::null();
            std::ptr::null()
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_get_head(database: *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>, err: *mut *const c_char) -> *const SyncDatabaseLayer<DirectoryLayerStore> {
    match (*database).head() {
        Ok(None) => {
            *err = std::ptr::null();
            std::ptr::null()
        },
        Ok(Some(layer)) => {
            *err = std::ptr::null();
            Box::into_raw(Box::new(layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_set_head(database: *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>, layer_ptr: *const SyncDatabaseLayer<DirectoryLayerStore>, err: *mut *const c_char) -> bool {
    match (*database).set_head(&*layer_ptr) {
        Ok(b) => {
            *err = std::ptr::null();
            b

        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn store_create_base_layer(store: *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>, err: *mut *const c_char) -> *const SyncDatabaseLayerBuilder<DirectoryLayerStore> {
    match (*store).create_base_layer() {
        Ok(builder) => {
            *err = std::ptr::null();
            Box::into_raw(Box::new(builder))
        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_open_write(layer: *mut SyncDatabaseLayer<DirectoryLayerStore>, err: *mut *const c_char) -> *const SyncDatabaseLayerBuilder<DirectoryLayerStore> {
    match (*layer).open_write() {
        Ok(builder) => {
            *err = std::ptr::null();
            Box::into_raw(Box::new(builder))
        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_open_write(database: *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>, err: *mut *const c_char) -> *const SyncDatabaseLayerBuilder<DirectoryLayerStore> {
    match (*database)
        .head()
        .and_then(|layer|
                  layer.map(|l|match l.open_write() {
                      Ok(builder) => Ok(Some(builder)),
                      Err(e) => Err(e)
                  }).unwrap_or(Ok(None))) {
            Ok(Some(builder)) => {
                *err = std::ptr::null();
                Box::into_raw(Box::new(builder))
            },
            Ok(None) => {
                *err = std::ptr::null();
                std::ptr::null()
            }
            Err(e) => {
                *err = error_to_cstring(e).into_raw();
                std::ptr::null()
            }
        }
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_id_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject: u64, predicate: u64, object: u64, err: *mut *const c_char) -> bool {
    match (*builder).add_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(r) => {
            *err = std::ptr::null();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_string_node_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject_ptr: *const c_char, predicate_ptr: *const c_char, object_ptr: *const c_char, err: *mut *const c_char) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null(),
        Err(e) => *err = error_to_cstring(e).into_raw()
    };
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_string_value_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject_ptr: *const c_char, predicate_ptr: *const c_char, object_ptr: *const c_char, err: *mut *const c_char) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null(),
        Err(e) => *err = error_to_cstring(e).into_raw()
    };
}


#[no_mangle]
pub unsafe extern "C" fn builder_remove_id_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject: u64, predicate: u64, object: u64, err: *mut *const c_char) -> bool {
    match (*builder).remove_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(r) => {
            *err = std::ptr::null();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_remove_string_node_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject_ptr: *const c_char, predicate_ptr: *const c_char, object_ptr: *const c_char, err: *mut *const c_char) -> bool {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(&StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(r) => {
            *err = std::ptr::null();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_remove_string_value_triple(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, subject_ptr: *const c_char, predicate_ptr: *const c_char, object_ptr: *const c_char, err: *mut *const c_char) -> bool {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(&StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(r) => {
            *err = std::ptr::null();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_commit(builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>, err: *mut *const c_char) -> *const SyncDatabaseLayer<DirectoryLayerStore> {
    match (*builder).commit() {
        Ok(layer) => {
            *err = std::ptr::null();
            Box::into_raw(Box::new(layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_node_and_value_count(layer: *const SyncDatabaseLayer<DirectoryLayerStore>) -> usize {
    (*layer).node_and_value_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_count(layer: *const SyncDatabaseLayer<DirectoryLayerStore>) -> usize {
    (*layer).predicate_count()
}


#[no_mangle]
pub unsafe extern "C" fn layer_subject_id(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, subject: *const c_char) -> u64 {
    let cstr = CStr::from_ptr(subject).to_string_lossy();
    (*layer).subject_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_id(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, predicate: *const c_char) -> u64 {
    let cstr = CStr::from_ptr(predicate).to_string_lossy();
    (*layer).predicate_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_node_id(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, object: *const c_char) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_node_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_value_id(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, object: *const c_char) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_value_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_subject(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, id: u64) -> *const c_char {
    (*layer).id_subject(id).map(|s|CString::new(s).unwrap().into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_predicate(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, id: u64) -> *const c_char {
    (*layer).id_predicate(id).map(|s|CString::new(s).unwrap().into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_object(layer: *const SyncDatabaseLayer<DirectoryLayerStore>, id: u64, object_type: *mut u8) -> *const c_char {
    (*layer).id_object(id).map(|x| match x {
        ObjectType::Node(s) => {
            *object_type = 0;
            s
        },
        ObjectType::Value(s) => {
            *object_type = 1;
            s
        }
    }).map(|s|CString::new(s).unwrap().into_raw() as *const c_char)
        .unwrap_or(std::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_directory_store(store: *mut SyncStore<DirectoryLabelStore, DirectoryLayerStore>) {
    Box::from_raw(store);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_db(db: *mut SyncDatabase<DirectoryLabelStore, DirectoryLayerStore>) {
    Box::from_raw(db);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer(layer: *mut SyncDatabaseLayer<DirectoryLayerStore>) {
    Box::from_raw(layer);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer_builder(layer_builder: *mut SyncDatabaseLayerBuilder<DirectoryLayerStore>) {
    Box::from_raw(layer_builder);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_cstring(cstring_ptr: *mut c_char) {
    CString::from_raw(cstring_ptr);
}
