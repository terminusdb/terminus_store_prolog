void* open_directory_store(char* dir_name);
void cleanup_directory_store(void* store);
void cleanup_db(void* db);
void cleanup_layer(void* layer);
void cleanup_layer_builder(void* layer_builder);
void cleanup_cstring(char* c_string);
void* create_database(char* name, void* store, char** err);
void* database_get_head(void* db, char** err);
