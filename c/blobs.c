#include <assert.h>
#include <SWI-Stream.h>
#include <SWI-Prolog.h>
#include <stdio.h>
#include <string.h>
#include <stdatomic.h>
#include "terminus_store.h"
#include "error.h"

_Atomic uint64_t n_store_blobs = 0;
static void acquire_store_blob(atom_t a) {
    n_store_blobs++;
}

static int write_store_blob(IOSTREAM *out, atom_t a, int flags) {
    Sfprintf(out, "<store_blob>");
    return TRUE;
}

static int release_store_blob(atom_t a) {
    void* store = PL_blob_data(a, NULL, NULL);
    cleanup_store(store);
    n_store_blobs--;
    return TRUE;
}

PL_blob_t store_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "store",
    /*
      int           (*release)(atom_t a);
      int           (*compare)(atom_t a, atom_t b);
      int           (*write)(IOSTREAM *s, atom_t a, int flags);
      void          (*acquire)(atom_t a);
    */
    &release_store_blob,
    NULL,
    &write_store_blob,
    &acquire_store_blob,
};

_Atomic uint64_t n_named_graph_blobs = 0;
static void acquire_named_graph_blob(atom_t a) {
    n_named_graph_blobs++;
}

static int write_named_graph_blob(IOSTREAM *out, atom_t a, int flags) {
    void* named_graph = PL_blob_data(a, NULL, NULL);
    char* name = named_graph_get_name(named_graph);
    Sfprintf(out, "<named_graph %s>", name);
    cleanup_cstring(name);
    return TRUE;
}

static int release_named_graph_blob(atom_t a) {
    void* db = PL_blob_data(a, NULL, NULL);
    cleanup_db(db);
    n_named_graph_blobs--;
    return TRUE;
}

PL_blob_t named_graph_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "named_graph",
    /*
      int           (*release)(atom_t a);
      int           (*compare)(atom_t a, atom_t b);
      int           (*write)(IOSTREAM *s, atom_t a, int flags);
      void          (*acquire)(atom_t a);
    */
    &release_named_graph_blob,
    NULL,
    &write_named_graph_blob,
    &acquire_named_graph_blob,
};

_Atomic uint64_t n_layer_blobs = 0;
static void acquire_layer_blob(atom_t a) {
    n_layer_blobs++;
}

static int write_layer_blob(IOSTREAM *out, atom_t a, int flags) {
    void* layer = PL_blob_data(a, NULL, NULL);
    char* layer_id = layer_get_id(layer);
    Sfprintf(out, "<layer %s>", layer_id);
    cleanup_cstring(layer_id);
    return TRUE;
}

static int release_layer_blob(atom_t a) {
    void* layer = PL_blob_data(a, NULL, NULL);
    cleanup_layer(layer);
    n_layer_blobs--;
    return TRUE;
}

static int compare_layer_blob(atom_t a, atom_t b) {
    void* layer_a = PL_blob_data(a, NULL, NULL);
    void* layer_b = PL_blob_data(b, NULL, NULL);
    char* layer_id_a = layer_get_id(layer_a);
    char* layer_id_b = layer_get_id(layer_b);
    int compare = strcmp(layer_id_a, layer_id_b);
    cleanup_cstring(layer_id_a);
    cleanup_cstring(layer_id_b);
    return compare;
}


PL_blob_t layer_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "layer",
    &release_layer_blob,
    &compare_layer_blob,
    &write_layer_blob,
    &acquire_layer_blob,
};

_Atomic uint64_t n_layer_builder_blobs = 0;
static void acquire_layer_builder_blob(atom_t a) {
    n_layer_builder_blobs++;
}

static int write_layer_builder_blob(IOSTREAM *out, atom_t a, int flags) {
    void* builder = PL_blob_data(a, NULL, NULL);
    char* layer_id = layer_builder_get_id(builder);
    Sfprintf(out, "<builder %s>", layer_id);
    cleanup_cstring(layer_id);
    return TRUE;
}

static int release_layer_builder_blob(atom_t a) {
    void* builder = PL_blob_data(a, NULL, NULL);
    cleanup_layer_builder(builder);
    n_layer_builder_blobs--;
    return TRUE;
}

PL_blob_t layer_builder_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "layer_builder",
    &release_layer_builder_blob,
    NULL,
    &write_layer_builder_blob,
    &acquire_layer_builder_blob,
};

