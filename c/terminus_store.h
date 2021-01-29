#include <stdbool.h>

typedef struct {
  void *ptr;
  uintptr_t len;
  uintptr_t capacity;
} VecHandle;

typedef struct {
  uint64_t subject;
  uint64_t predicate;
} SubjectPredicatePair;

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
  uint32_t layer_id[5];
  uint32_t layer_parent_id[5];
  bool has_parent;
} LayerAndParent;

void add_csv_to_builder(char *name,
                        char *csv,
                        void *builder,
                        void *schema_builder,
                        char *data_prefix,
                        char *predicate_prefix,
                        int header,
                        int skip_header,
                        char **err);

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

void builder_apply_delta(void *builder, void *layer, char **err);

void builder_apply_diff(void *builder, void *layer, char **err);

void *builder_commit(void *builder, char **err);

bool builder_committed(void *builder);

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

void cleanup_cstring(char *cstring_ptr);

void cleanup_db(void *db);

void cleanup_layer(void *layer);

void cleanup_layer_and_parent_vec(VecHandle vec_handle);

void cleanup_layer_builder(void *layer_builder);

void cleanup_object_subject_predicates_iter(void *iter);

void cleanup_store(void *store);

void cleanup_subject_predicate_objects_iter(void *iter);

void cleanup_u64_iter(void *iter);

void cleanup_u64_pair_iter(void *iter);

void cleanup_u64_triple_iter(void *iter);

void cleanup_u8_vec(VecHandle vec_handle);

void *create_named_graph(void *store_ptr, char *name, char **err);

char *csv_iri(const char *csv_name, const char *prefix);

void *id_triple_addition_iter(void *layer, char **err);

void *id_triple_addition_o_iter(void *layer, uint64_t object, char **err);

void *id_triple_addition_p_iter(void *layer, uint64_t predicate, char **err);

void *id_triple_addition_s_iter(void *layer, uint64_t subject, char **err);

void *id_triple_addition_so_iter(void *layer, uint64_t subject, uint64_t object, char **err);

void *id_triple_addition_sp_iter(void *layer, uint64_t subject, uint64_t predicate, char **err);

bool id_triple_addition_spo_exists(void *layer,
                                   uint64_t subject,
                                   uint64_t predicate,
                                   uint64_t object,
                                   char **err);

void *id_triple_iter(void *layer);

void *id_triple_o_iter(void *layer, uint64_t object);

void *id_triple_p_iter(void *layer, uint64_t predicate);

void *id_triple_removal_iter(void *layer, char **err);

void *id_triple_removal_o_iter(void *layer, uint64_t object, char **err);

void *id_triple_removal_p_iter(void *layer, uint64_t predicate, char **err);

void *id_triple_removal_s_iter(void *layer, uint64_t subject, char **err);

void *id_triple_removal_so_iter(void *layer, uint64_t subject, uint64_t object, char **err);

void *id_triple_removal_sp_iter(void *layer, uint64_t subject, uint64_t predicate, char **err);

bool id_triple_removal_spo_exists(void *layer,
                                  uint64_t subject,
                                  uint64_t predicate,
                                  uint64_t object,
                                  char **err);

void *id_triple_s_iter(void *layer, uint64_t subject);

void *id_triple_so_iter(void *layer, uint64_t subject, uint64_t object);

void *id_triple_sp_iter(void *layer, uint64_t subject, uint64_t predicate);

bool id_triple_spo_exists(void *layer, uint64_t subject, uint64_t predicate, uint64_t object);

char *layer_builder_get_id(void *builder);

char *layer_get_id(void *layer);

char *layer_id_object(void *layer, uint64_t id, uint8_t *object_type);

char *layer_id_predicate(void *layer, uint64_t id);

char *layer_id_subject(void *layer, uint64_t id);

char *layer_id_to_string(const uint32_t *id);

uintptr_t layer_node_and_value_count(void *layer);

uint64_t layer_object_node_id(void *layer, char *object);

uint64_t layer_object_value_id(void *layer, char *object);

void *layer_open_write(void *layer, char **err);

void *layer_parent(void *layer, char **err);

uintptr_t layer_predicate_count(void *layer);

uint64_t layer_predicate_id(void *layer, char *predicate);

void layer_rollup(void *layer, char **err);

void *layer_squash(void *layer, char **err);

bool layer_string_to_id(const char *name_ptr, uint32_t (*result)[5], char **err);

uint64_t layer_subject_id(void *layer, char *subject);

uintptr_t layer_total_triple_addition_count(void *layer);

uintptr_t layer_total_triple_count(void *layer);

uintptr_t layer_total_triple_removal_count(void *layer);

uintptr_t layer_triple_addition_count(void *layer, char **err);

uintptr_t layer_triple_removal_count(void *layer, char **err);

bool named_graph_force_set_head(void *named_graph, void *layer_ptr, char **err);

void *named_graph_get_head(void *named_graph, char **err);

char *named_graph_get_name(void *named_graph);

void *named_graph_open_write(void *named_graph, char **err);

bool named_graph_set_head(void *named_graph, void *layer_ptr, char **err);

SubjectPredicatePair object_subject_predicate_pairs_iter_next(void *iter);

void *open_directory_store(char *dir);

void *open_memory_store(void);

void *open_named_graph(void *store, char *name, char **err);

VecHandle pack_export(void *store, const uint32_t (*layer_ids_ptr)[5], uintptr_t layer_ids_len);

void pack_import(void *store,
                 const uint8_t *pack_ptr,
                 uintptr_t pack_len,
                 const uint32_t (*layer_ids_ptr)[5],
                 uintptr_t layer_ids_len,
                 char **err);

VecHandle pack_layerids_and_parents(const uint8_t *pack_ptr, uintptr_t pack_len, char **err);

void *store_create_base_layer(void *store, char **err);

void *store_get_layer_from_id(void *store, char *id, char **err);

uint64_t subject_predicate_objects_iter_next(void *iter);

uint64_t u64_iter_next(void *iter);

U64Pair u64_pair_iter_next(void *iter);

U64Triple u64_triple_iter_next(void *iter);
