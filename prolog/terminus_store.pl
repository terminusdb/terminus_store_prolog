:- module(terminus_store, [
              open_memory_store/1,
              open_directory_store/2,
              deserialize_database/2,
              serialize_database/4,
              create_named_graph/3,
              open_named_graph/3,

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

              id_triple/4,
              triple/4,

              id_triple_addition/4,
              triple_addition/4,

              id_triple_removal/4,
              triple_removal/4,

              parent/2,

              layer_to_id/2,
              store_id_layer/3]).

:- use_foreign_library(foreign(libterminus_store)).

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%% pldocs for the foreign predicates %%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%! open_memory_store(-Store:store) is det
%
% Opens an in-memory store and unifies it with Store.
%
% @arg Store the returned in-memory store.

%! open_directory_store(+Path:text, -Store:store) is det.
%
% Opens a store backed by a directory, and unifies it with Store.
%
% This predicate does not check if the directory actually exists, but
% other store-related predicates will error when used with a store
% backed by a non-existent directory.
%
% @arg Path a file system path to the store directory. This can be either absolute and relative.
% @arg Store the returned directory store.

%! create_named_graph(+Store:store, +Name:text, -Graph:named_graph) is det.
%
% Create a new named graph with the given name, and unifies it with Graph.
%
% @arg Store the store to create the graph in.
% @arg Name the name which the new graph should have.
% @arg Graph the returned named graph.
% @throws if a graph with the given name already exists.

%! open_named_graph(+Store:store, +Name:text, -Graph:named_graph) is semidet.
%
% Opens an existing named graph with the given name.
%
% Fails if no graph with that name exists.
%
% @arg Store the store to create the graph in.
% @arg Name the name of the graph to be opened.
% @arg Graph the returned named graph.

%! head(+Graph:named_graph, -Layer:layer) is semidet.
%
% Retrieve the layer that a named graph points at.
% This is the equivalent of opening a read transaction with snapshot isolation on a named graph.
%
% Fails if the given graph has no head yet.
%
% @arg Graph the named graph to retrieve the head layer from.
% @arg Layer the returned head layer.

%! nb_set_head(+Graph:named_graph, +Layer:layer) is semidet.
%
% Set the given layer as the new head of the given graph.
%
% Fails if the new layer is not a proper child of the current head.
%
% This predicate does not support backtracking.
%
% @arg Graph the named graph to set the head layer of.
% @arg Layer the layer to make the new head of the graph.

%! open_write(+Store_Or_Layer:term, -Builder:layer_builder) is det.
%
% Creates a layer builder from either a parent layer, or a store.
%
% When Store_Or_Layer is a store, the resulting builder will create a
% base layer.
%
% When Store_Or_Layer is a layer, the resulting builder will create a
% child layer whose parent is the given layer.
%
% @arg Store_Or_layer a store when creating a new base layer, or the parent layer when creating a child layer.
% @arg Builder a layer builder to create the new layer.

%! nb_add_id_triple(+Builder:layer_builder, +Subject_Id:integer, +Predicate_Id:integer, +Object_Id: integer) is semidet.
%
% Add the given subject, predicate and object as a triple to the builder object.
%
% This fails if any of the Ids is out of range, or if the triple
% already exists, either in this builder or in a parent layer.
%
% @arg Builder the builder object to add this triple to.
% @arg Subject_Id the id of the triple subject.
% @arg Predicate_Id the id of the triple predicate.
% @arg Object_Id the id of the triple object.

%! nb_add_string_node_triple(+Builder:layer_builder, +Subject:text, +Predicate:text, +Object:text) is semidet.
%
% Add the given subject, predicate, and object as a triple to the
% builder object. The object is interpreted as pointing at a node,
% rather than being a literal value.
%
% This fails if the triple already exists in this builder object or a parent layer.
%
% @arg Builder the builder object to add this triple to.
% @arg Subject the triple subject.
% @arg Predicate the triple predicate.
% @arg Object the triple object, which is interpreted as a node.

%! nb_add_string_value_triple(+Builder:layer_builder, +Subject:text, +Predicate:text, +Object:text) is semidet.
%
% Add the given subject, predicate, and object as a triple to the
% builder object. The object is interpreted as a value, rather than a node.
%
% This fails if the triple already exists in this builder object or a parent layer.
%
% @arg Builder the builder object to add this triple to.
% @arg Subject the triple subject.
% @arg Predicate the triple predicate.
% @arg Object the triple object, which is interpreted as a value.

%! nb_remove_id_triple(+Builder:layer_builder, +Subject_Id:integer, +Predicate_Id:integer, +Object_Id: integer) is semidet.
%
% Add the given subject, predicate and object as a triple removal to the builder object.
%
% This fails if any of the Ids is out of range, or if the triple does
% not exist in a parent layer, or if the removal has already been
% registered in this builder.
%
% @arg Builder the builder object to add this triple removal to.
% @arg Subject_Id the id of the triple subject.
% @arg Predicate_Id the id of the triple predicate.
% @arg Object_Id the id of the triple object.

%! nb_remove_string_node_triple(+Builder:layer_builder, +Subject:text, +Predicate:text, +Object:text) is semidet.
%
% Add the given subject, predicate, and object as a triple removal to the
% builder object. The object is interpreted as pointing at a node,
% rather than being a literal value.
%
% This fails if the triple does not exist in a parent layer, or if the
% removal has already been registered in this builder.
%
% @arg Builder the builder object to add this triple removal to.
% @arg Subject the triple subject.
% @arg Predicate the triple predicate.
% @arg Object the triple object, which is interpreted as a node.

%! nb_remove_string_value_triple(+Builder:layer_builder, +Subject:text, +Predicate:text, +Object:text) is semidet.
%
% Add the given subject, predicate, and object as a triple removal to
% the builder object. The object is interpreted as a value, rather
% than a node.
%
% This fails if the triple does not exist in a parent layer, or if the
% removal has already been registered in this builder.
%
% @arg Builder the builder object to add this triple removal to.
% @arg Subject the triple subject.
% @arg Predicate the triple predicate.
% @arg Object the triple object, which is interpreted as a node.

%! nb_commit(+Builder:layer_builder, -Layer:layer) is det.
%
% Commit the layer builder, turning it into a layer.
%
% @arg Builder the layer builder to commit.
% @arg Layer the layer that will be returned.
% @throws if the builder has already been committed.

%! node_and_value_count(+Layer:layer, -Count:integer) is det.
%
% Unify Count with the amount of nodes and values known to this layer,
% including all parent layers.
%
% @arg Layer the layer for which to get a count.
% @arg Count the returned count.

%! predicate_count(+Layer:layer, -Count:integer) is det.
%
% Unify Count with the amount of predicates known to this layer,
% including all parent layers.
%
% @arg Layer the layer for which to get a count.
% @arg Count the returned count.

%! subject_to_id(+Layer:layer, +Subject:text, -Id:integer) is semidet.
%
% Convert the given subject to its id representation in the given layer.
% Fails if this subject is not known in the given layer.
%
% @arg Layer the layer to use for the conversion.
% @arg Subject an atom or string containing the subject.
% @arg Id the id of the subject in the given layer.

%! id_to_subject(Layer:layer, +Id:integer, -Subject:string) is semidet.
%
% Convert the given id to a subject using the given layer.
% Fails if the id is out of range for subjects.
%
% @arg Layer the layer to use for the conversion.
% @arg Id the id to convert into a subject.
% @arg Subject the subject which the id refers to.

%! predicate_to_id(+Layer:layer, +Predicate:text, -Id:integer) is semidet.
%
% Convert the given predicate to its id representation in the given layer.
% Fails if this predicate is not known in the given layer.
%
% @arg Layer the layer to use for the conversion.
% @arg Predicate an atom or string containing the predicate.
% @arg Id the id of the predicate in the given layer.

%! id_to_predicate(Layer:layer, +Id:integer, -Predicate:string) is semidet.
%
% Convert the given id to a predicate using the given layer.
% Fails if the id is out of range for predicates.
%
% @arg Layer the layer to use for the conversion.
% @arg Id the id to convert into a predicate.
% @arg Predicate the predicate which the id refers to.

%! object_node_to_id(+Layer:layer, +Object:text, -Id:integer) is semidet.
%
% Convert the given node object to its id representation in the given layer.
% Fails if this subject is not known in the given layer.
%
% @arg Layer the layer to use for the conversion.
% @arg Object an atom or string containing the object. The object is assumed to refer to a node.
% @arg Id the id of the object in the given layer.

%! object_value_to_id(+Layer:layer, +Object:text, -Id:integer) is semidet.
%
% Convert the given value object to its id representation in the given layer.
% Fails if this subject is not known in the given layer.
%
% @arg Layer the layer to use for the conversion.
% @arg Object an atom or string containing the object. The object is assumed to refer to a literal value.
% @arg Id the id of the object in the given layer.

%! id_to_object(Layer:layer, +Id:integer, -Object:string, -Object_Type:atom) is semidet.
%
% Convert the given id to a object using the given layer.
% Fails if the id is out of range for objects.
%
% @arg Layer the layer to use for the conversion.
% @arg Id the id to convert into a object.
% @arg Object the object which the id refers to.
% @arg Object_Type the type of the object, either 'node' or 'value'.

%! parent(+Layer:layer, +Parent:layer) is semidet.
%
% Unifies Parent with the parent layer of Layer. Fails if that layer
% has no parent.
%
% @arg Layer the layer for which to do the parent lookup.
% @arg Parent the retrieved parent layer.

%! lookup_subject(+Layer:layer, -Subject:subject_lookup) is nondet.
%
% Unify Subject with a subject lookup. On backtracking, this'll unify
% with all possible subjects.
%
% A subject lookup caches a lookup in a layer so that further
% operations only have to traverse data relevant to one particular
% subject.
%
% @arg Layer the layer to do the lookup in.
% @arg Subject the returned subject lookup.

%! lookup_subject(+Layer:layer, +Subject_Id:integer, -Subject:subject_lookup) is semidet.
%
% Unify Subject with the subject lookup for the given subject
% id. Fails if the lookup cannot be done for that id.
%
% A subject lookup caches a lookup in a layer so that further
% operations only have to traverse data relevant to one particular
% subject.
%
% @arg Layer the layer to do the lookup in.
% @arg Subject_Id the subject id to do the lookup with
% @arg Subject the returned subject lookup.
%

:- meta_predicate install_debug_hook(2).

%!  install_debug_hook(+DebugPred:callable) is det
%
%   Install the argument as a hook to be called when
%   debug is called **in Rust**.
%
%   @arg DebugPred  an arity 2 predicate with args
%   that are (Topic, Contents), where Topic is a text
%   representation of a prolog term and Contents is text.
%   Topic is converted to a term and sent to [[debug/3]]
%   with format `'~w'`
%

:- meta_predicate install_log_hook(1).

%!  install_log_hook(+LogPred:callable) is det
%
%   Install the argument as a hook to be called when
%   log is called **in Rust**
%
%   @arg LogPred an arity 1 predicate which is sent to
%   [[http_log/2]] with `'~w'` as first arg.
%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%% End of foreign predicate pldocs   %%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%! nb_add_triple(+Builder, +Subject, +Predicate, +Object) is semidet
%
% Add a triple to the builder.
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

nb_add_triple(_,_,_,Object) :-
    throw(
        error(
            domain_error(oneof([node(), value(), number]), Object),
                         context(terminus_store:nb_remove_triple/4,
       'triple must either be numeric, or object must be of format node(..) or value(..)')
        )).



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

nb_remove_triple(_,_,_,Object) :-
    throw(
        error(
            domain_error(oneof([node(), value(), number]), Object),
                         context(terminus_store:nb_remove_triple/4,
       'triple must either be numeric, or object must be of format node(..) or value(..)')
        )).


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
    id_to_object(Layer, Id, Object_String, Type),
    Object =.. [Type, Object_String].

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
    id_to_object(Layer, Id, Object_String, Type),
    Object =.. [Type, Object_String].

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    ground(Object),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Subject_Predicate_Lookup),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Predicate_Lookup),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Object),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    !,

    lookup_subject(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Object),
    !,

    lookup_object(Layer, Object, Object_Lookup),
    object_lookup_subject_predicate(Object_Lookup, Subject, Predicate).

id_triple(Layer, Subject, Predicate, Object) :-
    ground(Predicate),
    !,

    lookup_predicate(Layer, Predicate, Predicate_Lookup),
    predicate_lookup_subject_predicate_pair(Predicate_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_subject(Subject_Predicate_Lookup, Subject),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple(Layer, Subject, Predicate, Object) :-
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

    id_triple(Layer, S_Id, P_Id, O_Id),

    (   ground(Subject)
    ->  true
    ;   subject_id(Layer, Subject, S_Id)),


    (   ground(Predicate)
    ->  true
    ;   predicate_id(Layer, Predicate, P_Id)),


    (   ground(Object)
    ->  true
    ;   object_id(Layer,Object, O_Id)).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    ground(Object),
    !,

    lookup_subject_addition(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Subject_Predicate_Lookup),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    !,

    lookup_subject_addition(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Predicate_Lookup),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Object),
    !,

    lookup_subject_addition(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    !,

    lookup_subject_addition(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Object),
    !,

    lookup_object_addition(Layer, Object, Object_Lookup),
    object_lookup_subject_predicate(Object_Lookup, Subject, Predicate).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    ground(Predicate),
    !,

    lookup_predicate_addition(Layer, Predicate, Predicate_Lookup),
    predicate_lookup_subject_predicate_pair(Predicate_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_subject(Subject_Predicate_Lookup, Subject),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple_addition(Layer, Subject, Predicate, Object) :-
    lookup_subject_addition(Layer, Subject_Lookup),
    subject_lookup_subject(Subject_Lookup, Subject),
    subject_lookup_predicate(Subject_Lookup, Predicate_Lookup),
    subject_predicate_lookup_predicate(Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

triple_addition(Layer, Subject, Predicate, Object) :-
    (   ground(Subject)
    ->  subject_id(Layer, Subject, S_Id)
    ;   true),

    (   ground(Predicate)
    ->  predicate_id(Layer, Predicate, P_Id)
    ;   true),

    (   ground(Object)
    ->  object_id(Layer, Object, O_Id)
    ;   true),

    id_triple_addition(Layer, S_Id, P_Id, O_Id),

    (   ground(Subject)
    ->  true
    ;   subject_id(Layer, Subject, S_Id)),


    (   ground(Predicate)
    ->  true
    ;   predicate_id(Layer, Predicate, P_Id)),


    (   ground(Object)
    ->  true
    ;   object_id(Layer,Object, O_Id)).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    ground(Object),
    !,

    lookup_subject_removal(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Subject_Predicate_Lookup),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Predicate),
    !,

    lookup_subject_removal(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Predicate, Predicate_Lookup),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    ground(Object),
    !,

    lookup_subject_removal(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_has_object(Subject_Predicate_Lookup, Object).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Subject),
    !,

    lookup_subject_removal(Layer, Subject, Subject_Lookup),
    subject_lookup_predicate(Subject_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_predicate(Subject_Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Object),
    !,

    lookup_object_removal(Layer, Object, Object_Lookup),
    object_lookup_subject_predicate(Object_Lookup, Subject, Predicate).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    ground(Predicate),
    !,

    lookup_predicate_removal(Layer, Predicate, Predicate_Lookup),
    predicate_lookup_subject_predicate_pair(Predicate_Lookup, Subject_Predicate_Lookup),
    subject_predicate_lookup_subject(Subject_Predicate_Lookup, Subject),
    subject_predicate_lookup_object(Subject_Predicate_Lookup, Object).

id_triple_removal(Layer, Subject, Predicate, Object) :-
    lookup_subject_removal(Layer, Subject_Lookup),
    subject_lookup_subject(Subject_Lookup, Subject),
    subject_lookup_predicate(Subject_Lookup, Predicate_Lookup),
    subject_predicate_lookup_predicate(Predicate_Lookup, Predicate),
    subject_predicate_lookup_object(Predicate_Lookup, Object).

triple_removal(Layer, Subject, Predicate, Object) :-
    (   ground(Subject)
    ->  subject_id(Layer, Subject, S_Id)
    ;   true),

    (   ground(Predicate)
    ->  predicate_id(Layer, Predicate, P_Id)
    ;   true),

    (   ground(Object)
    ->  object_id(Layer, Object, O_Id)
    ;   true),

    id_triple_removal(Layer, S_Id, P_Id, O_Id),

    (   ground(Subject)
    ->  true
    ;   subject_id(Layer, Subject, S_Id)),


    (   ground(Predicate)
    ->  true
    ;   predicate_id(Layer, Predicate, P_Id)),


    (   ground(Object)
    ->  true
    ;   object_id(Layer,Object, O_Id)).

blob_allocations(allocations{stores:Stores,
                             named_graphs:Named_Graphs,
                             layers:Layers,
                             layer_builders:Layer_Builders,
                             subject_lookups:Subject_Lookups,
                             subject_predicate_lookups:Subject_Predicate_Lookups,
                             predicate_lookups:Predicate_Lookups,
                             object_lookups:Object_Lookups}) :-
    num_store_blobs(Stores),
    num_named_graph_blobs(Named_Graphs),
    num_layer_blobs(Layers),
    num_layer_builder_blobs(Layer_Builders),
    num_subject_lookup_blobs(Subject_Lookups),
    num_subject_predicate_lookup_blobs(Subject_Predicate_Lookups),
    num_predicate_lookup_blobs(Predicate_Lookups),
    num_object_lookup_blobs(Object_Lookups).

		 /*******************************
		 *  Rust debug / logging support *
		 *******************************/

%!  rust_debug(+TopicText:text, +Content:text) is det
%
%   @arg TopicText string rep. of a debug topic, eg "layers(delete_layer)
%
%   Our debug provided to the Rust side
%
rust_debug(TopicText, ContentText) :-
    text_to_string(TopicText, TopicStr),
    term_string(Topic, TopicStr),
    debug(Topic, '~w', [ContentText]).

:- use_module(library(http/http_log)).

%!  rust_log(+ContentText:text) is det
%
%   @arg ContentText contents to be logged
%
rust_log(ContentText) :-
    http_log('~N~w~n', [ContentText]).

install_logging_hooks :-
    install_debug_hook(terminus_store:rust_debug),
    install_log_hook(terminus_store:rust_log).

:- initialization install_logging_hooks.

:- begin_tests(terminus_store).

:- use_module(library(filesex)).

		 /*******************************
		 *     Developer Utilities      *
		 *******************************/


clean :-
    delete_directory_and_contents("testdir").

createng :-
    make_directory("testdir"),
    open_directory_store("testdir", X),
    create_named_graph(X, "sometestdb", _).

create_memory_ng(DB) :-
    open_memory_store(X),
    create_named_graph(X, "sometestdb", DB).

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
    create_named_graph(X, "sometestdb", _).


test(create_db_on_memory) :-
    open_memory_store(X),
    create_named_graph(X, "sometestdb", _).

test(open_named_graph, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", X),
    open_named_graph(X, "sometestdb", _).

test(open_named_graph_memory) :-
    open_memory_store(X),
    create_named_graph(X, "sometestdb", _),
    open_named_graph(X, "sometestdb", _).

test(head_from_empty_db, [fail, cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", X),
    open_named_graph(X, "sometestdb", DB),
    head(DB, _). % should be false because we have no HEAD yet

test(head_from_empty_db_memory, [fail, setup(create_memory_ng(DB))]) :-
     head(DB, _).

test(open_write_from_db_without_head, [
    cleanup(clean),
    setup(createng),
    throws(
        terminus_store_rust_error('Create a base layer first before opening the named graph for write')
    )]) :-
    open_directory_store("testdir", X),
    open_named_graph(X, "sometestdb", DB),
    open_write(DB, _).


test(open_write_from_memory_ng_without_head, [
    setup(create_memory_ng(DB)),
    throws(
        terminus_store_rust_error('Create a base layer first before opening the named graph for write')
    )]) :-
    open_write(DB, _).

test(create_base_layer, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, _).


test(create_base_layer_memory) :-
    open_memory_store(Store),
    open_write(Store, _).

test(write_value_triple, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object").

test(serialize_db, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object1"),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object2"),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object3"),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object4"),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object5"),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object6"),
    nb_commit(Builder, Layer),
    open_named_graph(Store, "sometestdb", DB),
    nb_set_head(DB, Layer),
    layer_to_id(Layer, Layer_ID),
    serialize_database("testdir", [Layer_ID], ['sometestdb.label'], "test.tar.gz").

test(deserialize_db, [cleanup(clean), setup(createng)]) :-
    make_directory_path("tmp_extract_dir"),
    deserialize_database("test.tar.gz", "tmp_extract_dir"),
    delete_directory_and_contents("tmp_extract_dir").

test(write_value_triple_memory) :-
    open_memory_store(Store),
    open_write(Store, Builder),
    nb_add_string_value_triple(Builder, "Subject", "Predicate", "Object").

test(commit_and_set_header, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    open_named_graph(Store, "sometestdb", DB),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer).


test(commit_and_set_header_memory) :-
    open_memory_store(Store),
    open_write(Store, Builder),
    create_named_graph(Store, "sometestdb", DB),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer).

test(head_after_first_commit, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_named_graph(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    head(DB, _).

test(predicate_count, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_named_graph(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    head(DB, LayerHead),
    predicate_count(LayerHead, Count),
    Count == 1.

test(node_and_value_count, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    node_and_value_count(Layer, Count),
    Count == 2.

test(predicate_count_2, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_named_graph(Store, "sometestdb", DB),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_add_triple(Builder, "Subject2", "Predicate2", value("Object2")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),
    predicate_count(Layer, Count),
    Count == 2.

test(remove_triple, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    open_write(Layer, LayerBuilder),
    nb_remove_triple(LayerBuilder, "Subject", "Predicate", value("Object")).

test(triple_search_test, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(X, triple(Layer, "Subject", "Predicate", value(X)), Bag),
    Bag == ["Object"].


test(triple_search_test, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(Y-X, triple(Layer, "Subject", Y, value(X)), Bag),
    Bag == ["Predicate"-"Object"].


test(triple_search_test, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
    nb_commit(Builder, Layer),
    setof(X-Y-Z, triple(Layer, X, Y, value(Z)), Bag),
    Bag == ["Subject"-"Predicate"-"Object"].

test(backtracking_test, [cleanup(clean), setup(createng)]) :-
    open_directory_store("testdir", Store),
    open_write(Store, Builder),
    create_named_graph(Store, "testdb", DB),
    nb_add_triple(Builder, "A", "B", node("C")),
    nb_add_triple(Builder, "A", "D", node("C")),
    nb_add_triple(Builder, "A", "E", node("C")),
    nb_add_triple(Builder, "A", "E", node("O")),
    nb_add_triple(Builder, "A", "D", node("O")),
    nb_commit(Builder, Layer),
    nb_set_head(DB, Layer),

    findall(P, triple(Layer, "A", P, node("O")), Ps),
    Ps = ["D", "E"].
:- end_tests(terminus_store).
