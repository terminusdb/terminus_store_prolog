RUST_LIB_NAME = terminus_store_prolog
RUST_LIB = lib$(RUST_LIB_NAME).$(SOEXT)
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
CC = gcc
ARCH = 
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)
WINDOWS_TARGET = libterminus_store.dll
WIN_SWIPL_INCLUDE = "C:\Program Files\swipl\include"
WIN_TERMINUS_STORE_PROLOG_PATH = "C:\projects\terminus-store-prolog\rust\target\release"
WIN_SRCS = c/error.c c/blobs.c c/terminus_store.c
WIN_OBJS = error.o blobs.o terminus_store.o
CARGO_FLAGS=

all: release

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

windows: $(WINDOWS_TARGET)

$(WIN_OBJS): $(WIN_SRCS)
	cd rust; cargo build $(CARGO_FLAGS)
	$(CC) -O3 -Wall -c $^ -llibswipl -I $(WIN_SWIPL_INCLUDE) -L:$(WIN_TERMINUS_STORE_PROLOG_PATH) -lterminus_store_prolog

$(WINDOWS_TARGET): $(WIN_OBJS)
	$(CC) -shared -O3 -DLIBTERMINUS_STORE  -Wall -o $@ $^ -llibswipl -I $(WIN_SWIPL_INCLUDE) -L:$(WIN_TERMINUS_STORE_PROLOG_PATH) -lterminus_store_prolog -Wl,--out-implib,libterminus_store.a

check::

build:
	mkdir -p $(PACKSODIR)
	cd rust; cargo build $(CARGO_FLAGS)
	cp $(RUST_TARGET_DIR)/$(RUST_LIB) $(PACKSODIR)
	$(CC) -shared $(CFLAGS) -Wall -o $(TARGET) ./c/*.c -Isrc -L./$(PACKSODIR) -Wl,-rpath='$$ORIGIN' -l$(RUST_LIB_NAME)

debug: RUST_TARGET = debug
debug: CFLAGS += -ggdb
debug: build

release: RUST_TARGET = release
release: CARGO_FLAGS += --release
release: CFLAGS += -O3
release: build

windows_release: CARGO_FLAGS += --release
windows_release: windows

install::

clean:
	rm -rf *.$(SOEXT) lib buildenv.sh
	cd rust; cargo clean
