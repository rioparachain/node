# polkadot 0.9.19
curl -v localhost:9933/health
< HTTP/1.1 200 OK
{"isSyncing":true,"peers":4,"shouldHavePeers":true}
In
https://github.com/paritytech/substrate/issues/1017

# todo - investigate links
aws task definition --> Network settings --> links

# todo
 - https://www.proud2becloud.com/ecs-deployment-strategies-reduce-downtime-and-risk-with-blue-green-deployment/

# CONNECT TWO VPC's
 - https://us-east-1.console.aws.amazon.com/vpc/home?region=us-east-1#PeeringConnections:
 - https://docs.aws.amazon.com/vpc/latest/peering/what-is-vpc-peering.html
 - https://docs.aws.amazon.com/vpc/latest/peering/create-vpc-peering-connection.html

# Task roles and policies
 - https://aws.amazon.com/premiumsupport/knowledge-center/ecs-data-security-container-task/

# Troubleshooting

## CrossRegion
 - on ecr or secret manager errors (timeout) - attach NAT to VPC

# VPC peering
 - https://docs.aws.amazon.com/vpc/latest/userguide/vpc-peering.html
 - https://docs.aws.amazon.com/devicefarm/latest/developerguide/amazon-vpc-cross-region.html

