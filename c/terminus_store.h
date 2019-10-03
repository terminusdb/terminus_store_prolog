#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool builder_add_id_triple(void *builder,
                           uint64_t subject,
                           uint64_t predicate,
                           uint64_t object,
                           const char **err);

void builder_add_string_node_triple(void *builder,
                                    const char *subject_ptr,
                                    const char *predicate_ptr,
                                    const char *object_ptr,
                                    const char **err);

void builder_add_string_value_triple(void *builder,
                                     const char *subject_ptr,
                                     const char *predicate_ptr,
                                     const char *object_ptr,
                                     const char **err);

const void *builder_commit(void *builder, const char **err);

bool builder_remove_id_triple(void *builder,
                              uint64_t subject,
                              uint64_t predicate,
                              uint64_t object,
                              const char **err);

bool builder_remove_string_node_triple(void *builder,
                                       const char *subject_ptr,
                                       const char *predicate_ptr,
                                       const char *object_ptr,
                                       const char **err);

bool builder_remove_string_value_triple(void *builder,
                                        const char *subject_ptr,
                                        const char *predicate_ptr,
                                        const char *object_ptr,
                                        const char **err);

void cleanup_cstring(char *cstring_ptr);

void cleanup_db(void *db);

void cleanup_layer(void *layer);

void cleanup_layer_builder(void *layer_builder);

void cleanup_store(void *store);

const void *create_database(void *store_ptr, const char *name, const char **err);

const void *database_get_head(void *database, const char **err);

const void *database_open_write(void *database, const char **err);

bool database_set_head(void *database, const void *layer_ptr, const char **err);

const char *layer_id_object(const void *layer, uint64_t id, uint8_t *object_type);

const char *layer_id_predicate(const void *layer, uint64_t id);

const char *layer_id_subject(const void *layer, uint64_t id);

uintptr_t layer_node_and_value_count(const void *layer);

uint64_t layer_object_node_id(const void *layer, const char *object);

uint64_t layer_object_value_id(const void *layer, const char *object);

const void *layer_open_write(void *layer, const char **err);

uintptr_t layer_predicate_count(const void *layer);

uint64_t layer_predicate_id(const void *layer, const char *predicate);

uint64_t layer_subject_id(const void *layer, const char *subject);

const void *open_database(void *store, const char *name, const char **err);

const void *open_directory_store(const char *dir);

const void *open_memory_store(void);

const void *store_create_base_layer(void *store, const char **err);
