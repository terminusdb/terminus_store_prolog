:- module(terminus_store, [
              open_directory_store/2
          ]).

:- use_foreign_library(libterminus_store).

:- begin_tests(terminus_store).
test(open_directory_store_atom) :-
    open_directory_store(this_is_an_atom),
    open_directory_store("this is a string").


:- end_tests(terminus_store).
