all: feo doc

feo:
	rustc src/lib.rs

test:
	rustc --test src/lib.rs -o /tmp/feo-test
	/tmp/feo-test
	rustdoc --test src/lib.rs

doc:
	rustdoc src/lib.rs

.PHONY: all feo test doc
