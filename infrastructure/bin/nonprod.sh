echo "Creating non-prod infrastructure..."
aws cloudformation create-stack --stack-name rio-node-ecr --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-ecr.yml --region us-east-1
aws cloudformation create-stack --stack-name rio-node-cluster --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-cluster.yml --region us-east-1
aws cloudformation create-stack --stack-name rio-node-role --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-role.yaml --region us-east-1
aws cloudformation update-stack --stack-name rio-node-role --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-role-update.yaml --region us-east-1
sleep 60
aws cloudformation create-stack --stack-name rio-node-pipeline --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-pipeline.yaml --region us-east-1
aws cloudformation update-stack --stack-name rio-node-pipeline --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-pipeline.yaml --region us-east-1
---
aws cloudformation create-stack --stack-name rio-node-ecr --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-ecr.yml --region us-west-2
aws cloudformation create-stack --stack-name rio-node-cluster --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-cluster.yml --region us-west-2
---
aws cloudformation create-stack --stack-name rio-node-ecr --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-ecr.yml --region eu-central-1
aws cloudformation create-stack --stack-name rio-node-cluster --capabilities CAPABILITY_NAMED_IAM --template-body file://infrastructure/rio-node-cluster.yml --region eu-central-1
---

aws secretsmanager get-secret-value --secret-id arn:aws:secretsmanager:us-west-2:061416964074:secret:rio-node/testnet-MGuWnE --region us-west-2
