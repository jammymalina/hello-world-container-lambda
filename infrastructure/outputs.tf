output "function_name" {
  description = "Name of the Lambda function"
  value       = aws_lambda_function.lambda_function.function_name
}

output "api_id" {
  description = "ApiGateway ID"
  value       = aws_apigatewayv2_api.api_gw_api.id
}

output "api_endpoint" {
  description = "ApiGateway endpoint"
  value       = aws_apigatewayv2_api.api_gw_api.api_endpoint
}
