mod store;
mod named_graph;
mod layer;
mod builder;

#[no_mangle]
pub extern "C" fn install() {
    store::register_open_memory_store();
    store::register_open_directory_store();
    named_graph::register_create_named_graph();
    named_graph::register_open_named_graph();
    named_graph::register_head2();
    named_graph::register_head3();
}
