# terminus-store prolog bindings

[![Build Status](https://travis-ci.com/terminusdb/terminus_store_prolog.svg?branch=master)](https://travis-ci.com/terminusdb/terminus_store_prolog)

Prolog bindings for the terminus-store Rust library.

## Requirements

* cargo
* gcc
* swi-prolog (with the include headers)

## Compiling and running

```
make
swipl prolog/terminus_store.pl
```

## Running the tests
```
make
swipl -g run_tests -g halt prolog/terminus_store.pl
```


## Examples

### Creating a database and adding a triple

```prolog
open_directory_store("testdir", Store),
open_write(Store, Builder),
open_database(Store, "sometestdb", DB),
nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
nb_commit(Builder, Layer),
nb_set_head(DB, Layer).
```

### Add a triple to an existing database

```prolog
open_directory_store("testdir", Store),
open_database(Store, "sometestdb", DB),
open_write(DB, Builder),
nb_add_triple(Builder, "Subject2", "Predicate2", value("Object2")),
nb_commit(Builder, Layer),
nb_set_head(DB, Layer),
```

### Query triples
```prolog
open_directory_store("testdir", Store),
open_database(Store, "sometestdb", DB),
head(DB, Layer),
triple(Layer, Subject, Predicate, Object).
```

### Convert strings to ids and query by id
```prolog
open_directory_store("testdir", Store),
open_database(Store, "sometestdb", DB),
head(DB, Layer),
subject_id("Subject", S_Id),
triple_id(Layer, S_Id, P_Id, O_Id),
predicate_id(Predicate, P_Id),
object_id(Object, O_Id).
```
