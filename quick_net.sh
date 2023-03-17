#!/bin/sh
trgdir=./target/release
parabin=$trgdir/parachain-template-node
polkabin=$trgdir/polkadot
cp $parabin $polkabin
#tmp=`mktemp -d`
tmp=/tmp/mytmp
tmp2=/tmp/mytmp2
mkdir -p /tmp/mytmp
rm -Rf /tmp/mytmp/*

$polkabin build-spec --chain polkadot-local --disable-default-bootnode > $tmp/polkadot_plain_orig.json
$parabin build-spec --disable-default-bootnode > $tmp/para_plain_orig.json

node <<EOF
const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const cs = JSONbig.parse(fs.readFileSync("$tmp/para_plain_orig.json").toString());
cs.para_id = 1000;
cs.relay_chain = 'polkadot-local';
fs.writeFileSync("$tmp/para_plain.json", JSONbig.stringify(cs, null, '  '));
EOF

$parabin build-spec --chain $tmp/para_plain.json --raw --disable-default-bootnode > $tmp/para_raw.json
$parabin export-genesis-wasm --chain $tmp/para_raw.json > $tmp/para_wasm
$parabin export-genesis-state --chain $tmp/para_raw.json > $tmp/para_genst

node <<EOF
const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const cs = JSONbig.parse(fs.readFileSync("$tmp/polkadot_plain_orig.json").toString());
const paraWasm = fs.readFileSync("$tmp/para_wasm").toString();
const paraGenesis = fs.readFileSync("$tmp/para_genst").toString();
cs.genesis.runtime.paras = [[[1000, {
  genesis_head: paraGenesis,
  validation_code: paraWasm,
  parachain: true,
}]]];
fs.writeFileSync("$tmp/polkadot_plain.json", JSONbig.stringify(cs, null, '  '));
EOF

$polkabin build-spec --chain $tmp/polkadot_plain.json --raw --disable-default-bootnode > $tmp/polkadot_raw.json

#relay01nk=`$polkabin key generate-node-key --bin --file $tmp2/relay01.nk 2>&1`
#relay02nk=`$polkabin key generate-node-key --bin --file $tmp2/relay02.nk 2>&1`
#relay03nk=`$polkabin key generate-node-key --bin --file $tmp2/relay03.nk 2>&1`
#col01nk=`$polkabin key generate-node-key --bin --file $tmp2/col01.nk 2>&1`
#col02nk=`$polkabin key generate-node-key --bin --file $tmp2/col02.nk 2>&1`

relay01nk=`$polkabin key inspect-node-key --bin --file $tmp2/relay01.nk 2>&1`
relay02nk=`$polkabin key inspect-node-key --bin --file $tmp2/relay02.nk 2>&1`
relay03nk=`$polkabin key inspect-node-key --bin --file $tmp2/relay03.nk 2>&1`
col01nk=`$polkabin key inspect-node-key --bin --file $tmp2/col01.nk 2>&1`
col02nk=`$polkabin key inspect-node-key --bin --file $tmp2/col02.nk 2>&1`

#echo "BOOTRELAY1 /ip4/127.0.0.1/tcp/9401/ws/p2p/$relay01nk"
#echo "BOOTRELAY2 /ip4/127.0.0.1/tcp/9402/ws/p2p/$relay02nk"
#echo "BOOTRELAY3 /ip4/127.0.0.1/tcp/9403/ws/p2p/$relay03nk"
#echo "BOOTCOL1 /ip4/127.0.0.1/tcp/9421/ws/p2p/$col01nk"
#echo "BOOTCOL2 /ip4/127.0.0.1/tcp/9422/ws/p2p/$col02nk"
#exit

#export RUST_LOG="cumulus-collator=debug"
#export RUST_LOG=runtime=debug

$polkabin --unsafe-ws-external --unsafe-rpc-external --no-prometheus --no-telemetry --rpc-cors all --rpc-methods Unsafe \
          --ws-port 9101 --rpc-port 9201 --node-key-file $tmp2/relay01.nk \
          --listen-addr "/ip4/127.0.0.1/tcp/9301" \
          --listen-addr "/ip4/127.0.0.1/tcp/9401/ws" \
          --bootnodes /ip4/127.0.0.1/tcp/9302/p2p/$relay02nk \
          --base-path $tmp/rel01 \
          --alice --chain $tmp/polkadot_raw.json > $tmp/relay01.log 2>&1 &
          #--listen-addr "/ip4/0.0.0.0/tcp/9401/ws" \

$polkabin --unsafe-ws-external --unsafe-rpc-external --no-prometheus --no-telemetry --rpc-cors all --rpc-methods Unsafe \
          --ws-port 9102 --rpc-port 9202 --node-key-file $tmp2/relay02.nk \
          --listen-addr "/ip4/127.0.0.1/tcp/9302" \
          --listen-addr "/ip4/127.0.0.1/tcp/9402/ws" \
          --bootnodes /ip4/127.0.0.1/tcp/9301/p2p/$relay01nk \
          --base-path $tmp/rel02 \
          --bob --chain $tmp/polkadot_raw.json > $tmp/relay02.log 2>&1 &
          #--listen-addr "/ip4/0.0.0.0/tcp/9402/ws" \

$polkabin --unsafe-ws-external --unsafe-rpc-external --no-prometheus --no-telemetry --rpc-cors all --rpc-methods Unsafe \
          --ws-port 9103 --rpc-port 9203 --node-key-file $tmp2/relay03.nk \
          --listen-addr "/ip4/127.0.0.1/tcp/9303/ws" \
          --listen-addr "/ip4/127.0.0.1/tcp/9403/ws" \
          --bootnodes /ip4/127.0.0.1/tcp/9301/p2p/$relay01nk \
          --bootnodes /ip4/127.0.0.1/tcp/9302/p2p/$relay02nk \
          --base-path $tmp/rel03 \
          --chain $tmp/polkadot_raw.json > $tmp/relay03.log 2>&1 &
          #--listen-addr "/ip4/127.0.0.1/tcp/9303/ws" \
          #--listen-addr "/ip4/0.0.0.0/tcp/9402/ws" \

#export RUST_LOG=debug

$parabin --collator --unsafe-ws-external --unsafe-rpc-external --no-prometheus --no-telemetry --rpc-cors all --rpc-methods Unsafe \
         --ws-port 9121 --rpc-port 9221 --node-key-file $tmp2/col01.nk \
         --listen-addr "/ip4/127.0.0.1/tcp/9321" \
         --listen-addr "/ip4/127.0.0.1/tcp/9421/ws" \
         --alice --chain $tmp/para_raw.json \
         --base-path $tmp/col01 \
         --bootnodes /ip4/127.0.0.1/tcp/9322/p2p/$col02nk \
         -- \
         --bootnodes /ip4/127.0.0.1/tcp/9301/p2p/$relay01nk \
         --bootnodes /ip4/127.0.0.1/tcp/9302/p2p/$relay02nk \
         --chain $tmp/polkadot_raw.json > $tmp/col01.log 2>&1 &

         #-lruntime=debug \
         #--execution wasm \

#export RUST_LOG=

$parabin --collator --unsafe-ws-external --unsafe-rpc-external --no-prometheus --no-telemetry --rpc-cors all --rpc-methods Unsafe \
         --ws-port 9122 --rpc-port 9222 --node-key-file $tmp2/col02.nk \
         --listen-addr "/ip4/127.0.0.1/tcp/9322" \
         --listen-addr "/ip4/127.0.0.1/tcp/9422/ws" \
         --bob --chain $tmp/para_raw.json \
         --base-path $tmp/col02 \
         --bootnodes /ip4/127.0.0.1/tcp/9321/p2p/$col01nk \
         -- \
         --bootnodes /ip4/127.0.0.1/tcp/9301/p2p/$relay01nk \
         --bootnodes /ip4/127.0.0.1/tcp/9302/p2p/$relay02nk \
         --chain $tmp/polkadot_raw.json

         #--execution wasm \

# > $tmp/col01.log 2>&1 &
 
wait


