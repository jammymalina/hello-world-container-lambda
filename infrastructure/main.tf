terraform {
  backend "s3" {}
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "3.62.0"
    }
  }
}

provider "aws" {
  region = var.region
}

data "aws_caller_identity" "current" {}

locals {
  root_dir   = "${path.module}/.."
  account_id = data.aws_caller_identity.current.account_id

  default_memory_size = 128
  default_timeout     = 10
  lambda_functions = {
    checkerboard = {
      function_name = "checkerboard-lambda"
      memory_size   = local.default_memory_size
      timeout       = local.default_timeout
      build_args    = "--build-arg binary=checkerboard --build-arg stage=${var.stage} --build-arg log_level=${var.log_level}"
    }
  }
}

resource "aws_lambda_function" "lambda_function" {
  function_name = "${var.service_name}-${local.lambda_functions["checkerboard"].function_name}-${var.stage}"

  image_uri    = "${aws_ecr_repository.lambda_repository.repository_url}@${data.aws_ecr_image.lambda_image.id}"
  package_type = "Image"

  timeout     = local.lambda_functions["checkerboard"].timeout
  memory_size = local.lambda_functions["checkerboard"].memory_size
  role        = aws_iam_role.lambda_role.arn
}

resource "aws_ecr_repository" "lambda_repository" {
  name = "${var.service_name}-${local.lambda_functions["checkerboard"].function_name}-${var.stage}"
}

resource "null_resource" "lambda_ecr_image_builder" {
  triggers = {
    docker_file     = filesha256("${local.root_dir}/Dockerfile")
    cargo_file      = filesha256("${local.root_dir}/Cargo.toml")
    cargo_lock_file = filesha256("${local.root_dir}/Cargo.lock")
    src_dir         = sha256(join("", [for f in fileset("${local.root_dir}/src", "**") : filesha256("${local.root_dir}/src/${f}")]))
  }

  provisioner "local-exec" {
    working_dir = local.root_dir
    interpreter = ["/bin/bash", "-c"]
    command     = <<-EOT
      aws ecr get-login-password --region ${var.region} | docker login --username AWS --password-stdin ${local.account_id}.dkr.ecr.${var.region}.amazonaws.com
      docker image build -t ${aws_ecr_repository.lambda_repository.repository_url}:latest ${local.lambda_functions["checkerboard"].build_args} .
      docker push ${aws_ecr_repository.lambda_repository.repository_url}:latest
    EOT
  }
}

data "aws_ecr_image" "lambda_image" {
  depends_on = [
    null_resource.lambda_ecr_image_builder
  ]

  repository_name = "${var.service_name}-${local.lambda_functions["checkerboard"].function_name}-${var.stage}"
  image_tag       = "latest"
}


resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${aws_lambda_function.lambda_function.function_name}"
  retention_in_days = var.log_retention_in_days
}

resource "aws_iam_role" "lambda_role" {
  name = "${var.service_name}-${local.lambda_functions["checkerboard"].function_name}-iam-role-${var.region}-${var.stage}"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = ""
        Effect = "Allow"
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "basic_lambda_policy" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

# -----------------------------------------------------------------------------------------------------

resource "aws_apigatewayv2_api" "api_gw_api" {
  name          = "image-generator-api-${var.stage}"
  protocol_type = "HTTP"

  cors_configuration {
    allow_origins = ["*"]
  }
}

resource "aws_apigatewayv2_stage" "api_gw_stage" {
  api_id = aws_apigatewayv2_api.api_gw_api.id

  name        = "$default"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.api_gw_log_group.arn

    format = jsonencode({
      requestId               = "$context.requestId"
      sourceIp                = "$context.identity.sourceIp"
      requestTime             = "$context.requestTime"
      protocol                = "$context.protocol"
      httpMethod              = "$context.httpMethod"
      resourcePath            = "$context.resourcePath"
      routeKey                = "$context.routeKey"
      status                  = "$context.status"
      responseLength          = "$context.responseLength"
      integrationErrorMessage = "$context.integrationErrorMessage"
      }
    )
  }
}

resource "aws_apigatewayv2_integration" "api_gw_integration" {
  api_id = aws_apigatewayv2_api.api_gw_api.id

  integration_uri    = aws_lambda_function.lambda_function.invoke_arn
  integration_type   = "AWS_PROXY"
  integration_method = "POST"

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "checkerboard_route" {
  api_id = aws_apigatewayv2_api.api_gw_api.id

  route_key = "GET /checkerboard"
  target    = "integrations/${aws_apigatewayv2_integration.api_gw_integration.id}"
}

resource "aws_cloudwatch_log_group" "api_gw_log_group" {
  name              = "/aws/api_gw/${aws_apigatewayv2_api.api_gw_api.name}"
  retention_in_days = var.log_retention_in_days
}

resource "aws_lambda_permission" "api_gw_lambda_permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  function_name = aws_lambda_function.lambda_function.function_name

  source_arn = "${aws_apigatewayv2_api.api_gw_api.execution_arn}/*/*"
}
