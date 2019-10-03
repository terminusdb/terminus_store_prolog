#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool builder_add_id_triple(void *builder,
                           uint64_t subject,
                           uint64_t predicate,
                           uint64_t object,
                           char **err);

void builder_add_string_node_triple(void *builder,
                                    char *subject_ptr,
                                    char *predicate_ptr,
                                    char *object_ptr,
                                    char **err);

void builder_add_string_value_triple(void *builder,
                                     char *subject_ptr,
                                     char *predicate_ptr,
                                     char *object_ptr,
                                     char **err);

void *builder_commit(void *builder, char **err);

bool builder_remove_id_triple(void *builder,
                              uint64_t subject,
                              uint64_t predicate,
                              uint64_t object,
                              char **err);

bool builder_remove_string_node_triple(void *builder,
                                       char *subject_ptr,
                                       char *predicate_ptr,
                                       char *object_ptr,
                                       char **err);

bool builder_remove_string_value_triple(void *builder,
                                        char *subject_ptr,
                                        char *predicate_ptr,
                                        char *object_ptr,
                                        char **err);

void cleanup_cstring(char *cstring_ptr);

void cleanup_db(void *db);

void cleanup_layer(void *layer);

void cleanup_layer_builder(void *layer_builder);

void cleanup_store(void *store);

void *create_database(void *store_ptr, char *name, char **err);

void *database_get_head(void *database, char **err);

void *database_open_write(void *database, char **err);

bool database_set_head(void *database, void *layer_ptr, char **err);

char *layer_id_object(void *layer, uint64_t id, uint8_t *object_type);

char *layer_id_predicate(void *layer, uint64_t id);

char *layer_id_subject(void *layer, uint64_t id);

uintptr_t layer_node_and_value_count(void *layer);

uint64_t layer_object_node_id(void *layer, char *object);

uint64_t layer_object_value_id(void *layer, char *object);

void *layer_open_write(void *layer, char **err);

uintptr_t layer_predicate_count(void *layer);

uint64_t layer_predicate_id(void *layer, char *predicate);

uint64_t layer_subject_id(void *layer, char *subject);

void *open_database(void *store, char *name, char **err);

void *open_directory_store(char *dir);

void *open_memory_store(void);

void *store_create_base_layer(void *store, char **err);
