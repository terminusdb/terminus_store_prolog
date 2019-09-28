void hello_world();
size_t STORE_SIZE;
size_t DB_SIZE;
size_t LAYER_BUILDER_SIZE;
void* open_directory_store(char* dir_name);
void cleanup_directory_store(void* store);
void cleanup_db(void* db);
void cleanup_layer_builder(void* layer_builder);
void* create_database(char* name, void* store, char** err);
