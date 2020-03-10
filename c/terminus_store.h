#include <stdbool.h>

typedef struct {
  uint64_t subject;
  uint64_t predicate;
} SubjectPredicatePair;

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

extern void c_debug_via_prolog(const char *topic, const char *comment);

extern void c_log_via_prolog(const char *comment);

void cleanup_cstring(char *cstring_ptr);

void cleanup_db(void *db);

void cleanup_layer(void *layer);

void cleanup_layer_builder(void *layer_builder);

void cleanup_object_lookup(void *object_lookup);

void cleanup_object_subject_predicates_iter(void *iter);

void cleanup_objects_iter(void *iter);

void cleanup_predicate_lookup(void *subject_lookup);

void cleanup_predicates_iter(void *iter);

void cleanup_store(void *store);

void cleanup_subject_lookup(void *subject_lookup);

void cleanup_subject_predicate_lookup(void *objects_for_po_pair);

void cleanup_subject_predicate_objects_iter(void *iter);

void cleanup_subject_predicates_iter(void *iter);

void cleanup_subjects_iter(void *iter);

void *create_named_graph(void *store_ptr, char *name, char **err);

void deserialize_directory_store(char *tar_path, char *extract_path);

char *layer_builder_get_id(void *builder);

char *layer_get_id(void *layer);

char *layer_id_object(void *layer, uint64_t id, uint8_t *object_type);

char *layer_id_predicate(void *layer, uint64_t id);

char *layer_id_subject(void *layer, uint64_t id);

void *layer_lookup_object(void *layer, uint64_t object);

void *layer_lookup_object_addition(void *layer, uint64_t object);

void *layer_lookup_object_removal(void *layer, uint64_t object);

void *layer_lookup_predicate(void *layer, uint64_t predicate);

void *layer_lookup_predicate_addition(void *layer, uint64_t predicate);

void *layer_lookup_predicate_removal(void *layer, uint64_t predicate);

void *layer_lookup_subject(void *layer, uint64_t subject);

void *layer_lookup_subject_addition(void *layer, uint64_t subject);

void *layer_lookup_subject_removal(void *layer, uint64_t subject);

uintptr_t layer_node_and_value_count(void *layer);

void *layer_object_additions_iter(void *layer);

uint64_t layer_object_node_id(void *layer, char *object);

void *layer_object_removals_iter(void *layer);

uint64_t layer_object_value_id(void *layer, char *object);

void *layer_objects_iter(void *layer);

void *layer_open_write(void *layer, char **err);

void *layer_parent(void *layer);

void *layer_predicate_additions_iter(void *layer);

uintptr_t layer_predicate_count(void *layer);

uint64_t layer_predicate_id(void *layer, char *predicate);

void *layer_predicate_removals_iter(void *layer);

void *layer_predicates_iter(void *layer);

void *layer_subject_additions_iter(void *layer);

uint64_t layer_subject_id(void *layer, char *subject);

void *layer_subject_removals_iter(void *layer);

void *layer_subjects_iter(void *layer);

void *named_graph_get_head(void *named_graph, char **err);

char *named_graph_get_name(void *named_graph);

void *named_graph_open_write(void *named_graph, char **err);

bool named_graph_set_head(void *named_graph, void *layer_ptr, char **err);

bool object_lookup_lookup_subject_predicate_pair(void *object_lookup,
                                                 uint64_t subject,
                                                 uint64_t predicate);

uint64_t object_lookup_object(void *object_lookup);

void *object_lookup_subject_predicate_pairs_iter(void *object_lookup);

SubjectPredicatePair object_subject_predicate_pairs_iter_next(void *iter);

void *objects_iter_next(void *iter);

void *open_directory_store(char *dir);

void *open_memory_store(void);

void *open_named_graph(void *store, char *name, char **err);

uint64_t predicate_lookup_predicate(void *predicate_lookup);

void *predicate_lookup_subject_predicate_pairs_iter(void *predicate_lookup);

void *predicates_iter_next(void *iter);

void rust_install_prolog_debug_hook(void);

void rust_install_prolog_log_hook(void);

int serialize_directory_store(char *dir,
                              char **label_names,
                              int label_list_length,
                              char **layer_ids,
                              int layer_id_length,
                              char *filename);

void *store_create_base_layer(void *store, char **err);

void *store_get_layer_from_id(void *store, char *id, char **err);

void *subject_lookup_lookup_predicate(void *subject_lookup, uint64_t predicate);

void *subject_lookup_predicates_iter(void *subject_lookup);

uint64_t subject_lookup_subject(void *subject_lookup);

bool subject_predicate_lookup_lookup_object(void *objects, uint64_t object);

void *subject_predicate_lookup_objects_iter(void *objects);

uint64_t subject_predicate_lookup_predicate(void *objects_for_po_pair);

uint64_t subject_predicate_lookup_subject(void *objects_for_po_pair);

uint64_t subject_predicate_objects_iter_next(void *iter);

void *subject_predicates_iter_next(void *iter);

void *subjects_iter_next(void *iter);
