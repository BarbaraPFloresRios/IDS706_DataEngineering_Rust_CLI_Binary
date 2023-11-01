format:
	cargo fmt

release:
	cargo build --release

lint:
	cargo clippy

test:
	cargo test

run:
	#cargo run 

all: format lint test release run
