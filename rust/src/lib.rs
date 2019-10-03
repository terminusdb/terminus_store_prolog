use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::io;
use std::sync::Mutex;

use terminus_store::layer::{
    Layer, StringTriple, IdTriple, ObjectType, SubjectLookup,
    SubjectPredicateLookup
};
use terminus_store::store::sync::*;

#[no_mangle]
pub unsafe extern "C" fn open_memory_store() -> *mut SyncStore {
    let store = open_sync_memory_store();
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn open_directory_store(
    dir: *mut c_char,
) -> *mut SyncStore {
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
    name: *mut c_char,
    err: *mut *mut c_char,
) -> *mut SyncDatabase {
    let store = store_ptr as *mut SyncStore;
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
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_database(
    store: *mut SyncStore,
    name: *mut c_char,
    err: *mut *mut c_char,
) -> *mut SyncDatabase {
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = CStr::from_ptr(name);
    let db_name = db_name_cstr.to_str().unwrap();

    // Safe because we expect the swipl pointers to be decent
    match (*store).open(db_name) {
        Ok(Some(database)) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(database))
        }
        Ok(None) => {
            *err = std::ptr::null_mut();
            std::ptr::null_mut()
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_get_head(database: *mut SyncDatabase, err: *mut *mut c_char) -> *mut SyncDatabaseLayer {
    match (*database).head() {
        Ok(None) => {
            *err = std::ptr::null_mut();
            std::ptr::null_mut()
        },
        Ok(Some(layer)) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_set_head(database: *mut SyncDatabase, layer_ptr: *mut SyncDatabaseLayer, err: *mut *mut c_char) -> bool {
    match (*database).set_head(&*layer_ptr) {
        Ok(b) => {
            *err = std::ptr::null_mut();
            b

        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn store_create_base_layer(store: *mut SyncStore, err: *mut *mut c_char) -> *mut SyncDatabaseLayerBuilder {
    match (*store).create_base_layer() {
        Ok(builder) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(builder))
        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_open_write(layer: *mut SyncDatabaseLayer, err: *mut *mut c_char) -> *mut SyncDatabaseLayerBuilder {
    match (*layer).open_write() {
        Ok(builder) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(builder))
        },
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn database_open_write(database: *mut SyncDatabase, err: *mut *mut c_char) -> *mut SyncDatabaseLayerBuilder {
    match (*database)
        .head()
        .and_then(|layer|
                  layer.map(|l|match l.open_write() {
                      Ok(builder) => Ok(Some(builder)),
                      Err(e) => Err(e)
                  }).unwrap_or(Ok(None))) {
            Ok(Some(builder)) => {
                *err = std::ptr::null_mut();
                Box::into_raw(Box::new(builder))
            },
            Ok(None) => {
                *err = CString::new("Create a base layer first before opening the database for write")
                    .unwrap()
                    .into_raw();
                std::ptr::null_mut()
            }
            Err(e) => {
                *err = error_to_cstring(e).into_raw();
                std::ptr::null_mut()
            }
        }
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_id_triple(builder: *mut SyncDatabaseLayerBuilder, subject: u64, predicate: u64, object: u64, err: *mut *mut c_char) -> bool {
    match (*builder).add_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(r) => {
            *err = std::ptr::null_mut();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_string_node_triple(builder: *mut SyncDatabaseLayerBuilder, subject_ptr: *mut c_char, predicate_ptr: *mut c_char, object_ptr: *mut c_char, err: *mut *mut c_char) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null_mut(),
        Err(e) => *err = error_to_cstring(e).into_raw()
    };
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_string_value_triple(builder: *mut SyncDatabaseLayerBuilder, subject_ptr: *mut c_char, predicate_ptr: *mut c_char, object_ptr: *mut c_char, err: *mut *mut c_char) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null_mut(),
        Err(e) => *err = error_to_cstring(e).into_raw()
    };
}


#[no_mangle]
pub unsafe extern "C" fn builder_remove_id_triple(builder: *mut SyncDatabaseLayerBuilder, subject: u64, predicate: u64, object: u64, err: *mut *mut c_char) -> bool {
    match (*builder).remove_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(r) => {
            *err = std::ptr::null_mut();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_remove_string_node_triple(builder: *mut SyncDatabaseLayerBuilder, subject_ptr: *mut c_char, predicate_ptr: *mut c_char, object_ptr: *mut c_char, err: *mut *mut c_char) -> bool {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(&StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(r) => {
            *err = std::ptr::null_mut();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_remove_string_value_triple(builder: *mut SyncDatabaseLayerBuilder, subject_ptr: *mut c_char, predicate_ptr: *mut c_char, object_ptr: *mut c_char, err: *mut *mut c_char) -> bool {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(&StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(r) => {
            *err = std::ptr::null_mut();
            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_commit(builder: *mut SyncDatabaseLayerBuilder, err: *mut *mut c_char) -> *mut SyncDatabaseLayer {
    match (*builder).commit() {
        Ok(layer) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_node_and_value_count(layer: *mut SyncDatabaseLayer) -> usize {
    (*layer).node_and_value_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_count(layer: *mut SyncDatabaseLayer) -> usize {
    (*layer).predicate_count()
}


#[no_mangle]
pub unsafe extern "C" fn layer_subject_id(layer: *mut SyncDatabaseLayer, subject: *mut c_char) -> u64 {
    let cstr = CStr::from_ptr(subject).to_string_lossy();
    (*layer).subject_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_id(layer: *mut SyncDatabaseLayer, predicate: *mut c_char) -> u64 {
    let cstr = CStr::from_ptr(predicate).to_string_lossy();
    (*layer).predicate_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_node_id(layer: *mut SyncDatabaseLayer, object: *mut c_char) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_node_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_value_id(layer: *mut SyncDatabaseLayer, object: *mut c_char) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_value_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_subject(layer: *mut SyncDatabaseLayer, id: u64) -> *mut c_char {
    (*layer).id_subject(id).map(|s|CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_predicate(layer: *mut SyncDatabaseLayer, id: u64) -> *mut c_char {
    (*layer).id_predicate(id).map(|s|CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_object(layer: *mut SyncDatabaseLayer, id: u64, object_type: *mut u8) -> *mut c_char {
    (*layer).id_object(id).map(|x| match x {
        ObjectType::Node(s) => {
            *object_type = 0;
            s
        },
        ObjectType::Value(s) => {
            *object_type = 1;
            s
        }
    }).map(|s|CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_subject(layer: *mut SyncDatabaseLayer, subject: u64) -> *mut c_void {
    match (*layer).lookup_subject(subject) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_subjects_iter(layer: *mut SyncDatabaseLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).subjects()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subjects_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item=Box<dyn SubjectLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(subject_lookup) => Box::into_raw(Box::new(subject_lookup)) as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn subject_lookup_subject(subject_lookup: *mut c_void) -> u64 {
    let subject_lookup = subject_lookup as *mut Box<dyn SubjectLookup>;
    (*subject_lookup).subject()
}

#[no_mangle]
pub unsafe extern "C" fn subject_lookup_lookup_predicate(subject_lookup: *mut c_void, predicate: u64) -> *mut c_void {
    let subject_lookup = subject_lookup as *mut Box<dyn SubjectLookup>;
    // *mut Box<dyn SubjectPredicateLookup>;
    match (*subject_lookup).lookup_predicate(predicate) {
        None => std::ptr::null_mut(),
        Some(objects_for_po_pair) => {
            let obj_po_pair_box = Box::new(objects_for_po_pair);
            Box::into_raw(obj_po_pair_box) as *mut c_void
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn subject_lookup_predicates_iter(subject_lookup: *mut c_void) -> *mut c_void {
    let subject_lookup = subject_lookup as *mut Box<dyn SubjectLookup>;
    Box::into_raw(Box::new(Mutex::new((*subject_lookup).predicates()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicates_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item=Box<dyn SubjectPredicateLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(objects_for_po_pair) => Box::into_raw(Box::new(objects_for_po_pair)) as *mut c_void
    }
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_subject(objects_for_po_pair: *mut c_void) -> u64 {
    let objects_for_po_pair = objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>;
    (*objects_for_po_pair).subject()
    
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_predicate(objects_for_po_pair: *mut c_void) -> u64 {
    let objects_for_po_pair = objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>;
    (*objects_for_po_pair).predicate()
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_objects_iter(objects: *mut c_void) -> *mut c_void {
    let objects = objects as *mut Box<dyn SubjectPredicateLookup>;
    let iter: Box<dyn Iterator<Item=u64>> = Box::new((*objects).triples().map(|t|t.object));
    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_objects_iter_next(iter: *mut c_void) -> u64 {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item=u64>>>;
    (*iter).lock().expect("lock should succeed").next().unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_lookup_object(objects: *mut c_void, object: u64) -> bool {
    let objects = objects as *mut Box<dyn SubjectPredicateLookup>;
    (*objects).triple(object).is_some()
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_store(store: *mut SyncStore) {
    Box::from_raw(store);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_db(db: *mut SyncDatabase) {
    Box::from_raw(db);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer(layer: *mut SyncDatabaseLayer) {
    Box::from_raw(layer);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer_builder(layer_builder: *mut SyncDatabaseLayerBuilder) {
    Box::from_raw(layer_builder);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_lookup(subject_lookup: *mut c_void) {
    Box::from_raw(subject_lookup as *mut Box<dyn SubjectLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subjects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item=Box<dyn SubjectLookup>>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicate_lookup(objects_for_po_pair: *mut c_void) {
    Box::from_raw(objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicates_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as
                              *mut Mutex<Box<dyn Iterator<Item=Box<dyn SubjectPredicateLookup>>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicate_objects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item=u64>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_cstring(cstring_ptr: *mut c_char) {
    CString::from_raw(cstring_ptr);
}
