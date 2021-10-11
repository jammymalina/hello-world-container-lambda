SHELL := /bin/bash

TF_VAR_service_name := image-generator
export TF_VAR_service_name

terraform_init = terraform init -backend-config=environments/$(1)/backend.config -backend-config="key=$(1)/$(2)/${TF_VAR_service_name}" -backend-config="region=$(2)"
terraform_apply = terraform apply -var-file=environments/$(1)/variables.tfvars $(2)
terraform_destroy = terraform destroy -var-file=environments/$(1)/variables.tfvars $(2)


check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))

.PHONY: all test clean

deploy:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	echo "Deploying ${TF_VAR_service_name} to ${stage}, region: ${region}"
	cd infrastructure \
		&& export TF_VAR_region=${region} && export TF_VAR_stage=${stage} \
		&& $(call terraform_init,$(strip ${stage}),$(strip ${region})) \
		&& $(call terraform_apply,$(strip ${stage}),$(strip ${terraform_args}))

teardown:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	echo "Destroying ${TF_VAR_service_name}, ${stage}, region: ${region}"
	cd infrastructure \
		&& export TF_VAR_region=${region} && export TF_VAR_stage=${stage} \
		&& $(call terraform_init,$(strip ${stage}),$(strip ${region})) \
		&& $(call terraform_destroy,$(strip ${stage}),$(strip ${terraform_args}))

test:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	cd infrastructure \
		&& export TF_VAR_region=${region} && export TF_VAR_stage=${stage} \
		&& $(call terraform_init,$(strip ${stage}),$(strip ${region})) \
		&& aws lambda invoke --region ${region} --function-name $$(terraform output -raw function_name) --payload fileb://../test/events/checkerboard_api_gw.json /dev/stdout | cat
