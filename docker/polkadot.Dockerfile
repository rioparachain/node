# Using a fixed hash of the nix distribution for docker.
ARG NIX_VERSION
ARG NIX_TAG
FROM nixos/nix:${NIX_VERSION}@sha256:${NIX_TAG} AS NIX

# It is necessary for the correct download of packages.
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# By default, the logic of the first failure for sh.
SHELL ["/bin/sh", "-xe", "-c"]

# We create a nix expression for the rust build environment, where fixed variables are used.
COPY ./docker/scripts/nix/pkgs.nix /pkgs.nix

# Making a nix expression for the side environment containing dependencies and env variables.
COPY ./docker/scripts/nix/shell.nix /shell.nix

RUN nix-env -i nodejs git

## Preparing Utilities.
## We force it to execute on the side of the environment without executing the build node commands themselves.
#RUN nix-shell /shell.nix --run true

FROM NIX AS BUILD

ARG POLKADOT_VERSION
ENV POLKADOT_VERSION=$POLKADOT_VERSION
ENV POLKADOT_BRANCH=release-$POLKADOT_VERSION
ENV CUMULUS_BRANCH=polkadot-$POLKADOT_VERSION

WORKDIR /rio/src
COPY . .

# submodule - works slow
#RUN git submodule update --init --remote
# this way is faster
RUN mkdir -p submodules || true
RUN git clone -b $POLKADOT_BRANCH --single-branch --depth 1 https://github.com/paritytech/polkadot submodules/polkadot
RUN git clone -b $CUMULUS_BRANCH --single-branch --depth 1 https://github.com/paritytech/cumulus submodules/cumulus
RUN nix-shell /shell.nix --run ./scripts/patches_apply.sh
RUN nix-shell /shell.nix --run ./cumulus_patch.sh

RUN nix-shell /shell.nix --run 'cargo build --release --features fast-runtime'
