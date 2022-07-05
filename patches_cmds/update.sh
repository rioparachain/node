#!/bin/sh -e
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

for load_fun in $loads
do
$load_fun

for toml in $toml_list
do
  dst_toml="$workdir/$toml"
  dst_toml_tmp=`dirname $dst_toml`/.tmp
  dst_toml_name=`basename $dst_toml`
  if [ -f $dst_toml ]; then
    if [ -f $dst_toml.patch -a $dst_toml.patch -nt $dst_toml ]; then
      continue
    fi
    mkdir -p $dst_toml_tmp/original $dst_toml_tmp/migrate
    get_original $prefix/$toml \
      | rewrite_path \
      > $dst_toml_tmp/original/$dst_toml_name
    cat $dst_toml | sed "s,$polkadot_version_old_regex,$polkadot_version,g" > $dst_toml_tmp/migrate/$dst_toml_name
    format_toml $dst_toml_tmp/original
    format_toml $dst_toml_tmp/migrate
    diff -u $dst_toml_tmp/original/$dst_toml_name $dst_toml_tmp/migrate/$dst_toml_name \
      | remove_time_from_patch > $dst_toml.patch || test "$?" -le 1
    rm -Rf $dst_toml_tmp
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
