.PHONY: build check test fmt check-fmt fmt-dprint check-dprint clippy pr-checks

TOP := $(shell git rev-parse --show-toplevel)
NIX_SHELL_FLAGS := $(TOP)/docker/nix/shell.nix
NIX_SHELL = $(shell cmd="`which cached-nix-shell` $(NIX_SHELL_FLAGS) --run" || cmd="`which nix-shell` $(NIX_SHELL_FLAGS) --run" || cmd="`which bash` -c"; echo $$cmd)

DPRINT_CARGO_TOML_FILES = $(patsubst %/Cargo.toml,%,$(shell $(NIX_SHELL) 'dprint output-file-paths' | grep 'Cargo\.toml'))

FLAGS := --release --features fast-runtime --features rio-testnet
#FLAGS := --release --features fast-runtime

build:
	$(NIX_SHELL) 'cargo build $(FLAGS)'

check:
	$(NIX_SHELL) 'SKIP_WASM_BUILD=1; cargo check $(FLAGS)'

fmt:
	$(NIX_SHELL) 'cargo fmt'

check-fmt:
	$(NIX_SHELL) 'cargo fmt --check'

fmt-dprint: NIX_SHELL_FLAGS := -p dprint -p cargo-sort
fmt-dprint:
	$(NIX_SHELL) 'dprint fmt'
	set -xe; \
	for dir in $(DPRINT_CARGO_TOML_FILES); \
	do $(NIX_SHELL) "cd $$dir && cargo-sort -g"; done

check-dprint: NIX_SHELL_FLAGS := -p dprint -p cargo-sort
check-dprint:
	$(NIX_SHELL) 'dprint check'
	set -xe; \
	for dir in $(DPRINT_CARGO_TOML_FILES); \
	do $(NIX_SHELL) "cd $$dir && cargo-sort -g -c"; done

clippy:
	$(NIX_SHELL) 'SKIP_WASM_BUILD=1 env -u RUSTFLAGS cargo clippy --all-targets $(FLAGS)'

test:
	$(NIX_SHELL) 'cargo test $(FLAGS)'

pr-checks: check-dprint check-fmt clippy test

