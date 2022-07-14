FROM chainspec AS CHAINSPEC
FROM polkadot AS POLKADOT

FROM alpine:latest

# By default, the logic of the first failure for sh.
SHELL ["/bin/sh", "-xe", "-c"]

RUN apk add --no-cache curl nodejs nfs-utils sed gawk

WORKDIR /rio

COPY --from=POLKADOT  /export/nix/store /nix/store
COPY --from=CHAINSPEC /rio/src/target/release/parachain-rio ./release/parachain-rio
COPY --from=CHAINSPEC /rio/src/target/release/relaychain-rio ./release/relaychain-rio

COPY --from=CHAINSPEC /rio/chainspec ./chainspec
COPY --from=CHAINSPEC /rio/keys ./keys

COPY --from=CHAINSPEC /rio/src/docker/scripts/start_collator.sh ./start_collator.sh
COPY --from=CHAINSPEC /rio/src/docker/scripts/start_relay.sh ./start_relay.sh

