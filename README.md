# terminus-store prolog bindings

[![Actions Status](https://github.com/terminusdb/terminus_store_prolog/workflows/Publish/badge.svg)](https://github.com/terminusdb/terminus_store_prolog/actions)

Prolog bindings for the [terminus-store](https://github.com/terminusdb/terminus-store/) Rust library.

## Requirements

* cargo
* gcc
* swi-prolog (with the include headers)

## Installing
This library is downloadable through SWI-Prolog's package management system. In a swipl instance, run
```prolog
pack_install(terminus_store_prolog).
```

Then you can use the library with
```prolog
use_module(library(terminus_store)).
```
## Compiling and running without installing (for testing purposes)
If you need to compile manually, for example to test a change without reinstalling the pack, follow these instructions.

Use the wrapper script `./make.sh` rather than the Makefile directly. The wrapper script will set up swipl environment variables which the build needs.

Also, use the provided `./run_swipl` script to start a test instance. This will ensure the foreign library will be located properly.
```
./make.sh
./run_swipl
```

## Running the tests
```
./make.sh
./run_swipl -g run_tests -g halt
```


## Examples

### Creating a named graph and adding a triple
Create a new directory (`testdir` in this example), then do the following:

```prolog
open_directory_store("testdir", Store),
open_write(Store, Builder),
create_named_graph(Store, "sometestdb", DB),
nb_add_triple(Builder, "Subject", "Predicate", value("Object")),
nb_commit(Builder, Layer),
nb_set_head(DB, Layer).
```

### Add a triple to an existing named graph

```prolog
open_directory_store("testdir", Store),
open_named_graph(Store, "sometestdb", DB),
open_write(DB, Builder),
nb_add_triple(Builder, "Subject2", "Predicate2", value("Object2")),
nb_commit(Builder, Layer),
nb_set_head(DB, Layer),
```

### Query triples
```prolog
open_directory_store("testdir", Store),
open_named_graph(Store, "sometestdb", DB),
head(DB, Layer),
triple(Layer, Subject, Predicate, Object).
```

### Convert strings to ids and query by id
```prolog
open_directory_store("testdir", Store),
open_named_graph(Store, "sometestdb", DB),
head(DB, Layer),
subject_id(Layer, "Subject", S_Id),
id_triple(Layer, S_Id, P_Id, O_Id),
predicate_id(Layer, Predicate, P_Id),
object_id(Layer, Object, O_Id).
```
