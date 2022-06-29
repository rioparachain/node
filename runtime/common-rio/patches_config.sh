if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="polkadot"

prefix="runtime/common"

toml_list='
Cargo.toml
'

