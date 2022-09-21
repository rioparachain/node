# ..:: Rio Core Network ::..

## [RELEASE NOTES](./RELEASE_NOTES.md)

## TESTNET URL's

### WS
 - [Relay](wss://rio-testnet-relay.riocorenetwork.com)
 - [Parachain](wss://rio-testnet-collator.riocorenetwork.com)

### RPC (health-check)
 - [Relay](https://rio-testnet-relay-rpc.riocorenetwork.com/health)
 - [Parachain](https://rio-testnet-collator-rpc.riocorenetwork.com/health)

### Telemetry
 - [Relay](http://3.89.91.186:3000/#list/0xdfa36d69aa63a31410cd8bc47af78f4f27743aad6847c4b00e94ba70ba2587d6)
 - [Parachain](http://3.89.91.186:3000/#list/0xa79d4d745152595c699bc1a2111993c775bce65218855dd0c5d5dd4a721f7045)

### Polkadot
 - [Relay Chain Info](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frio-testnet-relay.riocorenetwork.com#/explorer)
 - [ParaChain Info](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frio-testnet-collator.riocorenetwork.com#/explorer)

## BUILD

```shell
export MNEMONIC="..." && \
export AIRDROP_SCAN_PAGES=10 && \
docker-compose -f ./docker-compose-build.yml build --no-cache \
	--build-arg "MNEMONIC=${MNEMONIC}" \
	--build-arg "AIRDROP_SCAN_PAGES=${AIRDROP_SCAN_PAGES}"

```

## START

```shell
docker-compose -f ./docker-compose-build.yml up chainspec && \
docker-compose up
```

## Links
 - [Nix vars to improve build speed](https://nixos.org/manual/nix/stable/command-ref/env-common.html)
