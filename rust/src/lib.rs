use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::Mutex;

use terminus_store::layer::{IdTriple, Layer, ObjectType, StringTriple};
use terminus_store::storage::{name_to_string, string_to_name};
use terminus_store::store::sync::*;

mod csv;
use crate::csv::{csv_name_iri, import_csv};

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

fn error_to_cstring<E: Display>(error: E) -> CString {
    CString::new(format!("{}", error)).unwrap()
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
pub unsafe extern "C" fn named_graph_get_head_version(
    named_graph: *mut SyncNamedGraph,
    version: *mut u64,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
    match (*named_graph).head_version() {
        Ok((None, v)) => {
            *version = v;
            *err = std::ptr::null_mut();
            std::ptr::null_mut()
        }
        Ok((Some(layer), v)) => {
            *version = v;
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            *version = 0;
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
pub unsafe extern "C" fn named_graph_force_set_head(
    named_graph: *mut SyncNamedGraph,
    layer_ptr: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) {
    match (*named_graph).force_set_head(&*layer_ptr) {
        Ok(()) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn named_graph_force_set_head_version(
    named_graph: *mut SyncNamedGraph,
    layer_ptr: *mut SyncStoreLayer,
    version: u64,
    err: *mut *mut c_char,
) -> bool {
    match (*named_graph).force_set_head_version(&*layer_ptr, version) {
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
) {
    match (*builder).add_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(()) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
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

    match (*builder).add_string_triple(StringTriple::new_node(&subject, &predicate, &object)) {
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

    match (*builder).add_string_triple(StringTriple::new_value(&subject, &predicate, &object)) {
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
) {
    match (*builder).remove_id_triple(IdTriple::new(subject, predicate, object)) {
        Ok(()) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
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
) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(StringTriple::new_node(&subject, &predicate, &object)) {
        Ok(r) => {
            *err = std::ptr::null_mut();

            r
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
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
) {
    let subject = CStr::from_ptr(subject_ptr).to_string_lossy();
    let predicate = CStr::from_ptr(predicate_ptr).to_string_lossy();
    let object = CStr::from_ptr(object_ptr).to_string_lossy();

    match (*builder).remove_string_triple(StringTriple::new_value(&subject, &predicate, &object)) {
        Ok(()) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_committed(builder: *mut SyncStoreLayerBuilder) -> bool {
    (*builder).committed()
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
pub unsafe extern "C" fn builder_apply_delta(
    builder: *mut SyncStoreLayerBuilder,
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) {
    match (*builder).apply_delta(&*layer) {
        Ok(_) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn builder_apply_diff(
    builder: *mut SyncStoreLayerBuilder,
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) {
    match (*builder).apply_diff(&*layer) {
        Ok(_) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_parent(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
    *err = std::ptr::null_mut();
    match (*layer).parent() {
        Ok(Some(parent)) => Box::into_raw(Box::new(parent)),
        Ok(None) => std::ptr::null_mut(),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_squash(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> *mut SyncStoreLayer {
    match (*layer).squash() {
        Ok(new_layer) => {
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(new_layer))
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
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
pub unsafe extern "C" fn layer_triple_addition_count(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> usize {
    match (*layer).triple_layer_addition_count() {
        Ok(count) => {
            *err = std::ptr::null_mut();
            count
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_triple_removal_count(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> usize {
    match (*layer).triple_layer_removal_count() {
        Ok(count) => {
            *err = std::ptr::null_mut();
            count
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_total_triple_addition_count(layer: *mut SyncStoreLayer) -> usize {
    (*layer).triple_addition_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_total_triple_removal_count(layer: *mut SyncStoreLayer) -> usize {
    (*layer).triple_removal_count()
}

#[no_mangle]
pub unsafe extern "C" fn layer_total_triple_count(layer: *mut SyncStoreLayer) -> usize {
    (*layer).triple_count()
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

#[repr(C)]
pub struct U64Pair {
    pub first: u64,
    pub second: u64,
}

impl U64Pair {
    fn new(first: u64, second: u64) -> Self {
        Self { first, second }
    }
}

#[repr(C)]
pub struct U64Triple {
    pub first: u64,
    pub second: u64,
    pub third: u64,
}
impl U64Triple {
    fn new(first: u64, second: u64, third: u64) -> Self {
        Self {
            first,
            second,
            third,
        }
    }
}

// normal triple lookup
#[no_mangle]
pub extern "C" fn id_triple_spo_exists(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
    object: u64,
) -> bool {
    unsafe { &(*layer) }.triple_exists(subject, predicate, object)
}

#[no_mangle]
pub extern "C" fn id_triple_sp_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = u64>> = Box::new(
        unsafe { &(*layer) }
            .triples_sp(subject, predicate)
            .map(|triple| triple.object),
    );

    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn id_triple_so_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    object: u64,
) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = u64>> = Box::new(
        unsafe { &(*layer) }
            .triples_s(subject)
            .filter(move |triple| triple.object == object)
            .map(|triple| triple.predicate),
    );

    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn id_triple_s_iter(layer: *mut SyncStoreLayer, subject: u64) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = U64Pair>> = Box::new(
        unsafe { &(*layer) }
            .triples_s(subject)
            .map(|triple| U64Pair::new(triple.predicate, triple.object)),
    );

    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn id_triple_p_iter(layer: *mut SyncStoreLayer, predicate: u64) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = U64Pair>> = Box::new(
        unsafe { &(*layer) }
            .triples_p(predicate)
            .map(|triple| U64Pair::new(triple.subject, triple.object)),
    );
    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn id_triple_o_iter(layer: *mut SyncStoreLayer, object: u64) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = U64Pair>> = Box::new(
        unsafe { &(*layer) }
            .triples_o(object)
            .map(|triple| U64Pair::new(triple.subject, triple.predicate)),
    );
    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn id_triple_iter(layer: *mut SyncStoreLayer) -> *mut c_void {
    let iter: Box<dyn Iterator<Item = U64Triple>> = Box::new(
        unsafe { &(*layer) }
            .triples()
            .map(|t| U64Triple::new(t.subject, t.predicate, t.object)),
    );

    Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
}

// triple addition lookup
#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_spo_exists(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
    object: u64,
    err: *mut *mut c_char,
) -> bool {
    match (*layer).triple_addition_exists(subject, predicate, object) {
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
pub unsafe extern "C" fn id_triple_addition_sp_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions_sp(subject, predicate) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = u64>> = Box::new(it.map(|triple| triple.object));
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_so_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    object: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions_s(subject) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = u64>> = Box::new(
                it.filter(move |triple| triple.object == object)
                    .map(|triple| triple.predicate),
            );

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_s_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions_s(subject) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.predicate, triple.object)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_p_iter(
    layer: *mut SyncStoreLayer,
    predicate: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions_p(predicate) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.subject, triple.object)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_o_iter(
    layer: *mut SyncStoreLayer,
    object: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions_o(object) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.subject, triple.predicate)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_addition_iter(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_additions() {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Triple>> = Box::new(
                it.map(|triple| U64Triple::new(triple.subject, triple.predicate, triple.object)),
            );

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

// triple removal lookup
#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_spo_exists(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
    object: u64,
    err: *mut *mut c_char,
) -> bool {
    match (*layer).triple_removal_exists(subject, predicate, object) {
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
pub unsafe extern "C" fn id_triple_removal_sp_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    predicate: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals_sp(subject, predicate) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = u64>> = Box::new(it.map(|triple| triple.object));
            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_so_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    object: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals_s(subject) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = u64>> = Box::new(
                it.filter(move |triple| triple.object == object)
                    .map(|triple| triple.predicate),
            );

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_s_iter(
    layer: *mut SyncStoreLayer,
    subject: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals_s(subject) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.predicate, triple.object)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_p_iter(
    layer: *mut SyncStoreLayer,
    predicate: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals_p(predicate) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.subject, triple.object)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_o_iter(
    layer: *mut SyncStoreLayer,
    object: u64,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals_o(object) {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Pair>> =
                Box::new(it.map(|triple| U64Pair::new(triple.subject, triple.predicate)));

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn id_triple_removal_iter(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> *mut c_void {
    match (*layer).triple_removals() {
        Ok(it) => {
            let iter: Box<dyn Iterator<Item = U64Triple>> = Box::new(
                it.map(|triple| U64Triple::new(triple.subject, triple.predicate, triple.object)),
            );

            *err = std::ptr::null_mut();
            Box::into_raw(Box::new(Mutex::new(iter))) as *mut c_void
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            std::ptr::null_mut()
        }
    }
}

// iterators
#[no_mangle]
pub unsafe extern "C" fn u64_iter_next(iter: *mut c_void) -> u64 {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = u64>>>;
    (*iter)
        .lock()
        .expect("lock should succeed")
        .next()
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn u64_pair_iter_next(iter: *mut c_void) -> U64Pair {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = U64Pair>>>;
    (*iter)
        .lock()
        .expect("lock should succeed")
        .next()
        .unwrap_or(U64Pair::new(0, 0))
}

#[no_mangle]
pub unsafe extern "C" fn u64_triple_iter_next(iter: *mut c_void) -> U64Triple {
    let iter = iter as *mut Mutex<Box<dyn Iterator<Item = U64Triple>>>;
    (*iter)
        .lock()
        .expect("lock should succeed")
        .next()
        .unwrap_or(U64Triple::new(0, 0, 0))
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
pub unsafe extern "C" fn layer_id_to_string(id: *const u32) -> *mut c_char {
    let id_array = [
        *id,
        *id.offset(1),
        *id.offset(2),
        *id.offset(3),
        *id.offset(4),
    ];
    let name_string = name_to_string(id_array);

    CString::new(name_string)
        .expect("layer name to string conversion should always result in a valid unicode string")
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn layer_string_to_id(
    name_ptr: *const c_char,
    result: *mut [u32; 5],
    err: *mut *mut c_char,
) -> bool {
    let name_slice = std::slice::from_raw_parts(name_ptr as *const u8, 40); // ids are 5 * 4 bytes, in hexadecimal, which works out to 40.
    let name_str = match std::str::from_utf8(name_slice) {
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
            *result = [0, 0, 0, 0, 0];

            return false;
        }
        Ok(ns) => ns,
    };

    match string_to_name(name_str) {
        Ok(name) => {
            *err = std::ptr::null_mut();
            *result = name;

            true
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            *result = [0, 0, 0, 0, 0];

            false
        }
    }
}

#[repr(C)]
pub struct VecHandle {
    ptr: *mut c_void,
    len: usize,
    capacity: usize,
}

impl VecHandle {
    fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn pack_export(
    store: *mut SyncStore,
    layer_ids_ptr: *const [u32; 5],
    layer_ids_len: usize,
    err: *mut *mut c_char,
) -> VecHandle {
    let layer_ids = std::slice::from_raw_parts(layer_ids_ptr, layer_ids_len);
    let vec: Vec<_> = layer_ids.to_vec();
    let result = (*store).export_layers(Box::new(vec.into_iter()));
    if let Err(e) = result {
        *err = error_to_cstring(e).into_raw();
        return VecHandle {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        };
    }
    let mut result = result.unwrap();
    *err = std::ptr::null_mut();

    let len = result.len();
    let capacity = result.capacity();
    let ptr: *mut c_void = &mut result[0] as *mut u8 as *mut c_void;

    std::mem::forget(result);

    VecHandle { ptr, len, capacity }
}

#[no_mangle]
pub unsafe extern "C" fn pack_import(
    store: *mut SyncStore,
    pack_ptr: *const u8,
    pack_len: usize,
    layer_ids_ptr: *const [u32; 5],
    layer_ids_len: usize,
    err: *mut *mut c_char,
) {
    let pack = std::slice::from_raw_parts(pack_ptr, pack_len);
    let layer_ids = std::slice::from_raw_parts(layer_ids_ptr, layer_ids_len);
    let vec: Vec<_> = layer_ids.to_vec();
    match (*store).import_layers(pack, Box::new(vec.into_iter())) {
        Ok(()) => *err = std::ptr::null_mut(),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[repr(C)]
pub struct LayerAndParent {
    layer_id: [u32; 5],
    layer_parent_id: [u32; 5],
    has_parent: bool,
}

#[no_mangle]
pub unsafe extern "C" fn pack_layerids_and_parents(
    pack_ptr: *const u8,
    pack_len: usize,
    err: *mut *mut c_char,
) -> VecHandle {
    let pack = std::slice::from_raw_parts(pack_ptr, pack_len);
    match terminus_store::storage::pack::pack_layer_parents(pack) {
        Ok(id_parent_map) => {
            let mut result_vec: Vec<LayerAndParent> = id_parent_map
                .into_iter()
                .map(|(id, parent)| LayerAndParent {
                    layer_id: id,
                    has_parent: parent.is_some(),
                    layer_parent_id: parent.unwrap_or([0, 0, 0, 0, 0]),
                })
                .collect();

            let len = result_vec.len();
            let capacity = result_vec.capacity();
            let ptr = &mut result_vec[0] as *mut LayerAndParent as *mut c_void;

            let result = VecHandle { ptr, len, capacity };

            *err = std::ptr::null_mut();

            std::mem::forget(result_vec);
            result
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            VecHandle::null()
        }
    }

    // do a thing with into_boxed_slice

    // gotta cleanup the boxed slice somehow too
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
pub unsafe extern "C" fn cleanup_subject_predicate_objects_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = u64>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_object_subject_predicates_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = (u64, u64)>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_u64_triple_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = U64Triple>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_u64_pair_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = U64Pair>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_u64_iter(iter: *mut c_void) {
    let _iter = Box::from_raw(iter as *mut Mutex<Box<dyn Iterator<Item = u64>>>);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_cstring(cstring_ptr: *mut c_char) {
    CString::from_raw(cstring_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_u8_vec(vec_handle: VecHandle) {
    let _vec: Vec<u8> = Vec::from_raw_parts(
        vec_handle.ptr as *mut u8,
        vec_handle.len,
        vec_handle.capacity,
    );
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer_and_parent_vec(vec_handle: VecHandle) {
    let _vec: Vec<LayerAndParent> = Vec::from_raw_parts(
        vec_handle.ptr as *mut LayerAndParent,
        vec_handle.len,
        vec_handle.capacity,
    );
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_layer_vec(vec_handle: VecHandle) {
    let _vec: Vec<[u32; 5]> = Vec::from_raw_parts(
        vec_handle.ptr as *mut [u32; 5],
        vec_handle.len,
        vec_handle.capacity,
    );
}

#[no_mangle]
pub unsafe extern "C" fn add_csv_to_builder(
    name: *mut c_char,
    csv: *mut c_char,
    builder: *mut SyncStoreLayerBuilder,
    schema_builder: *mut SyncStoreLayerBuilder,
    data_prefix: *mut c_char,
    predicate_prefix: *mut c_char,
    header: c_int,
    skip_header: c_int,
    err: *mut *mut c_char,
) {
    let csv_name = CStr::from_ptr(name).to_str().unwrap().to_string();
    let csv_path = CStr::from_ptr(csv).to_str().unwrap().to_string();
    let data_prefix = CStr::from_ptr(data_prefix).to_str().unwrap().to_string();
    let predicate_prefix = CStr::from_ptr(predicate_prefix)
        .to_str()
        .unwrap()
        .to_string();

    let header = header != 0;
    let skip_header = skip_header != 0;

    let schema_builder_option;
    if schema_builder.is_null() {
        schema_builder_option = None
    } else {
        schema_builder_option = Some(&*schema_builder)
    }

    let result = import_csv(
        csv_name,
        csv_path,
        &*builder,
        schema_builder_option,
        data_prefix,
        predicate_prefix,
        header,
        skip_header,
    );

    match result {
        Ok(()) => {
            *err = std::ptr::null_mut();
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn csv_iri(csv_name: *const c_char, prefix: *const c_char) -> *mut c_char {
    let csv_name_str = CStr::from_ptr(csv_name).to_string_lossy().to_string();
    let prefix_str = CStr::from_ptr(prefix).to_string_lossy().to_string();
    let (_, node) = csv_name_iri(csv_name_str, prefix_str);
    CString::new(node).unwrap().into_raw() as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn layer_rollup(layer: *mut SyncStoreLayer, err: *mut *mut c_char) {
    match (*layer).rollup() {
        Ok(()) => *err = std::ptr::null_mut(),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_rollup_upto(
    layer: *mut SyncStoreLayer,
    upto: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) {
    match (*layer).rollup_upto(&*upto) {
        Ok(()) => *err = std::ptr::null_mut(),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn layer_imprecise_rollup_upto(
    layer: *mut SyncStoreLayer,
    upto: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) {
    match (*layer).imprecise_rollup_upto(&*upto) {
        Ok(()) => *err = std::ptr::null_mut(),
        Err(e) => {
            *err = error_to_cstring(e).into_raw();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn retrieve_layer_stack_names(
    layer: *mut SyncStoreLayer,
    err: *mut *mut c_char,
) -> VecHandle {
    match (*layer).retrieve_layer_stack_names() {
        Ok(mut names) => {
            let len = names.len();
            let capacity = names.capacity();
            let ptr = &mut names[0] as *mut [u32; 5] as *mut c_void;

            let result = VecHandle { ptr, len, capacity };

            *err = std::ptr::null_mut();

            std::mem::forget(names);
            result
        }
        Err(e) => {
            *err = error_to_cstring(e).into_raw();

            VecHandle::null()
        }
    }
}
