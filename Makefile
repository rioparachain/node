.PHONY: build check test fmt

TOP := $(shell git rev-parse --show-toplevel)
NIX_SHELL := $(shell cmd="`which cached-nix-shell` $(TOP)/docker/nix/shell.nix --run" || cmd="`which nix-shell` $(TOP)/docker/nix/shell.nix --run" || cmd="`which bash` -c"; echo $$cmd)

FLAGS := --release --features fast-runtime

build:
	$(NIX_SHELL) 'cargo build $(FLAGS)'

check:
	$(NIX_SHELL) 'SKIP_WASM_BUILD=1; cargo check $(FLAGS)'

test:
	$(NIX_SHELL) 'SKIP_WASM_BUILD=1; cargo test $(FLAGS)'

fmt:
	$(NIX_SHELL) 'cargo fmt'
