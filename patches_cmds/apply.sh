#!/bin/sh -e
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

for load_fun in $loads
do
$load_fun

for toml in $toml_list
do
  dst_toml="$workdir/$toml"
  if [ \( "$1" = "all" -o "$1" = "$dst_toml.patch" \) -a -f $dst_toml.patch ]; then
    get_original $prefix/$toml \
      | rewrite_path \
      > $dst_toml.tmp
    patch $dst_toml.tmp $dst_toml.patch
    mv $dst_toml.tmp $dst_toml
  fi
done

for rs in $rs_list
do
  dst_rs="$workdir/$rs"
  if [ \( "$1" = "all" -o "$1" = "$dst_rs.patch" \) -a -f $dst_rs.patch ]; then
    get_original $prefix/$rs \
      > $dst_rs.tmp
    patch $dst_rs.tmp $dst_rs.patch
    mv $dst_rs.tmp $dst_rs
  fi
done

done
