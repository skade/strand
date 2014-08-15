RUST ?= rust
RUSTC ?= rustc
RUSTTEST ?= rustc --test
RUSTFLAGS ?= -O --out-dir build -L build -L leveldb/build -L leveldb/leveldb -C link-args="-lleveldb"

VERSION=0.1-pre

strand: leveldb/build/leveldb*.rlib
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs

test: strand
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) --test src/test/test.rs
	rm -rf testdbs
	mkdir testdbs
	LD_LIBRARY_PATH=leveldb build/test

leveldb/build/leveldb*.rlib:
	cd leveldb; make
