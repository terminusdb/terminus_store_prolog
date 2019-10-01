void* open_directory_store(char* dir_name);
void cleanup_directory_store(void* store);
void cleanup_db(void* db);
void cleanup_layer(void* layer);
void cleanup_layer_builder(void* layer_builder);
void cleanup_cstring(char* c_string);
void cleanup_po_pairs_for_subject(void* po_pairs_for_subject);
void cleanup_po_pairs_iter(void* iter);
void cleanup_objects_for_po_pair(void* objects_for_po_pair);
void cleanup_objects_iter(void* iter);

void* create_database(void* store, char* name, char** err);
void* open_database(void* store, char* name, char** err);
void* database_get_head(void* db, char** err);
void* database_set_head(void* db, void* layer, char** err);
void* store_create_base_layer(void* db, char** err);
void* database_open_write(void* layer, char** err);
void* layer_open_write(void* layer, char** err);
_Bool builder_add_id_triple(void* builder, uint64_t subject, uint64_t predicate, uint64_t object, char** err);
void builder_add_string_node_triple(void* builder, char* subject, char* predicate, char* object, char** err);
void builder_add_string_value_triple(void* builder, char* subject, char* predicate, char* object, char** err);
_Bool builder_remove_id_triple(void* builder, uint64_t subject, uint64_t predicate, uint64_t object, char** err);
_Bool builder_remove_string_node_triple(void* builder, char* subject, char* predicate, char* object, char** err);
_Bool builder_remove_string_value_triple(void* builder, char* subject, char* predicate, char* object, char** err);
void* builder_commit(void* builder, char** err);

size_t layer_node_and_value_count(void* layer);
size_t layer_predicate_count(void* layer);

uint64_t layer_subject_id(void* layer, char* subject);
uint64_t layer_predicate_id(void* layer, char* predicate);
uint64_t layer_object_node_id(void* layer, char* object);
uint64_t layer_object_value_id(void* layer, char* value);

char* layer_id_subject(void* layer, uint64_t id);
char* layer_id_predicate(void* layer, uint64_t id);
char* layer_id_object(void* layer, uint64_t id, char* object_type);

void* layer_predicate_object_pairs_for_subject(void* layer, uint64_t subject);
void* layer_predicate_object_pairs_iter(void* layer);
void* predicate_object_pairs_iter_next(void* iter);
uint64_t predicate_object_pairs_subject(void* po_pairs);

void* predicate_object_pair_get_objects_for_predicate(void* po_pairs, uint64_t predicate);
void* predicate_object_pair_get_objects_iter(void* po_pairs);
uint64_t objects_subject(void* objects_for_po_pair);
uint64_t objects_predicate(void* objects_for_po_pair);
_Bool objects_has_object(void* objects_for_po_pair, uint64_t object);
void* objects_iter(void* objects_for_po_pair);
uint64_t objects_iter_next(void* objects_for_po_pair);
