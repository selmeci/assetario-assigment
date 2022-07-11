variable "region" {
  default = "eu-central-1"
}

terraform {
  backend "s3" {
    bucket = "selma-solutions"
    key    = "tf/assetario.tf"
    region = "eu-central-1"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.14"
    }
    archive = {
      source  = "hashicorp/archive"
      version = "~> 2.2"
    }
    null = {
      source  = "hashicorp/null"
      version = "~> 3.1"
    }
  }
}

resource "aws_appsync_graphql_api" "assetario" {
  authentication_type = "API_KEY"
  name                = "assetario"

  schema = file("../schema.graphql")
}

resource "aws_appsync_api_key" "assetario" {
  api_id  = aws_appsync_graphql_api.assetario.id
  expires = timeadd(timestamp(), "8736h")
}


