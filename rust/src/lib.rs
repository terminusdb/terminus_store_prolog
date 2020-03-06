use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uchar, c_int, c_void};
use std::io;
use std::slice;
use std::sync::Mutex;
use std::path::Path;

extern "C" {
    // Our C function definitions
    pub fn c_debug_via_prolog(topic: *const c_char, comment: *const c_char) -> ();
    pub fn c_log_via_prolog(comment: *const c_char) -> ();
}

use terminus_store::logging::{add_debug_hook, add_logging_hook, DebugSink, LoggingSink};

// A DebugSink that writes the debug info to SWI-Prolog's debug/3
struct PrologDebug {}

impl DebugSink for PrologDebug {
    fn debug(&self, topic: &str, comment: &str) {
        // convert topic and comment to C strings
        let c_topic = CString::new(topic).expect("CString::new failed to convert topic");
        let c_comment = CString::new(comment).expect("CString::new failed to convert comment");

        unsafe { c_debug_via_prolog(c_topic.as_ptr(), c_comment.as_ptr()) }
        ()
    }
}

// A LoggingSink that writes the debug info to SWI-Prolog's http_log system
struct PrologLogger {}

impl LoggingSink for PrologLogger {
    fn log(&self, comment: &str) {
        let c_comment = CString::new(comment).expect("CString::new failed to convert comment");

        unsafe { c_log_via_prolog(c_comment.as_ptr()) }
    }
}

static DEBUG_SINK_IMPL: PrologDebug = PrologDebug {};
static LOGGING_SINK_IMPL: PrologLogger = PrologLogger {};

#[no_mangle]
pub unsafe extern "C" fn rust_install_prolog_debug_hook() {
    add_debug_hook(&DEBUG_SINK_IMPL);
}

#[no_mangle]
pub unsafe extern "C" fn rust_install_prolog_log_hook() {
    add_logging_hook(&LOGGING_SINK_IMPL);
}

use terminus_store::layer::{
    IdTriple, Layer, ObjectLookup, ObjectType, PredicateLookup, StringTriple, SubjectLookup,
    SubjectPredicateLookup,
};
use terminus_store::store;
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;

#[no_mangle]
pub unsafe extern "C" fn open_memory_store() -> *mut SyncStore {
    let store = open_sync_memory_store();
    Box::into_raw(Box::new(store))
}

#[no_mangle]
pub unsafe extern "C" fn open_directory_store(dir: *mut c_char) -> *mut SyncStore {
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
pub unsafe extern "C" fn serialize_directory_store(dir: *mut c_char, label_names: *mut *mut c_char,
                                                   label_list_length: c_int, layer_ids: *mut *mut c_char, layer_id_length: c_int) -> *mut c_uchar {
    let directory = CStr::from_ptr(dir).to_str().unwrap();
    let directory_path = Path::new(&directory);
    let label_names: Vec<&str> = slice::from_raw_parts(label_names, label_list_length as usize).iter().map(|x| CStr::from_ptr(*x).to_str().unwrap()).collect();
    let layer_ids: Vec<&str> = slice::from_raw_parts(layer_ids, layer_id_length as usize).iter().map(|x| CStr::from_ptr(*x).to_str().unwrap()).collect();
    store::serialize_directory_store(directory_path, label_names.as_slice(), layer_ids.as_slice()).as_mut_ptr()
}


#[no_mangle]
pub unsafe extern "C" fn deserialize_directory_store(tar_path: *mut c_char, directory_store_path: *mut c_char) {
    let directory = CStr::from_ptr(directory_store_path).to_str().unwrap();
    let directory_path = Path::new(&directory);
    let tar = CStr::from_ptr(tar_path).to_str().unwrap();
    let tar_path = Path::new(&tar);
    store::deserialize_directory_store(directory_path, tar_path).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn create_named_graph(
    store_ptr: *mut c_void,
    name: *mut c_char,
    err: *mut *mut c_char,
) -> *mut SyncNamedGraph {
    let store = store_ptr as *mut SyncStore;
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = CStr::from_ptr(name);
    let db_name = db_name_cstr.to_str().unwrap();

    // Safe because we expect the swipl pointers to be decent
    match (*store).create(db_name) {
        Ok(named_graph) => Box::into_raw(Box::new(named_graph)),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_named_graph(
    store: *mut SyncStore,
    name: *mut c_char,
    err: *mut *mut c_char,
) -> *mut SyncNamedGraph {
    // We assume it to be somewhat safe because swipl will check string types
    let db_name_cstr = CStr::from_ptr(name);
    let db_name = db_name_cstr.to_str().unwrap();

    // Safe because we expect the swipl pointers to be decent
    match (*store).open(db_name) {
        Ok(Some(named_graph)) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(named_graph))
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
pub unsafe extern "C" fn named_graph_get_name(named_graph: *mut SyncNamedGraph) -> *mut c_char {
    CString::new((*named_graph).name()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn named_graph_get_head(
    named_graph: *mut SyncNamedGraph,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
    match (*named_graph).head() {
        Ok(None) => {
            *err = std::ptr::null_mut();
            std::ptr::null_mut()
        }
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
pub unsafe extern "C" fn named_graph_set_head(
    named_graph: *mut SyncNamedGraph,
    layer_ptr: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> bool {
    match (*named_graph).set_head(&*layer_ptr) {
        Ok(b) => {
            *err = std::ptr::null_mut();
            b
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn store_create_base_layer(
    store: *mut SyncStore,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayerBuilder {
    match (*store).create_base_layer() {
        Ok(builder) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(builder))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_builder_get_id(builder: *mut SyncStoreLayerBuilder) -> *mut c_char {
    CString::new(name_to_string((*builder).name()))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn layer_get_id(layer: *mut SyncStoreLayer) -> *mut c_char {
    CString::new(name_to_string((*layer).name()))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn layer_open_write(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayerBuilder {
    match (*layer).open_write() {
        Ok(builder) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(builder))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn named_graph_open_write(
    named_graph: *mut SyncNamedGraph,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayerBuilder {
    match (*named_graph).head().and_then(|layer| {
        layer
            .map(|l| match l.open_write() {
                Ok(builder) => Ok(Some(builder)),
                Err(e) => Err(e),
            })
            .unwrap_or(Ok(None))
    }) {
        Ok(Some(builder)) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(builder))
        }
        Ok(None) => {
            *err =
                CString::new("Create a base layer first before opening the named graph for write")
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
pub unsafe extern "C" fn builder_add_id_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject: u64,
    predicate: u64,
    object: u64,
    err: *mut *mut c_char,
) -> bool {
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
pub unsafe extern "C" fn builder_add_string_node_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject_ptr: *mut c_char,
    predicate_ptr: *mut c_char,
    object_ptr: *mut c_char,
    err: *mut *mut c_char,
) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null_mut(),
        Err(e) => *err = error_to_cstring(e).into_raw(),
    };
}

#[no_mangle]
pub unsafe extern "C" fn builder_add_string_value_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject_ptr: *mut c_char,
    predicate_ptr: *mut c_char,
    object_ptr: *mut c_char,
    err: *mut *mut c_char,
) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).add_string_triple(&StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(_) => *err = std::ptr::null_mut(),
        Err(e) => *err = error_to_cstring(e).into_raw(),
    };
}

#[no_mangle]
pub unsafe extern "C" fn builder_remove_id_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject: u64,
    predicate: u64,
    object: u64,
    err: *mut *mut c_char,
) -> bool {
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
pub unsafe extern "C" fn builder_remove_string_node_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject_ptr: *mut c_char,
    predicate_ptr: *mut c_char,
    object_ptr: *mut c_char,
    err: *mut *mut c_char,
) -> bool {
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
pub unsafe extern "C" fn builder_remove_string_value_triple(
    builder: *mut SyncStoreLayerBuilder,
    subject_ptr: *mut c_char,
    predicate_ptr: *mut c_char,
    object_ptr: *mut c_char,
    err: *mut *mut c_char,
) -> bool {
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
pub unsafe extern "C" fn builder_commit(
    builder: *mut SyncStoreLayerBuilder,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
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
pub unsafe extern "C" fn layer_parent(layer: *mut SyncStoreLayer) -> *mut SyncStoreLayer {
    match (*layer).parent() {
        Some(parent) => Box::into_raw(Box::new(parent)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_node_and_value_count(layer: *mut SyncStoreLayer) -> usize {
    (*layer).node_and_value_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_count(layer: *mut SyncStoreLayer) -> usize {
    (*layer).predicate_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_subject_id(layer: *mut SyncStoreLayer, subject: *mut c_char) -> u64 {
    let cstr = CStr::from_ptr(subject).to_string_lossy();
    (*layer).subject_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_id(
    layer: *mut SyncStoreLayer,
    predicate: *mut c_char,
) -> u64 {
    let cstr = CStr::from_ptr(predicate).to_string_lossy();
    (*layer).predicate_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_node_id(
    layer: *mut SyncStoreLayer,
    object: *mut c_char,
) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_node_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_value_id(
    layer: *mut SyncStoreLayer,
    object: *mut c_char,
) -> u64 {
    let cstr = CStr::from_ptr(object).to_string_lossy();
    (*layer).object_value_id(&cstr).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_subject(layer: *mut SyncStoreLayer, id: u64) -> *mut c_char {
    (*layer)
        .id_subject(id)
        .map(|s| CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_predicate(layer: *mut SyncStoreLayer, id: u64) -> *mut c_char {
    (*layer)
        .id_predicate(id)
        .map(|s| CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_id_object(
    layer: *mut SyncStoreLayer,
    id: u64,
    object_type: *mut u8,
) -> *mut c_char {
    (*layer)
        .id_object(id)
        .map(|x| match x {
            ObjectType::Node(s) => {
                *object_type = 0;
                s
            }
            ObjectType::Value(s) => {
                *object_type = 1;
                s
            }
        })
        .map(|s| CString::new(s).unwrap().into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_subject(
    layer: *mut SyncStoreLayer,
    subject: u64,
) -> *mut c_void {
    match (*layer).lookup_subject(subject) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_subject_addition(
    layer: *mut SyncStoreLayer,
    subject: u64,
) -> *mut c_void {
    match (*layer).lookup_subject_addition(subject) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_subject_removal(
    layer: *mut SyncStoreLayer,
    subject: u64,
) -> *mut c_void {
    match (*layer).lookup_subject_removal(subject) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_predicate(
    layer: *mut SyncStoreLayer,
    predicate: u64,
) -> *mut c_void {
    match (*layer).lookup_predicate(predicate) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_predicate_addition(
    layer: *mut SyncStoreLayer,
    predicate: u64,
) -> *mut c_void {
    match (*layer).lookup_predicate_addition(predicate) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_predicate_removal(
    layer: *mut SyncStoreLayer,
    predicate: u64,
) -> *mut c_void {
    match (*layer).lookup_predicate_removal(predicate) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_object(
    layer: *mut SyncStoreLayer,
    object: u64,
) -> *mut c_void {
    match (*layer).lookup_object(object) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_object_addition(
    layer: *mut SyncStoreLayer,
    object: u64,
) -> *mut c_void {
    match (*layer).lookup_object_addition(object) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_lookup_object_removal(
    layer: *mut SyncStoreLayer,
    object: u64,
) -> *mut c_void {
    match (*layer).lookup_object_removal(object) {
        Some(result) => Box::into_raw(Box::new(result)) as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_subjects_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).subjects()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_subject_additions_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).subject_additions()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_subject_removals_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).subject_removals()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subjects_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn SubjectLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(subject_lookup) => Box::into_raw(Box::new(subject_lookup)) as *mut c_void,
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicates_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).predicates()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_additions_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).predicate_additions()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_predicate_removals_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).predicate_removals()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn predicates_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn ObjectLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(object_lookup) => Box::into_raw(Box::new(object_lookup)) as *mut c_void,
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_objects_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).objects()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_additions_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).object_additions()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn layer_object_removals_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    Box::into_raw(Box::new(Mutex::new((*layer).object_removals()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn objects_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn ObjectLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(object_lookup) => Box::into_raw(Box::new(object_lookup)) as *mut c_void,
    }
}

#[no_mangle]
pub unsafe extern "C" fn subject_lookup_subject(subject_lookup: *mut c_void) -> u64 {
    let subject_lookup = subject_lookup as *mut Box<dyn SubjectLookup>;
    (*subject_lookup).subject()
}

#[no_mangle]
pub unsafe extern "C" fn subject_lookup_lookup_predicate(
    subject_lookup: *mut c_void,
    predicate: u64,
) -> *mut c_void {
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
pub unsafe extern "C" fn subject_lookup_predicates_iter(
    subject_lookup: *mut c_void,
) -> *mut c_void {
    let subject_lookup = subject_lookup as *mut Box<dyn SubjectLookup>;
    Box::into_raw(Box::new(Mutex::new((*subject_lookup).predicates()))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicates_iter_next(iter: *mut c_void) -> *mut c_void {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn SubjectPredicateLookup>>>>;
    match (*iter).lock().expect("locking should succeed").next() {
        None => std::ptr::null_mut(),
        Some(objects_for_po_pair) => Box::into_raw(Box::new(objects_for_po_pair)) as *mut c_void,
    }
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_subject(objects_for_po_pair: *mut c_void) -> u64 {
    let objects_for_po_pair = objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>;
    (*objects_for_po_pair).subject()
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_predicate(
    objects_for_po_pair: *mut c_void,
) -> u64 {
    let objects_for_po_pair = objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>;
    (*objects_for_po_pair).predicate()
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_objects_iter(
    objects: *mut c_void,
) -> *mut c_void {
    let objects = objects as *mut Box<dyn SubjectPredicateLookup>;
    let iter: Box<dyn Iterator<Item = u64>> = Box::new((*objects).objects());
    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_objects_iter_next(iter: *mut c_void) -> u64 {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = u64>>>;
    (*iter)
        .lock()
        .expect("lock should succeed")
        .next()
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn subject_predicate_lookup_lookup_object(
    objects: *mut c_void,
    object: u64,
) -> bool {
    let objects = objects as *mut Box<dyn SubjectPredicateLookup>;
    (*objects).has_object(object)
}

#[no_mangle]
pub unsafe extern "C" fn predicate_lookup_predicate(predicate_lookup: *mut c_void) -> u64 {
    let predicate_lookup = predicate_lookup as *mut Box<dyn PredicateLookup>;
    (*predicate_lookup).predicate()
}

#[no_mangle]
pub unsafe extern "C" fn predicate_lookup_subject_predicate_pairs_iter(
    predicate_lookup: *mut c_void,
) -> *mut c_void {
    let predicate_lookup = predicate_lookup as *mut Box<dyn PredicateLookup>;
    Box::into_raw(Box::new(Mutex::new(
        (*predicate_lookup).subject_predicate_pairs(),
    ))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn object_lookup_object(object_lookup: *mut c_void) -> u64 {
    let object_lookup = object_lookup as *mut Box<dyn ObjectLookup>;
    (*object_lookup).object()
}

#[no_mangle]
pub unsafe extern "C" fn object_lookup_lookup_subject_predicate_pair(
    object_lookup: *mut c_void,
    subject: u64,
    predicate: u64,
) -> bool {
    let object_lookup = object_lookup as *mut Box<dyn ObjectLookup>;
    (*object_lookup).has_subject_predicate_pair(subject, predicate)
}

#[no_mangle]
pub unsafe extern "C" fn object_lookup_subject_predicate_pairs_iter(
    object_lookup: *mut c_void,
) -> *mut c_void {
    let object_lookup = object_lookup as *mut Box<dyn ObjectLookup>;
    let iter = Box::new((*object_lookup).subject_predicate_pairs())
        as Box<dyn Iterator<Item = (u64, u64)>>;
    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[repr(C)]
pub struct SubjectPredicatePair {
    pub subject: u64,
    pub predicate: u64,
}

#[no_mangle]
pub unsafe extern "C" fn object_subject_predicate_pairs_iter_next(
    iter: *mut c_void,
) -> SubjectPredicatePair {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = (u64, u64)>>>;
    let (subject, predicate) = (*iter)
        .lock()
        .expect("lock should succeed")
        .next()
        .unwrap_or((0, 0));

    SubjectPredicatePair { subject, predicate }
}

#[no_mangle]
pub unsafe extern "C" fn store_get_layer_from_id(
    store: *mut SyncStore,
    id: *mut c_char,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
    let id_cstr = CStr::from_ptr(id);
    let id_str = id_cstr.to_str().unwrap();

    match string_to_name(id_str).and_then(|id| (*store).get_layer_from_id(id)) {
        Ok(Some(layer)) => Box::into_raw(Box::new(layer)),
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
pub unsafe extern "C" fn cleanup_store(store: *mut SyncStore) {
    Box::from_raw(store);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_db(db: *mut SyncNamedGraph) {
    Box::from_raw(db);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer(layer: *mut SyncStoreLayer) {
    Box::from_raw(layer);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer_builder(layer_builder: *mut SyncStoreLayerBuilder) {
    Box::from_raw(layer_builder);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_lookup(subject_lookup: *mut c_void) {
    Box::from_raw(subject_lookup as *mut Box<dyn SubjectLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicate_lookup(objects_for_po_pair: *mut c_void) {
    Box::from_raw(objects_for_po_pair as *mut Box<dyn SubjectPredicateLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_predicate_lookup(subject_lookup: *mut c_void) {
    Box::from_raw(subject_lookup as *mut Box<dyn PredicateLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_object_lookup(object_lookup: *mut c_void) {
    Box::from_raw(object_lookup as *mut Box<dyn ObjectLookup>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subjects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn SubjectLookup>>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicates_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(
        iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn SubjectPredicateLookup>>>>,
    );
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_predicates_iter(iter: *mut c_void) {
    let _iter =
        Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn PredicateLookup>>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_objects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = Box<dyn ObjectLookup>>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_subject_predicate_objects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = u64>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_object_subject_predicates_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = (u64, u64)>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_cstring(cstring_ptr: *mut c_char) {
    CString::from_raw(cstring_ptr);
}
