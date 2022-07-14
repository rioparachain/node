if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="orml"

prefix="tokens"

toml_list='
Cargo.toml
'

rs_list='
src/lib.rs
src/weights.rs
src/imbalances.rs
src/impls.rs
src/mock.rs
src/tests.rs
src/tests_currency_adapter.rs
src/tests_events.rs
src/tests_fungibles.rs
src/tests_multicurrency.rs
'

