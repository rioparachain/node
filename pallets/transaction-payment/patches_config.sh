if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="substrate"

prefix="frame/transaction-payment"

toml_list='
Cargo.toml
'

