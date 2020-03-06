#include <SWI-Prolog.h>

/** Calculate the length of a prolog list */
size_t calculate_pl_list_len(term_t pl_list) {
    // will return -1 on a error, should probably return a struct instead
    // PL_skip_list(term_t +list, term_t -tail, size_t *len) is a better alternative...
    if (!PL_is_list(pl_list)) {
        return -1;
    }
    size_t length;
    term_t tail = PL_new_term_ref();
    PL_skip_list(pl_list, tail, &length);
    return length;
}
