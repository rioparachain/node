# Using a fixed hash of the nix distribution for docker.
ARG NIX_VERSION
ARG NIX_TAG
FROM nixos/nix:${NIX_VERSION}@sha256:${NIX_TAG} AS NIX

# It is necessary for the correct download of packages.
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# By default, the logic of the first failure for sh.
SHELL ["/bin/sh", "-xe", "-c"]

# We create a nix expression for the rust build environment, where fixed variables are used.
COPY ./docker/nix/pkgs.nix /pkgs.nix

# Making a nix expression for the side environment containing dependencies and env variables.
COPY ./docker/nix/shell.nix /shell.nix

RUN nix-env -i nodejs git rsync lrzip

FROM NIX AS BUILD

## Preparing Utilities.
## We force it to execute on the side of the environment without executing the build node commands themselves.
RUN nix-shell /shell.nix --run true

#RUN nix-env -i dprint cargo-sort

ARG POLKADOT_VERSION
ARG ORML_REV
ENV POLKADOT_VERSION=$POLKADOT_VERSION
ENV POLKADOT_BRANCH=release-$POLKADOT_VERSION
ENV CUMULUS_BRANCH=polkadot-$POLKADOT_VERSION
ENV SUBSTRATE_BRANCH=polkadot-$POLKADOT_VERSION
ENV ORML_REV=$ORML_REV

WORKDIR /rio/src
COPY . .

RUN nix-shell /shell.nix --run "./patches_cmds/subdir_apply_all.sh ."

RUN nix-shell /shell.nix --run 'cargo build --release --features fast-runtime --features rio-testnet'

# Make hardlink to use as tool, and also create combo archive.
RUN mv target/release/parachain-rio .; rm -Rf target; rm -Rf submodules
RUN mkdir -p target/release; mv parachain-rio target/release
RUN cd target/release; ln -f parachain-rio relaychain-rio

# Extract lib deps and copy store paths to `/export`.
RUN nix-shell /shell.nix --run "ldd target/release/parachain-rio" \
      | awk -F '/nix/store/' '{ print $2 }' | awk '{ print $1 }' \
      | sort | uniq > nix_store_paths.txt; \
    for path in `cat nix_store_paths.txt`; do \
      mkdir -p `dirname /export/nix/store/$path`; \
      cp -Lp /nix/store/$path /export/nix/store/$path; \
    done; \
    rm nix_store_paths.txt

ENTRYPOINT ["nix-shell", "/shell.nix"]

