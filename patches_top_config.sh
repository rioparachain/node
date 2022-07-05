if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

polkadot_version="v0.9.24"
polkadot_version_old_regex="v0\.9\.23"

submodules_list="polkadot cumulus substrate orml frontier ptemplate"

polkadot_repo="https://github.com/paritytech/polkadot"
polkadot_branch="release-$polkadot_version"
polkadot_submodules_path="submodules/polkadot"

cumulus_repo="https://github.com/paritytech/cumulus"
cumulus_branch="polkadot-$polkadot_version"
cumulus_submodules_path="submodules/cumulus"

substrate_repo="https://github.com/paritytech/substrate"
substrate_branch="polkadot-$polkadot_version"
substrate_submodules_path="submodules/substrate"

orml_repo="https://github.com/open-web3-stack/open-runtime-module-library"
orml_rev="27e3272d322dcdd915f0fc2002032e8d53a46523"
orml_submodules_path="submodules/open-runtime-module-library"

frontier_repo="https://github.com/paritytech/frontier"
frontier_rev="175e42fc47cb6cd2772cccb7ac3ff59fd2d1a4dd"
frontier_submodules_path="submodules/frontier"

ptemplate_repo="https://github.com/substrate-developer-hub/substrate-parachain-template"
ptemplate_rev="polkadot-$polkadot_version"
ptemplate_submodules_path="submodules/ptemplate"

get_original() {
  eval "cat \$${submodule}_submodules_path/\$1"
}

rewrite_path() {
  case $submodule in
    polkadot)
      sed "s,path = \"../[^\"]*\",git = \"$polkadot_repo\"@COMMA@ branch = \"$polkadot_branch\",g" | sed 's/@COMMA@/,/g'
      ;;
    substrate)
      sed "s,path = \"../[^\"]*\",git = \"$substrate_repo\"@COMMA@ branch = \"$substrate_branch\",g" | sed 's/@COMMA@/,/g'
      ;;
    cumulus)
      sed "s,path = \"../[^\"]*\",git = \"$cumulus_repo\"@COMMA@ branch = \"$cumulus_branch\",g" | sed 's/@COMMA@/,/g'
      ;;
    orml)
      sed "s,path = \"../[^\"]*\",git = \"$orml_repo\"@COMMA@ rev = \"$orml_rev\",g" | sed 's/@COMMA@/,/g'
      ;;
    ptemplate)
      sed "s,path = \"../[^\"]*\",git = \"$ptemplate_repo\"@COMMA@ rev = \"$ptemplate_rev\",g" | sed 's/@COMMA@/,/g'
      ;;
    *)
      exit 1
      ;;
  esac
}

format_toml() {
  # TODO: What it nix is not used.
  #nix-shell -p dprint -p cargo-sort --run "
  sh -c "
    set -xe
    dprint fmt $1/*.toml
    cd $1
    cargo-sort -g
    set +x
  "
}

remove_time_from_patch() {
  sed -r 's,^(\-\-\- [A-Za-z][0-9A-Za-z\./_-]*).*$,\1,g' |
  sed -r 's,^(\+\+\+ [A-Za-z][0-9A-Za-z\./_-]*).*$,\1,g'
}

loads=""

configs=`find node pallets runtime -type f -name patches_config.sh`

for config in $configs
do
  workdir=`dirname $config`
  name=`echo $workdir | sed 's,[/\-],_,g'`
  loadname=`echo load_$name`
  loads="$loads $loadname"
  eval "
    $loadname() {
      toml_list=''
      rs_list=''
      workdir=$workdir
      . $config
    }
  "
  . $config
done

