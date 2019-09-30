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
