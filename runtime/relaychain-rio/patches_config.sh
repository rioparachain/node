if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="polkadot"

prefix="runtime/polkadot"

toml_list='
Cargo.toml
constants/Cargo.toml
'

rs_list='
constants/src/lib.rs
src/lib.rs
src/xcm_config.rs
'

