#include <assert.h>
#include <SWI-Prolog.h>
#include <SWI-Stream.h>
#include <stdio.h>
#include <string.h>
#include "terminus_store.h"


static int write_store_blob(void *closure, atom_t a, int flags) {
  IOSTREAM *out = closure;
  Sfwrite("test", 1, strlen("test"), out);
  return TRUE;
}

static int release_store_blob(atom_t a) {
  void* store = PL_blob_data(a, NULL, NULL);
  cleanup_directory_store(store);
  return TRUE;
}

static PL_blob_t store_blob =
  {
   PL_BLOB_MAGIC,
   0,
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

static term_t check_string_or_atom_term(term_t to_check) {
  int term_type = PL_term_type(to_check);


  if (term_type != PL_ATOM && term_type != PL_STRING) {
    term_t except = PL_new_term_ref();
    int unify_res = PL_unify_term(except,
                                  PL_FUNCTOR_CHARS, "type_error", 1,
                                  PL_CHARS, "We only accept a string or atom as dir_name");
    assert(unify_res);
    PL_throw(except);
  }
}

static foreign_t pl_open_directory_store(term_t dir_name, term_t store_term) {
  if (PL_term_type(store_term) != PL_VARIABLE) {
    PL_fail;
  }
  check_string_or_atom_term(dir_name);

  char* dir_name_char;
  assert(PL_get_chars(dir_name, &dir_name_char, CVT_ATOM | CVT_STRING | CVT_EXCEPTION | REP_UTF8));
  void* store_ptr = open_directory_store(dir_name_char);
  PL_unify_blob(store_term, store_ptr, STORE_SIZE, &store_blob);
  PL_succeed;
}

static PL_blob_t database_blob =
  {
   PL_BLOB_MAGIC,
   PL_BLOB_UNIQUE,
   "database",
   /*
     int           (*release)(atom_t a);
     int           (*compare)(atom_t a, atom_t b);
     int           (*write)(IOSTREAM *s, atom_t a, int flags);
     void          (*acquire)(atom_t a);
   */
   NULL,
   NULL,
   NULL,
   NULL,
  };


static PL_blob_t layer_blob =
  {
   PL_BLOB_MAGIC,
   0,
   "layer",
   /*
     NULL,
     NULL,
     NULL,
     NULL,
   */
  };

static PL_blob_t layer_builder_blob =
  {
   PL_BLOB_MAGIC,
   PL_BLOB_UNIQUE,
   "layer_builder",
   /*
     NULL,
     NULL,
     NULL,
     NULL,
   */
  };



static foreign_t pl_hello_world() {
  hello_world();
  return 0;
}


install_t
install()
{
  PL_register_foreign("open_directory_store", 2,
                      pl_open_directory_store, 0);
}
