TARGET_DIR=target

all: $(TARGET_DIR)/ $(TARGET_DIR)/feo doc

$(TARGET_DIR)/:
	mkdir $(TARGET_DIR)/

clean:
	rm -rf $(TARGET_DIR)
	rm -rf doc

$(TARGET_DIR)/feo: $(TARGET_DIR)/libparse-*.rlib src/feo/*
	rustc src/feo/main.rs -o $(TARGET_DIR)/feo

$(TARGET_DIR)/libparse-*.rlib: src/parse/*
	rustc src/parse/lib.rs --out-dir $(TARGET_DIR)

target/test/parse: src/parse/
	mkdir -p target/test/
	rustc --test src/parse/lib.rs -o $(TARGET_DIR)/test/parse
	RUST_BACKTRACE=1 $(TARGET_DIR)/test/parse
	rustdoc --test src/parse/lib.rs

target/test/feo: src/feo/
	mkdir -p target/test/
	rustc --test src/feo/main.rs -o $(TARGET_DIR)/test/feo
	RUST_BACKTRACE=1 $(TARGET_DIR)/test/feo
	rustdoc --test src/feo/main.rs

test: target/test/feo target/test/parse

doc/parse/: src/parse/*
	rustdoc src/parse/lib.rs

doc: doc/parse/

.PHONY: all clean test doc
