

output "api-key" {
  value = nonsensitive(aws_appsync_api_key.assetario.key)
}