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
    cleanup_directory_store(store);
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

static int write_po_pairs_for_subject_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<po_pairs_for_subject>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_po_pairs_for_subject_blob(atom_t a) {
    void* po_pairs_for_subject = PL_blob_data(a, NULL, NULL);
    cleanup_po_pairs_for_subject(po_pairs_for_subject);
    return TRUE;
}

PL_blob_t po_pairs_for_subject_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "po_pairs_for_subject",
    &release_po_pairs_for_subject_blob,
    NULL,
    &write_po_pairs_for_subject_blob,
};

static int write_objects_blob(void *closure, atom_t a, int flags) {
    IOSTREAM *out = closure;
    char* contents = "#<objects_for_po_pair>";
    Sfwrite(contents, 1, strlen(contents), out);
    return TRUE;
}

static int release_objects_blob(atom_t a) {
    void* objects_for_po_pairs = PL_blob_data(a, NULL, NULL);
    cleanup_objects_for_po_pair(objects_for_po_pairs);
    return TRUE;
}

PL_blob_t objects_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "objects_for_po_pair",
    &release_objects_blob,
    NULL,
    &write_objects_blob,
};
