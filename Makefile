RUST_LIB_NAME = terminus_store_prolog
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
RUST_TARGET_LOCATION = rust/target/$(RUST_TARGET)/lib$(RUST_LIB_NAME).$(SOEXT)
TARGET = $(PACKSODIR)/libterminus_store.$(SOEXT)
CARGO_FLAGS =

ifeq ($(OS), Windows_NT)
SOEXT = dll
# NOTE: this is not guaranteed but we only support win64 now anyway
SWIARCH = x64-win64
RUST_TARGET_LOCATION = rust/target/$(RUST_TARGET)/$(RUST_LIB_NAME).$(SOEXT)
TARGET = lib/$(SWIARCH)/libterminus_store.$(SOEXT)
else ifeq ($(shell uname), Darwin)
SOEXT = dylib
else ifeq ($(SWIARCH), arm64-android)
endif


all: release

build:
	mkdir -p lib/$(SWIARCH)
	cd rust; cargo build $(CARGO_FLAGS)
	cp $(RUST_TARGET_LOCATION) $(TARGET)

check::

debug: RUST_TARGET = debug
debug: build

release: CARGO_FLAGS += --release
release: build

windows_release: release

install::

clean:
	rm -rf *.$(SOEXT) lib buildenv.sh
	cd rust; cargo clean
