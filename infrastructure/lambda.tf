
resource "null_resource" "cargo" {

  triggers = {
    refresh = timestamp()
  }

  provisioner "local-exec" {
    interpreter = ["/bin/bash", "-c"]

    command = <<EOF
      cd ..
      cargo lambda build --release --arm64
    EOF
  }

}

data "archive_file" "bootstrap" {
  depends_on = [null_resource.cargo]

  type        = "zip"
  output_path = "appsync.zip"
  source_file = "../target/lambda/appsync/bootstrap"
}

resource "aws_cloudwatch_log_group" "lambda" {
  name              = "/aws/lambda/assetario"
  retention_in_days = 1
}

resource "aws_iam_role_policy_attachment" "lambda" {
  role       = aws_iam_role.assetario.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}

resource "aws_lambda_function" "lambda" {
  filename      = "appsync.zip"
  handler       = "bootstrap"
  architectures = ["arm64"]
  function_name = "assetario-appsync"
  role          = aws_iam_role.assetario.arn
  timeout       = 30
  memory_size   = 128
  runtime       = "provided.al2"
  source_code_hash = data.archive_file.bootstrap.output_base64sha256

  depends_on = [
    data.archive_file.bootstrap,
    aws_iam_role_policy_attachment.lambda,
    aws_cloudwatch_log_group.lambda,
  ]

  environment {
    variables = {
      env : "demo"
      company : "assetario"
    }
  }
}