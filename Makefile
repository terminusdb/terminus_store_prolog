# See <https://www.swi-prolog.org/howto/ForeignPack.html> for documentation on
# creating a pack using non-Prolog code.

# The variables `SWIARCH` and `PACKSODIR` are set by `swipl`. It calls `make`
# when the pack is installed with `pack_install(terminus_store_prolog)`.
# However, we also want to be able to build this without using `swipl`. If these
# variables are not already set, we run a script with `swipl` to look up their
# values.

# Architecture string used by `swipl`.
SWIARCH ?= $(shell ./script/swiarch.pl)

# Pack shared object directory used by `swipl`.
PACKSODIR ?= lib/$(SWIARCH)

# Rust and Cargo variables
RUST_LIB_NAME := terminus_store_prolog
RUST_TARGET := release
CARGO_FLAGS :=

# Set some architecture-dependent variables.
ifeq ($(SWIARCH), x64-win64)
  # Shared object file extension
  SOEXT := dll
else
  RUST_LIB_NAME := lib$(RUST_LIB_NAME)
  ifeq ($(SWIARCH), x86_64-darwin)
    # While SOEXT is set by `swipl`, the value for macOS is not what we want
    # ("so"). So, we set it correctly here.
    SOEXT := dylib
  else
    SOEXT := so
  endif
endif

all: release

build:
	mkdir -p $(PACKSODIR)
	cd rust; cargo build $(CARGO_FLAGS)
	cp rust/target/$(RUST_TARGET)/$(RUST_LIB_NAME).$(SOEXT) \
	   $(PACKSODIR)/libterminus_store.$(SOEXT)

check::

debug: RUST_TARGET = debug
debug: build

release: CARGO_FLAGS += --release
release: build

install::

clean:
	rm -rf lib
	cd rust; cargo clean
