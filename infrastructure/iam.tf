
resource "aws_iam_role" "assetario" {
  name = "assetario"

  assume_role_policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Principal" : {
          "Service" : "appsync.amazonaws.com"
        },
        "Action" : "sts:AssumeRole"
      },
      {
        "Action" : "sts:AssumeRole",
        "Principal" : {
          "Service" : "lambda.amazonaws.com"
        },
        "Effect" : "Allow",
        "Sid" : ""
      }
    ]
  })
}

resource "aws_iam_role_policy" "appsync" {
  name = "assetarioLambda"
  role = aws_iam_role.assetario.id

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : [
          "lambda:InvokeFunction"
        ],
        "Effect" : "Allow",
        "Resource" : [
          "${aws_lambda_function.lambda.arn}*"
        ]
      }
    ]
  })
}

resource "aws_iam_policy" "lambda_logging" {
  name = "assetario-appsync-logging"

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ],
        "Resource" : "arn:aws:logs:*:*:*",
        "Effect" : "Allow"
      },
      {
        "Action" : [
          "xray:PutTraceSegments",
          "xray:PutTelemetryRecords"
        ],
        "Resource" : "*",
        "Effect" : "Allow"
      }
    ]
  })
}

resource "aws_iam_role_policy" "lambda" {
  name = "assetarioAppsync"
  role = aws_iam_role.assetario.id

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : [
          "dynamodb:*"
        ],
        "Effect" : "Allow",
        "Resource" : [
          "${aws_dynamodb_table.simplemaps.arn}*"
        ]
      },
      {
        "Action" : [
          "s3:*"
        ],
        "Effect" : "Allow",
        "Resource" : [
          "${aws_s3_bucket.assetario.arn}*"
        ]
      }
    ]
  })
}