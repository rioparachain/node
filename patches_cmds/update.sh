#!/bin/sh -e
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

for load_fun in $loads
do
$load_fun

for toml in $toml_list
do
  dst_toml="$workdir/$toml"
  if [ -f $dst_toml ]; then
    get_original $prefix/$toml \
      | rewrite_path \
      > $dst_toml.tmp
    cat $dst_toml | sed "s,$polkadot_version_old_regex,$polkadot_version,g" > ${dst_toml}.tmp.migrate
    diff -u $dst_toml.tmp $dst_toml.tmp.migrate | remove_time_from_patch > $dst_toml.patch || test "$?" -le 1
    rm -f $dst_toml.tmp*
  fi
done

for rs in $rs_list
do
  dst_rs="$workdir/$rs"
  if [ -f $dst_rs ]; then
    get_original $prefix/$rs \
      > $dst_rs.tmp
    diff -u $dst_rs.tmp $dst_rs | remove_time_from_patch > $dst_rs.patch || test "$?" -le 1
    rm -f $dst_rs.tmp*
  fi
done

done
