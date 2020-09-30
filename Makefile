RUST_LIB_NAME = terminus_store_prolog
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
CC = gcc
ARCH = 
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)
WIN_SWIPL_INCLUDE = "C:\Program Files\swipl\include"
WIN_TERMINUS_STORE_PROLOG_PATH = "C:\projects\terminus-store-prolog\rust\target\release"
SRCS = c/error.c c/blobs.c c/terminus_store.c
OBJS = error.o blobs.o terminus_store.o
CARGO_FLAGS =
BUILD_LD_OPTIONS = 

ifeq ($(SWIARCH),x86_64-darwin)
SOEXT = dylib
BUILD_LD_OPTIONS = -L$(SWIHOME)/$(PACKSODIR) $(SWILIB)
endif

all: release

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

windows: $(TARGET)

$(OBJS): $(SRCS)
	cd rust; cargo build $(CARGO_FLAGS)
	$(CC) $(CFLAGS) -c $^ -llibswipl -I$(WIN_SWIPL_INCLUDE)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -DLIBTERMINUS_STORE -shared -o $@ $^ -Wl,-Bstatic -l$(RUST_LIB_NAME) -Wl,-Bdynamic -lws2_32 -lwsock32 -luserenv -llibswipl -I$(WIN_SWIPL_INCLUDE) -L$(WIN_TERMINUS_STORE_PROLOG_PATH)

check::

build:
	mkdir -p $(PACKSODIR)
	cd rust; cargo build $(CARGO_FLAGS)
	$(CC) -shared $(CFLAGS) -Wall -o $(TARGET) ./c/*.c -Isrc -L./$(RUST_TARGET_DIR) $(BUILD_LD_OPTIONS) -l$(RUST_LIB_NAME)

debug: RUST_TARGET = debug
debug: CFLAGS += -ggdb
debug: build

release: RUST_TARGET = release
release: CARGO_FLAGS += --release
release: CFLAGS += -O3
release: build

windows_release: CARGO_FLAGS += --release
windows_release: CFLAGS += -O3 -Wall
windows_release: windows

install::

clean:
	rm -rf *.$(SOEXT) lib buildenv.sh
	cd rust; cargo clean
