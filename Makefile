.PHONY: all test

all:
	cargo build

test:
	target/x
	./check-line-length.sh
