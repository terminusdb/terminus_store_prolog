extern PL_blob_t store_blob_type;
extern PL_blob_t named_graph_blob_type;
extern PL_blob_t layer_blob_type;
extern PL_blob_t layer_builder_blob_type;
extern PL_blob_t subject_lookup_blob_type;
extern PL_blob_t subject_predicate_lookup_blob_type;
extern PL_blob_t predicate_lookup_blob_type;
extern PL_blob_t object_lookup_blob_type;

extern _Atomic uint64_t n_store_blobs;
extern _Atomic uint64_t n_named_graph_blobs;
extern _Atomic uint64_t n_layer_blobs;
extern _Atomic uint64_t n_layer_builder_blobs;
extern _Atomic uint64_t n_subject_lookup_blobs;
extern _Atomic uint64_t n_subject_predicate_lookup_blobs;
extern _Atomic uint64_t n_predicate_lookup_blobs;
extern _Atomic uint64_t n_object_lookup_blobs;
