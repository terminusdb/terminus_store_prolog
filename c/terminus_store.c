#include <SWI-Prolog.h>
#include <stdio.h>
#include "terminus_store.h"

static PL_blob_t layer_builder_blob =
  {
   PL_BLOB_MAGIC,
   PL_BLOB_UNIQUE,
   "layer_builder",
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

static foreign_t pl_hello_world() {
  hello_world();
  return 0;
}


static PL_blob_t label_builder_blob =
  {
   PL_BLOB_MAGIC,
   PL_BLOB_UNIQUE,
   "label_builder",
   /*
   NULL,
   NULL,
   NULL,
   NULL,
   */
  };



install_t
install()
{
  PL_register_foreign("hello_world", 0,
                      pl_hello_world, 0);
}
