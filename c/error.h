void throw_err(char* functor, char* err);
void throw_instantiation_err(term_t term);
void throw_type_error(term_t term, char* type);
void throw_rust_err(char* rust_err);

void* check_blob_type(term_t term, PL_blob_t* expected_type);
char* check_string_or_atom_term(term_t term);
