#!/bin/sh
ln -sf ../../submodules/polkadot/node/client/src node/relaychain-rio-client/src
ln -sf ../../submodules/polkadot/cli/build.rs node/relaychain-rio-cli/build.rs
ln -sf ../../../submodules/polkadot/cli/src/lib.rs node/relaychain-rio-cli/src/lib.rs
ln -sf ../../../submodules/polkadot/cli/src/error.rs node/relaychain-rio-cli/src/error.rs
ln -sf ../../../submodules/polkadot/cli/src/host_perf_check.rs node/relaychain-rio-cli/src/host_perf_check.rs
ln -sf ../../../submodules/polkadot/cli/src/cli.rs node/relaychain-rio-cli/src/cli.rs
ln -sf ../../../submodules/polkadot/node/service/src/relay_chain_selection.rs node/relaychain-rio-service/src/relay_chain_selection.rs
ln -sf ../../../submodules/polkadot/node/service/src/parachains_db node/relaychain-rio-service/src/parachains_db
ln -sf ../../../submodules/polkadot/node/service/src/lib.rs node/relaychain-rio-service/src/lib.rs
ln -sf ../../../submodules/polkadot/node/service/src/overseer.rs node/relaychain-rio-service/src/overseer.rs
ln -sf ../../../submodules/polkadot/node/service/src/tests.rs node/relaychain-rio-service/src/tests.rs
ln -sf ../../../submodules/polkadot/node/service/src/grandpa_support.rs node/relaychain-rio-service/src/grandpa_support.rs
ln -sf ../../../submodules/polkadot/src/main.rs node/relaychain-rio/src/main.rs
ln -sf ../../../../submodules/polkadot/runtime/polkadot/constants/src/weights runtime/relaychain-rio/constants/src/weights
ln -sf ../../submodules/polkadot/runtime/polkadot/build.rs runtime/relaychain-rio/build.rs
ln -sf ../../../submodules/polkadot/runtime/polkadot/src/bag_thresholds.rs runtime/relaychain-rio/src/bag_thresholds.rs
ln -sf ../../../submodules/polkadot/runtime/polkadot/src/weights runtime/relaychain-rio/src/weights