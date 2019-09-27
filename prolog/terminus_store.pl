:- module(terminus_store, [
              open_directory_store/2
          ]).

:- use_foreign_library(libterminus_store).

:- begin_tests(terminus_store).
test(open_directory_store_atom) :-
    open_directory_store(this_is_an_atom, _),
    open_directory_store("this is a string", _).

test(open_directory_store_atom_exception) :-
    catch(open_directory_store(234, _), E, true),
    print(E),
    E =@= type_error('We only accept a string or atom as dir_name').


:- end_tests(terminus_store).
