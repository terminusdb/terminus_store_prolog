mod builder;
mod layer;
mod named_graph;
mod store;

#[no_mangle]
pub extern "C" fn install() {
    store::register_open_memory_store();
    store::register_open_directory_store();
    store::register_open_write();
    named_graph::register_create_named_graph();
    named_graph::register_open_named_graph();
    named_graph::register_head2();
    named_graph::register_head3();
    named_graph::register_nb_set_head();
    layer::register_store_id_layer();
    layer::register_id_triple();
    builder::register_nb_add_id_triple();
    builder::register_nb_add_string_triple();
    builder::register_nb_commit();
}
