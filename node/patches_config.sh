if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

submodule="ptemplate"

prefix="node"

toml_list='
Cargo.toml
'

rs_list='
src/service.rs
src/main.rs
src/command.rs
src/cli.rs
src/chain_spec.rs
'

load_parachain_node_rpc() {
  submodule="frontier"
  prefix="template/node"
  toml_list=''
  workdir='node'
  rs_list='src/rpc.rs'
}
loads="$loads load_parachain_node_rpc"

