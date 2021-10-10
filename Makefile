SHELL := /bin/bash

TF_VAR_service_name := image-generator
export TF_VAR_service_name

check_defined = \
    $(strip $(foreach 1,$1, \
        $(call __check_defined,$1,$(strip $(value 2)))))
__check_defined = \
    $(if $(value $1),, \
      $(error Undefined $1$(if $2, ($2))))


deploy:
	@:$(call check_defined, stage, stage name)
	@:$(call check_defined, region, AWS region)
	terraform_args=(${terraform_args})
	echo "Deploying ${TF_VAR_service_name} to ${stage}, region: ${region}"
	cd infrastructure \
		&& source select_environment.sh ${stage} ${region} \
		&& tf init \
		&& tf apply -chdir=infrastructure ${terraform_args[@]}
