all: build

RUST_LIB = rust/target/debug/libterminus_store_prolog.so

check::

build:
	cd rust; cargo build
	gcc -shared -fPIC -o ./libterminus_store.so ./c/terminus_store.c -Isrc -L. -l:./$(RUST_LIB) -I/usr/lib/swi-prolog/include -I/usr/local/lib/swipl/include

install::

clean:
	rm -rf *.so
	cd rust; cargo clean
