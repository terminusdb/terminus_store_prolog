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
              predicate_id/3,
              object_id/3

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

object_id(Layer, Object, Id) :-
    ground(Id),
    !,
    id_to_object(Layer, Id, Object_Atom, Type),
    Object =.. [Type, Object_Atom].

object_id(Layer, node(Object), Id) :-
    ground(Object),
    !,
    object_node_to_id(Layer, Object, Id).

object_id(Layer, value(Object), Id) :-
    ground(Object),
    !,
    object_value_to_id(Layer, Object, Id).

object_id(Layer, Object, Id) :-
    node_and_value_count(Layer, Count),
    between(1, Count, Id),
    id_to_object(Layer, Id, Object_Atom, Type),
    Object =.. [Type, Object_Atom].

:- begin_tests(terminus_store).

:- use_module(library(filesex)).

clean :-
    delete_directory_and_contents("testdir").

test(open_directory_store_atom) :-
    open_directory_store(this_is_an_atom, _),
    open_directory_store("this is a string", _).

test(open_directory_store_atom_exception, [
         throws(error(type_error(atom,234), _))
     ]) :-
    open_directory_store(234, _).

test(create_db) :-
    make_directory("testdir"),
    open_directory_store("testdir", X),
    create_database(X, "sometestdb", _).

test(open_database) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", _).

test(head_from_empty_db, [fail]) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", DB),
    head(DB, _). % should be false because we have no HEAD yet

test(open_write_from_db_without_head, [
    throws(
        terminus_store_rust_error('Create a base layer first before opening the database for write')
    )]) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", DB),
    open_write(DB, _).

test(create_base_layer) :-
    open_directory_store("testdir", Store),
    open_write(Store, _).

test(write_value_triple) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object").

test(commit_and_set_header) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    open_database(Store, "sometestdb", DB),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object"),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer).

test(head_after_first_commit) :-
    open_directory_store("testdir", Store),
    open_database(Store, "sometestdb", DB),
    head(DB, _).

test(predicate_count) :-
    open_directory_store("testdir", Store),
    open_database(Store, "sometestdb", DB),
    head(DB, Layer),
    predicate_count(Layer, Count),
    Count == 1.

:- end_tests(terminus_store).
