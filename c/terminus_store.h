#include <stdbool.h>

typedef struct {
  uint64_t first;
  uint64_t second;
} U64Pair;

typedef struct {
  uint64_t first;
  uint64_t second;
  uint64_t third;
} U64Triple;

typedef struct {
  uint64_t subject;
  uint64_t predicate;
} SubjectPredicatePair;

typedef struct {
  void *ptr;
  uintptr_t len;
  uintptr_t capacity;
} VecHandle;

typedef struct {
  uint32_t layer_id[5];
  uint32_t layer_parent_id[5];
  bool has_parent;
} LayerAndParent;

void *open_memory_store(void);

void *open_directory_store(char *dir);

void *create_named_graph(void *store_ptr, char *name, char **err);

void *open_named_graph(void *store, char *name, char **err);

char *named_graph_get_name(void *named_graph);

void *named_graph_get_head(void *named_graph, char **err);

bool named_graph_set_head(void *named_graph, void *layer_ptr, char **err);

bool named_graph_force_set_head(void *named_graph, void *layer_ptr, char **err);

void *store_create_base_layer(void *store, char **err);

char *layer_builder_get_id(void *builder);

char *layer_get_id(void *layer);

void *layer_open_write(void *layer, char **err);

void *named_graph_open_write(void *named_graph, char **err);

void builder_add_id_triple(void *builder,
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

void builder_remove_id_triple(void *builder,
                              uint64_t subject,
                              uint64_t predicate,
                              uint64_t object,
                              char **err);

void builder_remove_string_node_triple(void *builder,
                                       char *subject_ptr,
                                       char *predicate_ptr,
                                       char *object_ptr,
                                       char **err);

void builder_remove_string_value_triple(void *builder,
                                        char *subject_ptr,
                                        char *predicate_ptr,
                                        char *object_ptr,
                                        char **err);

bool builder_committed(void *builder);

void *builder_commit(void *builder, char **err);

void builder_apply_delta(void *builder, void *layer, char **err);

void builder_apply_diff(void *builder, void *layer, char **err);

void *layer_parent(void *layer, char **err);

void *layer_squash(void *layer, char **err);

uintptr_t layer_node_and_value_count(void *layer);

uintptr_t layer_predicate_count(void *layer);

uint64_t layer_subject_id(void *layer, char *subject);

uint64_t layer_predicate_id(void *layer, char *predicate);

uint64_t layer_object_node_id(void *layer, char *object);

uint64_t layer_object_value_id(void *layer, char *object);

char *layer_id_subject(void *layer, uint64_t id);

char *layer_id_predicate(void *layer, uint64_t id);

char *layer_id_object(void *layer, uint64_t id, uint8_t *object_type);

uintptr_t layer_triple_addition_count(void *layer);

uintptr_t layer_triple_removal_count(void *layer);

uintptr_t layer_total_triple_addition_count(void *layer);

uintptr_t layer_total_triple_removal_count(void *layer);

uintptr_t layer_total_triple_count(void *layer);

void *layer_lookup_subject(void *layer, uint64_t subject);

void *layer_lookup_subject_addition(void *layer, uint64_t subject);

void *layer_lookup_subject_removal(void *layer, uint64_t subject);

void *layer_lookup_predicate(void *layer, uint64_t predicate);

void *layer_lookup_predicate_addition(void *layer, uint64_t predicate);

void *layer_lookup_predicate_removal(void *layer, uint64_t predicate);

void *layer_lookup_object(void *layer, uint64_t object);

void *layer_lookup_object_addition(void *layer, uint64_t object);

void *layer_lookup_object_removal(void *layer, uint64_t object);

void *layer_subjects_iter(void *layer);

void *layer_subject_additions_iter(void *layer);

void *layer_subject_removals_iter(void *layer);

void *subjects_iter_next(void *iter);

void *layer_predicates_iter(void *layer);

void *layer_predicate_additions_iter(void *layer);

void *layer_predicate_removals_iter(void *layer);

void *predicates_iter_next(void *iter);

void *layer_objects_iter(void *layer);

void *layer_object_additions_iter(void *layer);

void *layer_object_removals_iter(void *layer);

void *objects_iter_next(void *iter);

uint64_t subject_lookup_subject(void *subject_lookup);

void *subject_lookup_lookup_predicate(void *subject_lookup, uint64_t predicate);

void *subject_lookup_predicates_iter(void *subject_lookup);

void *subject_predicates_iter_next(void *iter);

uint64_t subject_predicate_lookup_subject(void *objects_for_po_pair);

uint64_t subject_predicate_lookup_predicate(void *objects_for_po_pair);

void *subject_predicate_lookup_objects_iter(void *objects);

uint64_t subject_predicate_objects_iter_next(void *iter);

bool subject_predicate_lookup_lookup_object(void *objects, uint64_t object);

uint64_t predicate_lookup_predicate(void *predicate_lookup);

void *predicate_lookup_subject_predicate_pairs_iter(void *predicate_lookup);

uint64_t object_lookup_object(void *object_lookup);

bool object_lookup_lookup_subject_predicate_pair(void *object_lookup,
                                                 uint64_t subject,
                                                 uint64_t predicate);

void *object_lookup_subject_predicate_pairs_iter(void *object_lookup);

bool id_triple_spo_exists(void *layer, uint64_t subject, uint64_t predicate, uint64_t object);

void *id_triple_sp_iter(void *layer, uint64_t subject, uint64_t predicate);

void *id_triple_so_iter(void *layer, uint64_t subject, uint64_t object);

void *id_triple_s_iter(void *layer, uint64_t subject);

void *id_triple_p_iter(void *layer, uint64_t predicate);

void *id_triple_o_iter(void *layer, uint64_t object);

void *id_triple_iter(void *layer);

bool id_triple_addition_spo_exists(void *layer,
                                   uint64_t subject,
                                   uint64_t predicate,
                                   uint64_t object);

void *id_triple_addition_sp_iter(void *layer, uint64_t subject, uint64_t predicate);

void *id_triple_addition_so_iter(void *layer, uint64_t subject, uint64_t object);

void *id_triple_addition_s_iter(void *layer, uint64_t subject);

void *id_triple_addition_p_iter(void *layer, uint64_t predicate);

void *id_triple_addition_o_iter(void *layer, uint64_t object);

void *id_triple_addition_iter(void *layer);

bool id_triple_removal_spo_exists(void *layer,
                                  uint64_t subject,
                                  uint64_t predicate,
                                  uint64_t object);

void *id_triple_removal_sp_iter(void *layer, uint64_t subject, uint64_t predicate);

void *id_triple_removal_so_iter(void *layer, uint64_t subject, uint64_t object);

void *id_triple_removal_s_iter(void *layer, uint64_t subject);

void *id_triple_removal_p_iter(void *layer, uint64_t predicate);

void *id_triple_removal_o_iter(void *layer, uint64_t object);

void *id_triple_removal_iter(void *layer);

uint64_t u64_iter_next(void *iter);

U64Pair u64_pair_iter_next(void *iter);

U64Triple u64_triple_iter_next(void *iter);

SubjectPredicatePair object_subject_predicate_pairs_iter_next(void *iter);

void *store_get_layer_from_id(void *store, char *id, char **err);

char *layer_id_to_string(const uint32_t *id);

bool layer_string_to_id(const char *name_ptr, uint32_t (*result)[5], char **err);

VecHandle pack_export(void *store, const uint32_t (*layer_ids_ptr)[5], uintptr_t layer_ids_len);

void pack_import(void *store,
                 const uint8_t *pack_ptr,
                 uintptr_t pack_len,
                 const uint32_t (*layer_ids_ptr)[5],
                 uintptr_t layer_ids_len,
                 char **err);

VecHandle pack_layerids_and_parents(const uint8_t *pack_ptr, uintptr_t pack_len, char **err);

void cleanup_store(void *store);

void cleanup_db(void *db);

void cleanup_layer(void *layer);

void cleanup_layer_builder(void *layer_builder);

void cleanup_subject_lookup(void *subject_lookup);

void cleanup_subject_predicate_lookup(void *objects_for_po_pair);

void cleanup_predicate_lookup(void *subject_lookup);

void cleanup_object_lookup(void *object_lookup);

void cleanup_subjects_iter(void *iter);

void cleanup_subject_predicates_iter(void *iter);

void cleanup_predicates_iter(void *iter);

void cleanup_objects_iter(void *iter);

void cleanup_subject_predicate_objects_iter(void *iter);

void cleanup_object_subject_predicates_iter(void *iter);

void cleanup_u64_triple_iter(void *iter);

void cleanup_u64_pair_iter(void *iter);

void cleanup_u64_iter(void *iter);

void cleanup_cstring(char *cstring_ptr);

void cleanup_u8_vec(VecHandle vec_handle);

void cleanup_layer_and_parent_vec(VecHandle vec_handle);

void add_csv_to_builder(char *name,
                        char *csv,
                        void *builder,
                        void *schema_builder,
                        char *data_prefix,
                        char *predicate_prefix,
                        int header,
                        int skip_header,
                        char **err);

char *csv_iri(const char *csv_name, const char *prefix);
