all: feo doc

feo: libfeo-*

libfeo-*: src/*
	rustc src/lib.rs

test: src/*
	rustc --test src/lib.rs -o /tmp/feo-test
	/tmp/feo-test
	rustdoc --test src/lib.rs

doc: src/*
	rustdoc src/lib.rs

.PHONY: all feo test doc
