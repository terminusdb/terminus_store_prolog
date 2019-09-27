all: build

RUST_LIB = rust/target/debug/libterminus_store_prolog.so

build:
	cd rust; cargo build --release
	gcc -shared -o ./libterminus_store.so ./c/terminus_store.c -Isrc -L. -l:./$(RUST_LIB) -I/usr/lib/swi-prolog/include

clean:
	rm -rf *.so
