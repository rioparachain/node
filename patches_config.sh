if [ "$USE_PATCHES_CONFIG" != "1" ]; then
  exit 0
fi

polkadot_repo="https://github.com/paritytech/polkadot"
polkadot_branch="release-v0.9.22"
polkadot_submodules_path="submodules/polkadot"

cumulus_repo="https://github.com/paritytech/cumulus"
cumulus_branch="polkadot-v0.9.19"
cumulus_submodules_path="submodules/cumulus"

get_original() {
  case $submodule in
    polkadot)
      cat $polkadot_submodules_path/$1
      ;;
    cumulus)
      cat $cumulus_submodules_path/$1
      ;;
    *)
      exit 1
      ;;
  esac
}

rewrite_path() {
  case $submodule in
    polkadot)
      sed "s,path = \"../[^\"]*\",git = \"$polkadot_repo\"@COMMA@ branch = \"$polkadot_branch\",g" | sed 's/@COMMA@/,/g'
      ;;
    cumulus)
      sed "s,path = \"../[^\"]*\",git = \"$cumulus_repo\"@COMMA@ branch = \"$cumulus_branch\",g" | sed 's/@COMMA@/,/g'
      ;;
    *)
      exit 1
      ;;
  esac
}

remove_time_from_patch() {
  sed -r 's,^(\-\-\- [A-Za-z][0-9A-Za-z\./_-]*).*$,\1,g' |
  sed -r 's,^(\+\+\+ [A-Za-z][0-9A-Za-z\./_-]*).*$,\1,g'
}

loads=""

loads="$loads load_runtime_relaychain_rio"
load_runtime_relaychain_rio() {
workdir="runtime/relaychain-rio"
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
}

loads="$loads load_relaychain_rio_cli"
load_relaychain_rio_cli() {
workdir="node/relaychain-rio-cli"
submodule="polkadot"
prefix="cli"
toml_list='
Cargo.toml
service/Cargo.toml
'
rs_list='
src/command.rs
'
}

loads="$loads load_relaychain_rio_service"
load_relaychain_rio_service() {
workdir="node/relaychain-rio-service"
submodule="polkadot"
prefix="node/service"
toml_list='
Cargo.toml
service/Cargo.toml
'
rs_list='
src/chain_spec.rs
'
}

loads="$loads load_relaychain_rio_client"
load_relaychain_rio_client() {
workdir="node/relaychain-rio-client"
submodule="polkadot"
prefix="node/client"
toml_list='
Cargo.toml
'
rs_list='
'
}




