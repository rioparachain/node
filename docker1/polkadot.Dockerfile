# Using a fixed hash of the nix distribution for docker.
ARG NIX_VERSION
ARG NIX_TAG
FROM nixos/nix:${NIX_VERSION}@sha256:${NIX_TAG}

# It is necessary for the correct download of packages.
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# By default, the logic of the first failure for sh.
SHELL ["/bin/sh", "-xe", "-c"]

# We create a nix expression for the rust build environment, where fixed variables are used.
COPY ./docker1/scripts/nix/pkgs.nix /pkgs.nix

# Making a nix expression for the side environment containing dependencies and env variables.
COPY ./docker1/scripts/nix/shell.nix /shell.nix

# Preparing Utilities.
# We force it to execute on the side of the environment without executing the build node commands themselves.
RUN nix-shell /shell.nix --run true
