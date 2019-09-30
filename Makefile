INCLUDES = -I/usr/lib/swi-prolog/include -I/usr/local/lib/swipl/include
CC = gcc
CFLAGS = -shared -fpic -Wall
RUST_LIB = rust/target/debug/libterminus_store_prolog.so

TARGET = libterminus_store.so

all: build


check::

build:
	cd rust; cargo build
	$(CC) $(CFLAGS) -o $(TARGET) ./c/*.c -Isrc -L. -l:./$(RUST_LIB) $(INCLUDES)

install::

clean:
	rm -rf *.so
	cd rust; cargo clean
