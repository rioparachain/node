#!/bin/sh -e

#rm -Rf $DIR

if [ ! -d ./$DIR ]
then
  git clone $REPOSITORY $DIR
  cd $DIR
  git submodule update --init --remote
  cd $OLDPWD
fi

cd $DIR
git checkout 78497273efd5246a33bedf05ff52d526a865b069
git branch -D $BRANCH || true
git fetch --all
git checkout $BRANCH
echo "COMMIT: `git rev-parse HEAD`"
cd submodules/polkadot
git checkout 17c7b9594aedbfc644d7e6e26f7bd244e68ccf4d
cd ../cumulus
git checkout ebdfbea0029dd3349ce0e9c758acc73acce04d18
cd ../../
./scripts/patches_apply.sh
./cumulus_patch.sh

