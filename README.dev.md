## Links
 - [Nix vars to improve build speed](https://nixos.org/manual/nix/stable/command-ref/env-common.html)

## Development notes
### Port mapping
```shell
ssh -L 41001:18.185.111.85:41001 -C -N -l ubuntu 18.185.111.85
```
### Light build
```shell
aws configure
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 061416964074.dkr.ecr.us-east-1.amazonaws.com/rio-node
docker pull 061416964074.dkr.ecr.us-east-1.amazonaws.com/rio-node:00476378
docker tag 061416964074.dkr.ecr.us-east-1.amazonaws.com/rio-node:00476378 runner
docker-compose down && docker-compose up
```

## Key distributor

ssh ubuntu@54.205.90.136

sshfs ubuntu@54.205.90.136:/home/ubuntu/distribute-key ./docker/distributor_mount -o 'allow_other,gid=10067' -ovolname=remote

umount ./docker/distributor_mount


## Ci/CD runner + Dev Machine

ssh root@172.105.247.128

## Links
- [Nix vars to improve build speed](https://nixos.org/manual/nix/stable/command-ref/env-common.html)
