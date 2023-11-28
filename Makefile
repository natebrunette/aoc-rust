TARGET_DIR := ~/bin/aoc
BINARY_NAME := aoc-rust

.PHONY: all
all: install

.PHONY: build
build:
	cargo build --release

.PHONY: install
install: build
	cp ./target/release/$(BINARY_NAME) $(TARGET_DIR)
