#!/bin/sh -xe
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

source_files_patterns="
'./Cargo.lock'
'./.cargo/config.toml'
'./.dprint.json'
'*/Cargo.toml'
'*.rs'
'*/.rustfmt.toml'
"

do_clean() {
  # Removing files and folders what is not needed.
  rm -Rf single-branch-tmp single-branch single-branch.sha256sum.txt.tmp
}

hash_of_list() {
  if [ -f $1 ]; then
    # Using sorted by hash list of files, input comes from sha256sum where hash is separated by two spaces.
    sort $1 | sha256sum | awk '{ print $1 }'
  else
    echo 0000000000000000000000000000000000000000000000000000000000000000
  fi
}

get_source_files() {
  rules=""
  sep=""
  for pat in $source_files_patterns
  do
    rules="$rules $sep -path $pat"
    sep="-or"
  done
  cd $1
  sh -ec "exec find . \\( $rules $2 \\) $3" | sed "s,^\./,$1/,g"
  cd $OLDPWD
}

do_clean

git clone --single-branch --depth 1 --no-local . single-branch-tmp

sh -c './patches_cmds/subdir_apply_all.sh single-branch-tmp'

# Fixing problem with symlinks by copy with symlink follow.
cp -LRp single-branch-tmp single-branch

# Remove unneeded submodules, it contains toml and rs files that is not needed.
rm -Rf single-branch/subm

# Make list of source files with hash.
get_source_files single-branch "" "-exec sha256sum {} \;" | sort -k2 > single-branch.sha256sum.txt.tmp
old_hash=`hash_of_list .cache/single-branch.sha256sum.txt`
new_hash=`hash_of_list single-branch.sha256sum.txt.tmp`

if [ "$old_hash" != "$new_hash" ]; then
  # Make list of files and dirs for cargo build process, makeing archive file.
  get_source_files single-branch "-or -name '.git'" "" > single-branch/list.txt
  # Create deterministic archive.
  tar --sort=name --owner=root:0 --group=root:0 --mtime='UTC 2019-01-01' \
      -cf- -T single-branch/list.txt \
    | gzip -n -9 > single-branch.tgz
  mv single-branch.sha256sum.txt.tmp single-branch.sha256sum.txt
  sha256sum single-branch.tgz > single-branch.tgz.sha256sum.txt
  do_clean
  mkdir -p .cache
  mv single-branch.tgz single-branch.tgz.sha256sum.txt single-branch.sha256sum.txt .cache/
  #git add single-branch.tgz.sha256sum.txt single-branch.sha256sum.txt
  #git commit -am "Adding new single branch archive hashes"
else
  do_clean
fi

