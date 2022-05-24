#!/bin/sh -xe

#if [ ! -d ./target ]
#then
    nix-shell /shell.nix --run 'cargo build --release --features fast-runtime '
#fi
