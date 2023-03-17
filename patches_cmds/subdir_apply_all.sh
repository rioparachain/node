#!/bin/sh -e
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

topdir=$PWD
subdir=$1

set -x
mkdir -p $subdir/subm
set +x

for subm in $submodules_list
do
  eval `echo submodules_path=\\\$${subm}_submodules_path`
  eval `echo repo=\\\$${subm}_repo`
  eval `echo branch=\\\$${subm}_branch`
  eval `echo rev=\\\$${subm}_rev`
  if [ "$branch" != "" ]; then
    set -x
    git clone -b $branch --single-branch --depth 1 $repo $subdir/$submodules_path
    set +x
  else
    set -x
    git clone --single-branch $repo $subdir/$submodules_path
    PWDBAK=$PWD
    cd $subdir/$submodules_path
    git checkout $rev
    cd $PWDBAK
    set +x
  fi
done

set -x

cd $subdir

# Applying patches to create rs and toml files, generate patched cumulus source files.
#./symlink_aws_fix.sh
#./patches_cmds/apply.sh all
./patches_cmds/cumulus_gen.sh
./patches_cmds/frontier_gen.sh

if [ "$ADD_PATCHED_TO_GIT" = "1" ]; then
  for file in `find cumulus`
  do
    git add -f $file
  done
fi
