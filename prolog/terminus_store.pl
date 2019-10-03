:- module(terminus_store, [
              open_memory_store/1,
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
              object_id/3,

              triple_id/4,
              triple/4
          ]).

:- use_foreign_library(libterminus_store).

/*
 * nb_add_triple(+Builder, +Subject, +Predicate, +Object) is semidet
 *
 * Add a trible to the builder
 */
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


/*
 * nb_add_triple(+Builder, +Subject, +Predicate, +Object) is semidet
 *
 * Remove a trible from the builder
 */
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

/*
 * subject_id(+Layer, +Subject, -Id) is semidet
 *
 * Get the ID from a subject
 */
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


/*
 * predicate_id(+Layer, +Predicate, -Id) is semidet
 *
 * Get the ID from a predicate
 */
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


/*
 * object_id(+Layer, +Predicate, -Id) is semidet
 *
 * Get the ID from an object
 */
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

triple_id(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    ground(Object),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Subject_Predicate_Lookup),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

triple_id(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Predicate_Lookup),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

triple_id(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate_Lookup),
    subject_predicate_lookup_predicate(Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

triple_id(Layer, Subject, Predicate, Object) :-
    lookup_subject(Layer, Subject_Lookup),
    subject_lookup_subject(Subject_Lookup, Subject),
    subject_lookup_predicate(Subject_Lookup, Predicate_Lookup),
    subject_predicate_lookup_predicate(Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

triple(Layer, Subject, Predicate, Object) :-
    (   ground(Subject)
    ->  subject_id(Layer, Subject, S_Id)
    ;   true),
    
    (   ground(Predicate)
    ->  predicate_id(Layer, Predicate, P_Id)
    ;   true),
    
    (   ground(Object)
    ->  object_id(Layer, Object, O_Id)
    ;   true),

    triple_id(Layer, S_Id, P_Id, O_Id),

    (   ground(Subject)
    ->  true
    ;   subject_id(Layer, Subject, S_Id)),


    (   ground(Predicate)
    ->  true
    ;   predicate_id(Layer, Predicate, P_Id)),


    (   ground(Object)
    ->  true
    ;   object_id(Layer,Object, O_Id)).

:- begin_tests(terminus_store).

:- use_module(library(filesex)).

clean :-
    delete_directory_and_contents("testdir").

createdb() :-
    make_directory("testdir"),
    open_directory_store("testdir", X),
    create_database(X, "sometestdb", _).

create_memory_db(DB) :-
    open_memory_store(X),
    create_database(X, "sometestdb", DB).


test(open_memory_store) :-
    open_memory_store(_).

test(open_directory_store_atom) :-
    open_directory_store(this_is_an_atom, _),
    open_directory_store("this is a string", _).

test(open_directory_store_atom_exception, [
         throws(error(type_error(atom,234), _))
     ]) :-
    open_directory_store(234, _).

test(create_db, [cleanup(clean)]) :-
    make_directory("testdir"),
    open_directory_store("testdir", X),
    create_database(X, "sometestdb", _).


test(create_db_on_memory) :-
    open_memory_store(X),
    create_database(X, "sometestdb", _).

test(open_database, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", _).

test(open_database_memory) :-
    open_memory_store(X),
    create_database(X, "sometestdb", _),
    open_database(X, "sometestdb", _).

test(head_from_empty_db, [fail, cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", DB),
    head(DB, _). % should be false because we have no HEAD yet

test(head_from_empty_db_memory, [fail, setup(create_memory_db(DB))]) :-
     head(DB, _).

test(open_write_from_db_without_head, [
    cleanup(clean),
    setup(createdb),
    throws(
        terminus_store_rust_error('Create a base layer first before opening the database for write')
    )]) :-
    open_directory_store("testdir", X),
    open_database(X, "sometestdb", DB),
    open_write(DB, _).


test(open_write_from_memory_db_without_head, [
    setup(create_memory_db(DB)),
    throws(
        terminus_store_rust_error('Create a base layer first before opening the database for write')
    )]) :-
    open_write(DB, _).

test(create_base_layer, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, _).


test(create_base_layer_memory) :-
    open_memory_store(Store),
    open_write(Store, _).

test(write_value_triple, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object").

test(write_value_triple_memory) :-
    open_memory_store(Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object").

test(commit_and_set_header, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    open_database(Store, "sometestdb", DB),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer).


test(commit_and_set_header_memory) :-
    open_memory_store(Store),
    open_write(Store, Builder),
    create_database(Store, "sometestdb", DB),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer).

test(head_after_first_commit, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_database(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    head(DB, _).

test(predicate_count, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_database(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    head(DB, LayerHead),
    predicate_count(LayerHead, Count),
    Count == 1.

test(node_and_value_count, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    node_and_value_count(Layer, Count),
    Count == 2.

test(predicate_count_2, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_database(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_add_triple(Builder, "Subject2", "Predicate2", value("Object2")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    predicate_count(Layer, Count),
    Count == 2.

test(remove_triple, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    open_write(Layer, LayerBuilder),
    nb_remove_triple(LayerBuilder, "Subject", "Predicate", value("Object")).

test(triple_search_test, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(X, triple(Layer, 'Subject', 'Predicate', value(X)), Bag),
    Bag == ['Object'].


test(triple_search_test, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(Y-X, triple(Layer, 'Subject', Y, value(X)), Bag),
    Bag == ['Predicate'-'Object'].


test(triple_search_test, [cleanup(clean), setup(createdb)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(X-Y-Z, triple(Layer, X, Y, value(Z)), Bag),
    Bag == ['Subject'-'Predicate'-'Object'].

:- end_tests(terminus_store).
