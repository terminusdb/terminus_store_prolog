int throw_err(char* functor, char* err);
int throw_instantiation_err(term_t term);
int throw_type_error(term_t term, char* type);
int throw_rust_err(char* rust_err);

void* check_blob_type(term_t term, PL_blob_t* expected_type);
char* check_string_or_atom_term(term_t term);
