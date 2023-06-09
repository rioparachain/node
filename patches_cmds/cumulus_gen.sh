#!/bin/sh -e

USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

dirs="
client/cli
client/collator
client/consensus/aura
client/consensus/common
client/consensus/relay-chain
client/network
client/pov-recovery
client/service
client/relay-chain-interface
client/relay-chain-inprocess-interface
client/relay-chain-rpc-interface
client/relay-chain-minimal-node
primitives/parachain-inherent
"

polkadot_pat="git = \"https://github.com/paritytech/polkadot\". branch = \"release-$polkadot_version\""
cumulus_fill="git = \"https://github.com/paritytech/cumulus\"@COM@ branch = \"polkadot-$polkadot_version\""

rewrite() {
  sed "s,$2 = { $polkadot_pat },$2 = { package = \"$3\"@COM@ path = \"${1}${3}\" },g"
}

rewrites() {
  #rewrite ${1}node/ polkadot-client relaychain-rio-client | \
  #rewrite ${1}node/ polkadot-service relaychain-rio-service | \
  cat | \
  sed "s,path = \"[\./]*primitives/core\",$cumulus_fill,g" | \
  sed "s,path = \"[\./]*../core\",$cumulus_fill,g" | \
  sed "s,path = \"[\./]*test/client\",$cumulus_fill,g" | \
  sed "s,path = \"[\./]*test/service\",$cumulus_fill,g" | \
  sed "s,path = \"[\./]*test/runtime\",$cumulus_fill,g" | \
  sed "s,path = \"[\./]*test/relay-sproof-builder\",$cumulus_fill,g" | \
  sed 's/@COM@/,/g'
}

for dir in $dirs
do
  dst=cumulus/$dir
  src=subm/$dst
  back=`echo $dir | sed 's,[^/]*,..,g'`
  mkdir -p $dst
  #ln -rsf $src/* $dst/
  cp -Rp $src/* $dst/
  rm -f $dst/Cargo.toml
  cat $src/Cargo.toml | rewrites $back/../ > $dst/Cargo.toml
  ln -rsf subm/cumulus/.rustfmt.toml $dst/
  for pfile in `find $dst -name "*.patch"`
  do
    sh -ec "cd `dirname $pfile`; patch" < $pfile
  done
done

