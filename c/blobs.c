#include <assert.h>
#include <SWI-Prolog.h>
#include <SWI-Stream.h>
#include <stdio.h>
#include <string.h>
#include "terminus_store.h"
#include "error.h"

static int write_store_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<store_blob>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_store_blob(atom_t a) {
    void* store = PL_blob_data(a, NULL, NULL);
    cleanup_store(store);
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
    NULL,
};


static int write_database_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<database>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_database_blob(atom_t a) {
    void* db = PL_blob_data(a, NULL, NULL);
    cleanup_db(db);
    return TRUE;
}

PL_blob_t database_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "database",
    /*
      int           (*release)(atom_t a);
      int           (*compare)(atom_t a, atom_t b);
      int           (*write)(IOSTREAM *s, atom_t a, int flags);
      void          (*acquire)(atom_t a);
    */
    &release_database_blob,
    NULL,
    &write_database_blob,
    NULL,
};

static int write_layer_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<layer>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_layer_blob(atom_t a) {
    void* layer = PL_blob_data(a, NULL, NULL);
    cleanup_layer(layer);
    return TRUE;
}


PL_blob_t layer_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "layer",
    &release_layer_blob,
    NULL,
    &write_layer_blob,
    NULL,
};

static int write_layer_builder_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<layer_builder>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_layer_builder_blob(atom_t a) {
    void* builder = PL_blob_data(a, NULL, NULL);
    cleanup_layer_builder(builder);
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
};

static int write_subject_lookup_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<subject_lookup>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_subject_lookup_blob(atom_t a) {
    void* subject_lookup = PL_blob_data(a, NULL, NULL);
    cleanup_subject_lookup(subject_lookup);
    return TRUE;
}

PL_blob_t subject_lookup_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "subject_lookup",
    &release_subject_lookup_blob,
    NULL,
    &write_subject_lookup_blob,
};

static int write_subject_predicate_lookup_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<subject_predicate_lookup>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_subject_predicate_lookup_blob(atom_t a) {
    void* subject_predicate_lookups = PL_blob_data(a, NULL, NULL);
    cleanup_subject_predicate_lookup(subject_predicate_lookups);
    return TRUE;
}

PL_blob_t subject_predicate_lookup_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "subject_predicate_lookup",
    &release_subject_predicate_lookup_blob,
    NULL,
    &write_subject_predicate_lookup_blob,
};
