RUST_LIB_NAME = terminus_store_prolog
RUST_LIB = lib$(RUST_LIB_NAME).$(SOEXT)
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
SWI_INCLUDE = -I"/usr/lib/swi-prolog/include" -I"/usr/local/lib/swipl/include" -I"/usr/lib/swipl/include"
CFLAGS = -fPIC -pthread
SOEXT = so 
CC = gcc
OUTPUT_DIR = lib
TARGET = $(OUTPUT_DIR)/libterminus_store.$(SOEXT)
CARGO_FLAGS=

all: release

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

check::

build:
	mkdir -p $(OUTPUT_DIR)
	cd rust; cargo build $(CARGO_FLAGS)
	mv $(RUST_TARGET_DIR)/$(RUST_LIB) lib
	$(CC) -shared $(CFLAGS) -o $(TARGET) ./c/*.c -Isrc $(SWI_INCLUDE) -L./$(OUTPUT_DIR) -Wl,-rpath $(CURDIR)/$(OUTPUT_DIR) -l$(RUST_LIB_NAME)

debug: RUST_TARGET = debug
debug: CFLAGS += -ggdb
debug: build

release: RUST_TARGET = release
release: CARGO_FLAGS += --release
release: build

install::

clean:
	rm -rf *.so lib
	cd rust; cargo clean
