all: feo

feo:
	rustc src/lib.rs

test:
	rustc --test src/lib.rs -o /tmp/feo-test
	/tmp/feo-test

.PHONY: all feo test