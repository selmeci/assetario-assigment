
resource "aws_dynamodb_table" "simplemaps" {
  name         = "AssetarioSimpleMaps"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "PK"
  range_key    = "SK"

  attribute {
    name = "PK"
    type = "S"
  }

  attribute {
    name = "SK"
    type = "S"
  }

  attribute {
    name = "G1PK"
    type = "S"
  }

  attribute {
    name = "G1SK"
    type = "S"
  }

  attribute {
    name = "G2PK"
    type = "S"
  }

  attribute {
    name = "G2SK"
    type = "S"
  }

  global_secondary_index {
    name            = "GSI1"
    hash_key        = "G1PK"
    range_key       = "G1SK"
    projection_type = "ALL"
  }

  global_secondary_index {
    name            = "GSI2"
    hash_key        = "G2PK"
    range_key       = "G2SK"
    projection_type = "ALL"
  }

  tags = {
    Name        = "AssetarioSimpleMaps"
    Environment = "demo"
  }
}