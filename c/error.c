#include <assert.h>
#include <SWI-Prolog.h>
#include <SWI-Stream.h>
#include <stdio.h>
#include <string.h>
#include "terminus_store.h"

int throw_err(char* functor, char* err) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, functor, 1,
                         PL_UTF8_CHARS, err));

    return PL_throw(except);
}

int throw_instantiation_err(term_t term) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, "error", 2,
                         PL_UTF8_CHARS, "instantiation_error",
                         PL_TERM, term));

    return PL_throw(except);
}

int throw_type_error(term_t term, char* type) {
    term_t except = PL_new_term_ref();
    assert(PL_unify_term(except,
                         PL_FUNCTOR_CHARS, "error", 2,
                         PL_FUNCTOR_CHARS, "type_error", 2,
                         PL_UTF8_CHARS, type,
                         PL_TERM, term,
                         PL_VARIABLE));

    return PL_throw(except);
}

void* check_blob_type(term_t term, PL_blob_t* expected_type) {
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

char* check_atom_term(term_t term) {
    if (PL_term_type(term) == PL_VARIABLE) {
        throw_instantiation_err(term);
    }

    int term_type = PL_term_type(term);

    if (term_type != PL_ATOM) {
        throw_type_error(term, "atom");
    }

    char* result;
    assert(PL_get_chars(term, &result, CVT_ATOM | CVT_EXCEPTION | REP_UTF8));

    return result;
}

char* check_string_or_atom_term(term_t term) {
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

int throw_rust_err(char* rust_err) {
    term_t except = PL_new_term_ref();
    int unify_res = PL_unify_term(except,
                                  PL_FUNCTOR_CHARS, "terminus_store_rust_error", 1,
                                  PL_UTF8_CHARS, rust_err);

    cleanup_cstring(rust_err);

    assert(unify_res);
    return PL_throw(except);
}
