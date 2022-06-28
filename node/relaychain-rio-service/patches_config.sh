if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="polkadot"

prefix="node/service"

toml_list='
Cargo.toml
service/Cargo.toml
'

rs_list='
src/chain_spec.rs
'

