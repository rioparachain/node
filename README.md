# ..:: Rio Core Network ::..

## TESTNET ACCESS POINTS

### WS

[Relay Chain](wss://rio-testnet-relay.riocorenetwork.com) /
[ParaChain](wss://rio-testnet-collator.riocorenetwork.com)

### RPC

[Relay Chain](https://rio-testnet-relay-rpc.riocorenetwork.com/health) /
[ParaChain](https://rio-testnet-collator-rpc.riocorenetwork.com/health)

### Telemetry

[Relay Chain](http://3.89.91.186:3000/#list/0xdfa36d69aa63a31410cd8bc47af78f4f27743aad6847c4b00e94ba70ba2587d6) /
[ParaChain](http://3.89.91.186:3000/#list/0xa79d4d745152595c699bc1a2111993c775bce65218855dd0c5d5dd4a721f7045)

### Chain Info

[Relay Chain](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frio-testnet-relay.riocorenetwork.com#/explorer) /
[ParaChain](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frio-testnet-collator.riocorenetwork.com#/explorer)

## DEVELOPMENT INSTRUCTIONS

### BUILD

```shell
export MNEMONIC="..." && \
export AIRDROP_SCAN_PAGES=10 && \
docker-compose -f ./docker-compose-build.yml build --no-cache \
	--build-arg "MNEMONIC=${MNEMONIC}" \
	--build-arg "AIRDROP_SCAN_PAGES=${AIRDROP_SCAN_PAGES}"

```

### START

```shell
docker-compose -f ./docker-compose-build.yml up chainspec && \
docker-compose up
```

## MORE

### [RELEASE NOTES](./RELEASE_NOTES.md)

### [WALLET EXTENTION](https://rio-wallet-extension.s3.amazonaws.com/build.7z)
