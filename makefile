# Parameters
DOMAIN_NAME := sanjams.dev

# Constants
STACK_NAME := "$(shell echo "sanjams.dev" | tr '.' '-')-website-infra"
# CloudFront only support certificates in us-east-1
AWS_REGION := us-east-1

.PHONY: local cfn-init cfn-update release invalidate

local:
	open site/index.html

export AWS_PROFILE := prsn
cfn-init:
	aws --region $(AWS_REGION) cloudformation create-stack \
		--stack-name $(STACK_NAME) \
		--template-body file://configuration/template.yaml \
		--parameter \
			ParameterKey="DomainName",ParameterValue=$(DOMAIN_NAME)

export AWS_PROFILE := prsn
cfn-update:
	aws --region $(AWS_REGION) cloudformation update-stack \
		--stack-name $(STACK_NAME) \
		--template-body file://configuration/template.yaml \
		--parameter \
			ParameterKey="DomainName",ParameterValue=$(DOMAIN_NAME)

export AWS_PROFILE := prsn
release:
	aws s3 cp site s3://$(DOMAIN_NAME) \
		--recursive \
		--cache-control max-age=86400

export AWS_PROFILE := prsn
invalidate:
	aws cloudfront create-invalidation \
		--distribution-id $(shell aws --region $(AWS_REGION) --profile $(AWS_PROFILE) cloudformation describe-stacks --stack-name $(STACK_NAME) --query "Stacks[0].Outputs[?OutputKey=='CloudfrontDistributionId'].OutputValue" --output text) \
		--paths "/*"
