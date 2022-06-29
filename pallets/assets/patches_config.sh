if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="orml"

prefix="tokens"

toml_list='
Cargo.toml
'

rs_list='
src/lib.rs
src/weights.rs
'

