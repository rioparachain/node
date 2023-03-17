#!/bin/sh -e

# The flow of changing is to first to rebase or merge with new parachain template
# Change variables in patches_top_config.sh
# Please update the submodules to new version of polkadot
# Please keep Cargo.toml file that must be patched, but insteat use this script and do patches_cmd/update.sh
#   you can edit patch and do `apply.sh patch_file`, and after this do update.sh to optimize patch
# Please change orml commit rev manually

old='0\.9\.37'
new='0.9.38'

list=`find -H node pallets runtime -name "Cargo.toml" -exec grep -l $old {} \;`
for i in $list
do
  echo $i
  # Uncommend this line if you ready
  # cat $i | sed "s,$old,$new,g" > $i.tmp && mv $i.tmp $i
done
