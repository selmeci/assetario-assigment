resource "aws_appsync_datasource" "appsync" {
  api_id           = aws_appsync_graphql_api.assetario.id
  name             = "appsync"
  service_role_arn = aws_iam_role.assetario.arn
  type             = "AWS_LAMBDA"

  lambda_config {
    function_arn = aws_lambda_function.lambda.arn
  }
}

resource "aws_appsync_resolver" "tree" {
  api_id      = aws_appsync_graphql_api.assetario.id
  field       = "tree"
  type        = "Query"
  data_source = aws_appsync_datasource.appsync.name
}

resource "aws_appsync_resolver" "state" {
  api_id      = aws_appsync_graphql_api.assetario.id
  field       = "state"
  type        = "Query"
  data_source = aws_appsync_datasource.appsync.name
}

resource "aws_appsync_resolver" "country" {
  api_id      = aws_appsync_graphql_api.assetario.id
  field       = "country"
  type        = "Query"
  data_source = aws_appsync_datasource.appsync.name
}