#!/bin/sh -e

USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

dirs="
client/consensus
client/db
client/rpc
client/rpc-core
client/mapping-sync
primitives/consensus
primitives/storage
primitives/rpc
primitives/evm
primitives/self-contained
primitives/dynamic-fee
primitives/ethereum
template/runtime
frame/evm
frame/evm/precompile/modexp
frame/evm/precompile/sha3fips
frame/evm/precompile/dispatch
frame/evm/precompile/bn128
frame/evm/test-vector-support
frame/evm/precompile/simple
frame/hotfix-sufficients
frame/ethereum
frame/base-fee
frame/dynamic-fee
"

substrate_pat="git = \"https://github.com/paritytech/substrate\". branch = \"master\""
substrate_fill="git = \"https://github.com/paritytech/substrate\"@COM@ branch = \"polkadot-$polkadot_version\""
polkadot_pat="git = \"https://github.com/paritytech/polkadot\". branch = \"release-$polkadot_version\""
cumulus_fill="git = \"https://github.com/paritytech/cumulus\"@COM@ branch = \"polkadot-$polkadot_version\""

rewrite() {
  sed "s,$2 = { $substrate_pat },$2 = { package = \"$3\"@COM@ path = \"${1}${3}\" },g"
  #sed "s,$2 = { $polkadot_pat },$2 = { package = \"$3\"@COM@ path = \"${1}${3}\" },g"
}

rewrites() {
  #cat
  sed "s,$substrate_pat,$substrate_fill,g" | \
  sed 's/@COM@/,/g'
#  rewrite ${1}node/ polkadot-client relaychain-rio-client | \
#  rewrite ${1}node/ polkadot-service relaychain-rio-service | \
#  sed "s,path = \"[\./]*primitives/core\",$cumulus_fill,g" | \
#  sed "s,path = \"[\./]*../core\",$cumulus_fill,g" | \
#  sed "s,path = \"[\./]*test/client\",$cumulus_fill,g" | \
#  sed "s,path = \"[\./]*test/service\",$cumulus_fill,g" | \
#  sed "s,path = \"[\./]*test/runtime\",$cumulus_fill,g" | \
#  sed "s,path = \"[\./]*test/relay-sproof-builder\",$cumulus_fill,g" | \
}

for dir in $dirs
do
  dst=frontier/$dir
  src=subm/$dst
  back=`echo $dir | sed 's,[^/]*,..,g'`
  mkdir -p $dst
  #ln -rsf $src/* $dst/
  cp -Rp $src/* $dst/
  rm -f $dst/Cargo.toml
  cat $src/Cargo.toml | rewrites $back/../ > $dst/Cargo.toml
  ln -rsf subm/frontier/rustfmt.toml $dst/.rustfmt.toml
  for pfile in `find $dst -name "*.patch"`
  do
    sh -ec "cd `dirname $pfile`; patch" < $pfile
  done
done

