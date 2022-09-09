# Release Notes

## Release 0.1 (December 2021)
* upgraded substrate version from 2.0 to 3.0, upgraded web wallet to support 3.0 API
* introduced full EVM support (based on Astar)
* airdrop (automate chain spec configuration to include balances from the old network)
* automated build process (compile + run tests + configuration = build)

## Release 0.2 (March 2022)
* automated deployment processes (AWS building process triggered by the commit
    into "develop" branch)
* MantraDAO-like staking released (implemented)

## Release 0.3 (June 2022)
* introduced Relay Chain with the built-in (included into chain spec) Parachain;
    * two binaries available: Relay Chain (Validators) binary and Parachain (Collators) binary
    * reconfigured automated build and deployment processes
* launched telemetry support
* optimized binary upgrade process by introducing patching mechanism
* upgraded to Polkadot v0.9.24

## Release 1.0 (September 2022)
* Rio wallet browser extension released
* improved EVM support by adding the Frontier pallet directly (bypassing Astar)
* improved the automated deployment process (introduced archive nodes support, and graceful
    validator replacement)
