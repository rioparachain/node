if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="polkadot"

prefix="node/client"

toml_list='
Cargo.toml
'

