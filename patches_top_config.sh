if [ "$USE_PATCHES_CONFIG" != "1" ]; then exit 0; fi

add_patched_file_to_git()
{
  if [ "$ADD_PATCHED_TO_GIT" = "1" ]; then
    git add -f $1
  fi
}

polkadot_version="v0.9.37"
polkadot_version_old_regex="v0\.9\.36"

submodules_list="ptemplate cumulus orml frontier aframe"

ptemplate_repo="https://github.com/substrate-developer-hub/substrate-parachain-template"
ptemplate_rev="6cde4f130cc41287ef2ed5a00093991197f3c1a3"
ptemplate_submodules_path="subm/ptemplate"

cumulus_repo="https://github.com/paritytech/cumulus"
cumulus_branch="polkadot-$polkadot_version"
cumulus_submodules_path="subm/cumulus"

orml_repo="https://github.com/open-web3-stack/open-runtime-module-library"
orml_rev="16b6c1149a15674d21c87244b7988a667e2c14d9"
orml_submodules_path="subm/orml"

frontier_repo="https://github.com/paritytech/frontier"
frontier_branch="polkadot-$polkadot_version"
frontier_submodules_path="subm/frontier"

aframe_repo="https://github.com/AstarNetwork/astar-frame"
aframe_rev="07b479cd20c8f6cbd09efe0086e85eefe4430a9d"
aframe_submodules_path="subm/aframe"

get_original() {
  eval "cat \$${submodule}_submodules_path/\$1"
}

rewrite_path() {
  case $submodule in

    ptemplate)
      sed "s,path = \"../[^\"]*\",git = \"$ptemplate_repo\"@COMMA@ rev = \"$ptemplate_rev\",g" | sed 's/@COMMA@/,/g'
      ;;

    cumulus)
      sed "s,path = \"../[^\"]*\",git = \"$cumulus_repo\"@COMMA@ branch = \"$cumulus_branch\",g" | sed 's/@COMMA@/,/g'
      ;;

    orml)
      sed "s,path = \"../[^\"]*\",git = \"$orml_repo\"@COMMA@ branch = \"$orml_dev\",g" | sed 's/@COMMA@/,/g'
      ;;

    frontier)
      sed "s,path = \"../[^\"]*\",git = \"$frontier_repo\"@COMMA@ branch = \"$frontier_branch\",g" | sed 's/@COMMA@/,/g'
      ;;

    aframe)
      sed "s,path = \"../[^\"]*\",git = \"$aframe_repo\"@COMMA@ rev = \"$aframe_rev\",g" | sed 's/@COMMA@/,/g'
      ;;

    *)
      exit 1
      ;;
  esac
}

format_toml() {
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

