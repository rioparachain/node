#!/bin/sh -xe
USE_PATCHES_CONFIG=1
. ./patches_top_config.sh

TOP=$PWD

submodule_add() {
  git submodule add -f $repo $submodules_path
}

submodule_update() {
  git submodule update --init --remote $submodules_path
}

submodule_checkout() {
  cd $TOP/$submodules_path
  git fetch
  if [ "$branch" != "" ]; then
    git checkout $branch
    git pull
  else
    git checkout $rev
  fi
  cd $TOP
}

for subm in $submodules_list
do
  cd $TOP
  eval `echo submodules_path=\\\$${subm}_submodules_path`
  eval `echo repo=\\\$${subm}_repo`
  eval `echo branch=\\\$${subm}_branch`
  eval `echo rev=\\\$${subm}_rev`
  output=`git submodule status $submodules_path` 2> /dev/null || submodule_add
  if echo $output | grep -q '^-'; then
    submodule_update
  fi
  if [ ! -f $submodules_path/.git ]; then
    submodule_update
  fi
  submodule_checkout
done

