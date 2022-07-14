ARG IMAGE_POLKADOT=polkadot
FROM ${IMAGE_POLKADOT} AS POLKADOT

WORKDIR /rio/src
RUN cp -r ./target /target
COPY . .
RUN cp -r /target .
RUN ls -lA ./target

ENTRYPOINT ["nix-shell", "/shell.nix"]
