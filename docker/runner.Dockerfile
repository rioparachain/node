# todo remove values

ARG FROM_IMAGE=chainspec
FROM ${FROM_IMAGE} AS CHAINSPEC

# copy only nessesary files, avoiding directories, to make more compact build
RUN mkdir -p /rio/release
RUN cp /rio/src/target/release/* /rio/release/ || true

# gives 2 high vulnerabilities
# - https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2022-28391
# - https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-3711
FROM nixos/nix:2.3.12@sha256:d9bb3b85b846eb0b6c5204e0d76639dff72c7871fb68f5d4edcfbb727f8a5653
# err: awk command not found
#FROM nixos/nix:latest

# It is necessary for the correct download of packages.
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# By default, the logic of the first failure for sh.
SHELL ["/bin/sh", "-xe", "-c"]

# We create a nix expression for the rust build environment, where fixed variables are used.
COPY ./docker/scripts/nix/pkgs.nix /pkgs.nix

# Making a nix expression for the side environment containing dependencies and env variables.
COPY ./docker/scripts/nix/shell.nix /shell.nix

RUN nix-env -i curl nodejs nfs-utils gnused gawk

WORKDIR /rio

COPY --from=CHAINSPEC /rio/chainspec ./chainspec
COPY --from=CHAINSPEC /rio/keys ./keys

COPY --from=CHAINSPEC /rio/src/docker/scripts/start_collator.sh ./start_collator.sh
COPY --from=CHAINSPEC /rio/src/docker/scripts/start_relay.sh ./start_relay.sh

COPY --from=CHAINSPEC /rio/release ./release

# detect current $NIX_LIB
# RUN ldd ./release/relaychain-rio
ARG NIX_LIB=/nix/store/z56jcx3j1gfyk4sv7g8iaan0ssbdkhz1-glibc-2.33-56
COPY --from=CHAINSPEC ${NIX_LIB} ${NIX_LIB}
