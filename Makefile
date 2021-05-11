RUST_LIB_NAME = terminus_store_prolog_rewrite
RUST_TARGET=release
RUST_TARGET_DIR = rust/target/$(RUST_TARGET)/
RUST_TARGET_LOCATION = rust/target/$(RUST_TARGET)/lib$(RUST_LIB_NAME).$(SOEXT)
ARCH = 
TARGET = $(PACKSODIR)/libterminus_store_rewrite.$(SOEXT)
CARGO_FLAGS =

ifeq ($(OS), Windows_NT)
SOEXT = dll
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

windows_release: CARGO_FLAGS += --release
windows_release: windows

install::

clean:
	rm -rf *.$(SOEXT) lib buildenv.sh
	cd rust; cargo clean
