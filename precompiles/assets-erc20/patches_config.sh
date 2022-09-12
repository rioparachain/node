if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="aframe"

prefix="precompiles/assets-erc20"

toml_list='
Cargo.toml
'

rs_list='
src/lib.rs
src/mock.rs
src/tests.rs
'

