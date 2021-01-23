RUST_LIB_NAME = terminus_store_prolog
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
CC = gcc
ARCH = 
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)
WIN_SWIPL_INCLUDE = "/c/Program Files/swipl/include"
WIN_SWIPL_BIN = "/c/Program Files/swipl/bin"
SRCS = c/error.c c/blobs.c c/terminus_store.c
OBJS = target/error.o target/blobs.o target/terminus_store.o
CARGO_FLAGS =
BUILD_LD_OPTIONS =-Wl,-Bstatic -L./$(RUST_TARGET_DIR) -l$(RUST_LIB_NAME) -Wl,-Bdynamic -lc -fPIC

ifeq ($(OS), Windows_NT)
PACKSODIR = lib/x64-win64/
BUILD_LD_OPTIONS = -Wl,-Bstatic -l$(RUST_LIB_NAME) -Wl,-Bdynamic -lws2_32 -lwsock32 -luserenv -L$(WIN_SWIPL_BIN) -lswipl -I$(WIN_SWIPL_INCLUDE) -L$(RUST_TARGET_DIR)
SOEXT = dll
SWIARCH = x64-win64
else ifeq ($(shell uname),Darwin)
SOEXT = dylib
BUILD_LD_OPTIONS = $(LDSOFLAGS) -L$(SWIHOME)/$(PACKSODIR) $(SWILIB) -L./$(RUST_TARGET_DIR) -lterminus_store_prolog
endif

all: release

rust_bindings:
	cbindgen --config rust/cbindgen.toml rust/src/lib.rs --output c/terminus_store.h

windows: $(TARGET)
build: $(TARGET)

$(OBJS): target/%.o: c/%.c
	mkdir -p target || true
	$(CC) $(CFLAGS) -c $< -llibswipl -I$(WIN_SWIPL_INCLUDE) $(BUILD_LD_OPTIONS) -o $@

$(TARGET): $(OBJS)
	mkdir -p lib/$(SWIARCH)
	cd rust; cargo build $(CARGO_FLAGS)
	$(CC) -shared -o $@ $^ $(CFLAGS) $(BUILD_LD_OPTIONS)

check::

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
	rm -rf *.$(SOEXT) lib buildenv.sh target
	cd rust; cargo clean
