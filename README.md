# BUILD
> docker-compose -f ./docker-compose-build.yml build
>
> docker-compose -f ./docker-compose-build.yml up chainspec

# START
> docker-compose up
or single
> docker-compose up relay-node-01

# Links
    Nix vars to improve build speed
    - https://nixos.org/manual/nix/stable/command-ref/env-common.html

# Port mapping
> ssh -L 41001:18.185.111.85:41001 -C -N -l ubuntu 18.185.111.85


# Light
 - login
> aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 061416964074.dkr.ecr.us-east-1.amazonaws.com/rio-node

> docker-compose up

