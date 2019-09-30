:- module(terminus_store, [
              open_directory_store/2,

              create_database/3,
              open_database/3,

              head/2,
              nb_set_head/2,

              open_write/2,

              nb_add_triple/4,
              nb_remove_triple/4,
              nb_commit/2,

              node_and_value_count/2,
              predicate_count/2,
              subject_id/3,
              predicate_id/3
              %object_id/3,

              %triple/4
          ]).

:- use_foreign_library(libterminus_store).

nb_add_triple(Builder, Subject, Predicate, Object) :-
    integer(Subject),
    integer(Predicate),
    integer(Object),
    !,
    nb_add_id_triple(Builder, Subject, Predicate, Object).

nb_add_triple(Builder, Subject, Predicate, node(Object)) :-
    !,
    nb_add_string_node_triple(Builder, Subject, Predicate, Object).

nb_add_triple(Builder, Subject, Predicate, value(Object)) :-
    !,
    nb_add_string_value_triple(Builder, Subject, Predicate, Object).

nb_add_triple(_,_,_,_) :-
    throw('triple must either be numeric, or object must be of format node(..) or value(..)').

nb_remove_triple(Builder, Subject, Predicate, Object) :-
    integer(Subject),
    integer(Predicate),
    integer(Object),
    !,
    nb_remove_id_triple(Builder, Subject, Predicate, Object).

nb_remove_triple(Builder, Subject, Predicate, node(Object)) :-
    !,
    nb_remove_string_node_triple(Builder, Subject, Predicate, Object).

nb_remove_triple(Builder, Subject, Predicate, value(Object)) :-
    !,
    nb_remove_string_value_triple(Builder, Subject, Predicate, Object).

nb_remove_triple(_,_,_,_) :-
    throw('triple must either be numeric, or object must be of format node(..) or value(..)').

subject_id(Layer, Subject, Id) :-
    ground(Id),
    !,
    id_to_subject(Layer, Id, Subject).

subject_id(Layer, Subject, Id) :-
    ground(Subject),
    !,
    subject_to_id(Layer, Subject, Id).

subject_id(Layer, Subject, Id) :-
    node_and_value_count(Layer, Count),
    between(1, Count, Id),
    id_to_subject(Layer, Id, Subject).

predicate_id(Layer, Predicate, Id) :-
    ground(Id),
    !,
    id_to_predicate(Layer, Id, Predicate).

predicate_id(Layer, Predicate, Id) :-
    ground(Predicate),
    !,
    predicate_to_id(Layer, Predicate, Id).

predicate_id(Layer, Predicate, Id) :-
    node_and_value_count(Layer, Count),
    between(1, Count, Id),
    id_to_predicate(Layer, Id, Predicate).

:- begin_tests(terminus_store).
test(open_directory_store_atom) :-
    open_directory_store(this_is_an_atom, _),
    open_directory_store("this is a string", _).

test(open_directory_store_atom_exception) :-
    catch(open_directory_store(234, _), E, true),
    print(E),
    E =@= type_error('We only accept a string or atom as dir_name').

test(create_db) :-
    (   exists_directory("testdir")
    ->  true
    ;   make_directory("testdir")),
    open_directory_store("testdir", X),
    create_database(X, "sometestdb", _).

:- end_tests(terminus_store).
