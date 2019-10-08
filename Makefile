RUST_LIB_NAME = terminus_store_prolog
RUST_LIB = lib$(RUST_LIB_NAME).$(SOEXT)
RUST_TARGET=release
RUST_SOURCE_PATH = rust/target/$(RUST_TARGET)/$(RUST_LIB)
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)

all: build

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

check::

build:
	mkdir -p $(PACKSODIR)
	cd rust; cargo build --$(RUST_TARGET)
	mv $(RUST_SOURCE_PATH) ./$(PACKSODIR)
	$(CC) -shared $(CFLAGS) -o $(TARGET) ./c/*.c -Isrc -L./$(PACKSODIR) -Wl,-rpath $(CURDIR)/$(PACKSODIR) -l$(RUST_LIB_NAME)

debug: RUST_TARGET = debug
debug: CFLAGS += -ggdb
debug:
	mkdir -p $(PACKSODIR)
	cd rust; cargo build
	mv $(RUST_SOURCE_PATH) ./$(PACKSODIR)
	$(CC) -shared $(CFLAGS) -o $(TARGET) ./c/*.c -Isrc -L./$(PACKSODIR) -Wl,-rpath $(CURDIR)/$(PACKSODIR) -l$(RUST_LIB_NAME)


install::

clean:
	rm -rf *.so
	cd rust; cargo clean
