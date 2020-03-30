RUST_LIB_NAME = terminus_store_prolog
RUST_LIB = lib$(RUST_LIB_NAME).$(SOEXT)
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
CC = gcc
ARCH = 
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)
WIN_SWIPL_INCLUDE = "C:\Program Files\swipl\include"
WIN_TERMINUS_STORE_PROLOG_PATH = "C:\projects\terminus-store-prolog\rust\target\release"
SRCS = c/error.c c/blobs.c c/terminus_store.c
OBJS = error.o blobs.o terminus_store.o
CARGO_FLAGS=

all: release

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

windows: $(TARGET)

$(OBJS): $(SRCS)
	cd rust; cargo build $(CARGO_FLAGS)
	$(CC) $(CFLAGS) -c $^ -llibswipl -I $(WIN_SWIPL_INCLUDE) -L:$(WIN_TERMINUS_STORE_PROLOG_PATH) -lterminus_store_prolog

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -DLIBTERMINUS_STORE -shared -o $@ $^ -llibswipl -I $(WIN_SWIPL_INCLUDE) -L:$(WIN_TERMINUS_STORE_PROLOG_PATH) -lterminus_store_prolog -Wl,--out-implib,libterminus_store.a

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
windows_release: TARGET = libterminus_store.dll
windows_release: CFLAGS += -O3 -Wall
windows_release: windows

install::

clean:
	rm -rf *.$(SOEXT) lib buildenv.sh
	cd rust; cargo clean
