SHELL := /bin/bash

TF_VAR_service_name := image-generator
export TF_VAR_service_name

terraform_init = terraform -chdir=infrastructure init -backend-config=environments/$(1)/backend.config -backend-config="key=$(1)/$(2)/${TF_VAR_service_name}" -backend-config="region=$(2)"
terraform_apply = terraform -chdir=infrastructure apply -var-file=environments/$(1)/variables.tfvars $(2)
terraform_destroy = terraform -chdir=infrastructure destroy -var-file=environments/$(1)/variables.tfvars $(2)
terraform_output = terraform -chdir=infrastructure output -raw $(1)



check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))

.PHONY: all test clean

init:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	@echo "Initializing ${TF_VAR_service_name}, ${stage}, region: ${region}"
	@$(call terraform_init,$(strip ${stage}),$(strip ${region}))

deploy:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	@echo "Deploying ${TF_VAR_service_name} to ${stage}, region: ${region}"
	@export TF_VAR_region=${region} && export TF_VAR_stage=${stage} \
		&& $(call terraform_init,$(strip ${stage}),$(strip ${region})) \
		&& $(call terraform_apply,$(strip ${stage}),$(strip ${terraform_args}))

teardown:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	@echo "Destroying ${TF_VAR_service_name}, ${stage}, region: ${region}"
	@export TF_VAR_region=${region} && export TF_VAR_stage=${stage} \
		&& $(call terraform_init,$(strip ${stage}),$(strip ${region})) \
		&& $(call terraform_destroy,$(strip ${stage}),$(strip ${terraform_args}))

test:
	@rm -rf test/outputs && mkdir -p test/outputs
	@curl --silent --output test/outputs/checkerboard.png --location --request GET "$$($(call terraform_output,api_endpoint))/checkerboard?width=200&height=200&cellWidth=40&cellHeight=40"
	@compare -metric AE test/outputs/checkerboard.png test/snapshots/checkerboard.png null: &>/dev/null; \
		if [ $$? -eq 0 ]; then \
			echo "The generated image matches the snapshot"; \
		else \
			echo "The generated image does NOT match the snapshot, error occured"; \
		fi
