SHELL := /bin/bash

TF_VAR_service_name := image-generator
export TF_VAR_service_name

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
		&& source select_environment.sh ${stage} ${region} \
		&& tf apply ${terraform_args}

teardown:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	echo "Destroying ${TF_VAR_service_name} to ${stage}, region: ${region}"
	cd infrastructure \
		&& source select_environment.sh ${stage} ${region} \
		&& tf destroy ${terraform_args}

test:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	cd infrastructure \
		&& source select_environment.sh ${stage} ${region} \
		&& aws lambda invoke --region ${region} --function-name $$(tf output -raw function_name) --payload fileb://../test/events/checkerboard_api_gw.json /dev/stdout | cat
