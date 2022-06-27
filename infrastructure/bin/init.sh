aws s3api create-bucket --bucket rio-artifacts-us-east-1 --region us-east-1
aws s3api create-bucket --bucket rio-artifacts-us-east-1/rio-node-codebuild-cache --region us-east-1

aws s3api put-object --bucket rio-artifacts-us-east-1 --key rio-node-codebuild-cache/



# DONT> # create new Customer-managed keys manually
# replace in json -> Resource: # Use AWS->KMS->AWS managed keys->aws/codecommit
aws iam create-policy --policy-name non-prod-artifact-access-policy --policy-document file://infrastructure/rio-policy.json --region us-east-1

#use update instead
#aws iam delete-policy --policy-arn arn:aws:iam::061416964074:policy/non-prod-artifact-access-policy
#aws iam get-policy --policy-arn arn:aws:iam::061416964074:policy/non-prod-artifact-access-policy

# create EFS on deployed VPC
# - https://us-east-1.console.aws.amazon.com/efs/home?region=us-east-1#/file-systems
