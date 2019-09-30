#include <assert.h>
#include <SWI-Prolog.h>
#include <SWI-Stream.h>
#include <stdio.h>
#include <string.h>
#include "terminus_store.h"

static void throw_err(char* functor, char* err) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, functor, 1,
                         PL_UTF8_CHARS, err));

    PL_throw(except);
}

static void throw_instantiation_err(term_t term) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, "error", 2,
                         PL_UTF8_CHARS, "instantiation_error",
                         PL_TERM, term));

    PL_throw(except);
}

static void throw_type_error(term_t term, char* type) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, "error", 2,
                         PL_FUNCTOR_CHARS, "type_error", 2,
                         PL_UTF8_CHARS, type,
                         PL_TERM, term,
                         PL_VARIABLE));

    PL_throw(except);
}

static void* check_blob_type(term_t term, PL_blob_t* expected_type) {
    if (PL_term_type(term) == PL_VARIABLE) {
        throw_instantiation_err(term);
    }

    void* blob;
    PL_blob_t *type;
    if (!PL_get_blob(term, &blob, NULL, &type) || type != expected_type) {
        throw_type_error(term, expected_type->name);
    }

    return blob;
}

static char* check_string_or_atom_term(term_t term) {
    if (PL_term_type(term) == PL_VARIABLE) {
        throw_instantiation_err(term);
    }

    int term_type = PL_term_type(term);

    if (term_type != PL_ATOM && term_type != PL_STRING) {
        throw_type_error(term, "atom");
    }

    char* result;
    assert(PL_get_chars(term, &result, CVT_ATOM | CVT_STRING | CVT_EXCEPTION | REP_UTF8));

    return result;
}

static void throw_rust_err(char* rust_err) {
    term_t except = PL_new_term_ref();
    int unify_res = PL_unify_term(except,
                                  PL_FUNCTOR_CHARS, "terminus_store_rust_error", 1,
                                  PL_UTF8_CHARS, rust_err);

    cleanup_cstring(rust_err);

    assert(unify_res);
    PL_throw(except);
}


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

static PL_blob_t store_blob_type =
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

static PL_blob_t database_blob_type =
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


static PL_blob_t layer_blob_type =
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

static PL_blob_t layer_builder_blob_type =
{
    PL_BLOB_MAGIC,
    PL_BLOB_NOCOPY,
    "layer_builder",
    &release_layer_builder_blob,
    NULL,
    &write_layer_builder_blob,
};


static foreign_t pl_open_directory_store(term_t dir_name_term, term_t store_term) {
    if (PL_term_type(store_term) != PL_VARIABLE) {
        PL_fail;
    }
    char* dir_name = check_string_or_atom_term(dir_name_term);
    void* store_ptr = open_directory_store(dir_name);
    PL_unify_blob(store_term, store_ptr, 0, &store_blob_type);
    PL_succeed;
}

static foreign_t pl_create_database(term_t store_blob, term_t db_name_term, term_t db_term) {
    void* store = check_blob_type(store_blob, &store_blob_type);
    char* db_name = check_string_or_atom_term(db_name_term);

    if (PL_term_type(db_term) != PL_VARIABLE) {
        PL_fail;
    }

    char* err;
    void* db_ptr = create_database(store, db_name, &err);
    // Decent error handling, not only checking for null
    if (db_ptr == NULL) {
        throw_rust_err(err);
    }
    PL_unify_blob(db_term, db_ptr, 0, &database_blob_type);
    PL_succeed;
}

static foreign_t pl_open_database(term_t store_blob, term_t db_name_term, term_t db_term) {
    void* store = check_blob_type(store_blob, &store_blob_type);
    char* db_name = check_string_or_atom_term(db_name_term);

    if (PL_term_type(db_term) != PL_VARIABLE) {
        PL_fail;
    }

    char* err;
    void* db_ptr = open_database(store, db_name, &err);
    if (db_ptr == NULL) {
        if (err != NULL) {
            throw_rust_err(err);
        }
        PL_fail;
    }
    else {
        PL_unify_blob(db_term, db_ptr, 0, &database_blob_type);
        PL_succeed;
    }
}

static foreign_t pl_head(term_t database_blob_term, term_t layer_term) {
    void* database = check_blob_type(database_blob_term, &database_blob_type);

    char* err;
    void* layer_ptr = database_get_head(database, &err);
    if (layer_ptr == NULL) {
        if (err == NULL) {
            PL_fail;
        }
        else {
            throw_rust_err(err);
        }
    }
    else {
        PL_unify_blob(layer_term, layer_ptr, 0, &layer_blob_type);
    }

    PL_succeed;
}

static foreign_t pl_set_head(term_t database_blob_term, term_t layer_blob_term) {
    void* database = check_blob_type(database_blob_term, &database_blob_type);
    void* layer = check_blob_type(layer_blob_term, &layer_blob_type);

    char* err;
    if (database_set_head(database, layer, &err)) {
        PL_succeed;
    }
    else {
        PL_fail;
    }
}

static foreign_t pl_open_write(term_t layer_or_database_term, term_t builder_term) {
    if (PL_term_type(layer_or_database_term) == PL_VARIABLE) {
        throw_instantiation_err(layer_or_database_term);
    }

    PL_blob_t* blob_type;
    void* blob;
    if (!PL_get_blob(layer_or_database_term, &blob, NULL, &blob_type) || (blob_type != &store_blob_type && blob_type != &layer_blob_type)) {
        throw_type_error(layer_or_database_term, "layer");
    }

    if (PL_term_type(builder_term) != PL_VARIABLE) {
        PL_fail;
    }


    char* err;
    void* builder_ptr;
    if (blob_type == &store_blob_type) {
        builder_ptr = store_create_base_layer(blob, &err);
    }
    else if (blob_type == &layer_blob_type) {
        builder_ptr = layer_open_write(blob, &err);
    }
    else {
        abort();
    }

    if (builder_ptr == NULL) {
        assert(err);
        throw_rust_err(err);
    }
    else {
        PL_unify_blob(builder_term, builder_ptr, 0, &layer_builder_blob_type);
    }

    PL_succeed;
}

static foreign_t pl_add_id_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    int64_t subject, predicate, object;
    PL_get_int64_ex(subject_term, &subject);
    PL_get_int64_ex(predicate_term, &subject);
    PL_get_int64_ex(object_term, &subject);

    char *err;
    int result = builder_add_id_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    if (result) {
        PL_succeed;
    }
    else {
        PL_fail;
    }
}

static foreign_t pl_add_string_node_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    char* subject = check_string_or_atom_term(subject_term);
    char* predicate = check_string_or_atom_term(predicate_term);
    char* object = check_string_or_atom_term(object_term);

    char *err;
    builder_add_string_node_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    PL_succeed;
}

static foreign_t pl_add_string_value_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    char* subject = check_string_or_atom_term(subject_term);
    char* predicate = check_string_or_atom_term(predicate_term);
    char* object = check_string_or_atom_term(object_term);

    char *err;
    builder_add_string_value_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    PL_succeed;
}

static foreign_t pl_remove_id_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    int64_t subject, predicate, object;
    PL_get_int64_ex(subject_term, &subject);
    PL_get_int64_ex(predicate_term, &subject);
    PL_get_int64_ex(object_term, &subject);

    char *err;
    int result = builder_remove_id_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    if (result) {
        PL_succeed;
    }
    else {
        PL_fail;
    }
}

static foreign_t pl_remove_string_node_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    char* subject = check_string_or_atom_term(subject_term);
    char* predicate = check_string_or_atom_term(predicate_term);
    char* object = check_string_or_atom_term(object_term);

    char *err;
    int result = builder_remove_string_node_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    if (result) {
        PL_succeed;
    }
    else {
        PL_fail;
    }
}

static foreign_t pl_remove_string_value_triple(term_t builder_term, term_t subject_term, term_t predicate_term, term_t object_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);
    char* subject = check_string_or_atom_term(subject_term);
    char* predicate = check_string_or_atom_term(predicate_term);
    char* object = check_string_or_atom_term(object_term);

    char *err;
    int result = builder_remove_string_value_triple(builder, subject, predicate, object, &err);
    if (err != NULL) {
        throw_rust_err(err);
    }

    if (result) {
        PL_succeed;
    }
    else {
        PL_fail;
    }
}

static foreign_t pl_builder_commit(term_t builder_term, term_t layer_term) {
    void* builder = check_blob_type(builder_term, &layer_builder_blob_type);

    char* err;
    void* layer_ptr = builder_commit(builder, &err);
    if (layer_ptr == NULL) {
        throw_rust_err(err);
    }

    PL_unify_blob(layer_term, layer_ptr, 0, &layer_blob_type);
}

install_t
install()
{
    PL_register_foreign("create_database", 3,
                        pl_create_database, 0);
    PL_register_foreign("open_database", 3,
                        pl_open_database, 0);
    PL_register_foreign("open_directory_store", 2,
                        pl_open_directory_store, 0);
    PL_register_foreign("head", 2,
                        pl_head, 0);
    PL_register_foreign("nb_set_head", 2,
                        pl_set_head, 0);
    PL_register_foreign("open_write", 2,
                        pl_open_write, 0);
    PL_register_foreign("nb_add_id_triple", 4,
                        pl_add_id_triple, 0);
    PL_register_foreign("nb_add_string_node_triple", 4,
                        pl_add_string_node_triple, 0);
    PL_register_foreign("nb_add_string_value_triple", 4,
                        pl_add_string_value_triple, 0);
    PL_register_foreign("nb_remove_id_triple", 4,
                        pl_remove_id_triple, 0);
    PL_register_foreign("nb_remove_string_node_triple", 4,
                        pl_remove_string_node_triple, 0);
    PL_register_foreign("nb_remove_string_value_triple", 4,
                        pl_remove_string_value_triple, 0);
    PL_register_foreign("nb_commit", 2,
                        pl_builder_commit, 0);
}
