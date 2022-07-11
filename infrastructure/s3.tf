
resource "aws_s3_bucket" "assetario" {
  bucket = "selma-solutions-assetario"
  force_destroy = true
}

resource "aws_s3_bucket_acl" "storage" {
  bucket = aws_s3_bucket.assetario.id
  acl    = "public-read"
}

resource "aws_s3_object" "simplemaps" {
  bucket = aws_s3_bucket.assetario.id
  key    = "demo/assetario/simplemaps.zip"
  source = "../resources/simplemaps.zip"
  acl    = "public-read"

  etag = filemd5("../resources/simplemaps.zip")
}

resource "aws_s3_object" "simplemaps-small" {
  bucket = aws_s3_bucket.assetario.id
  key    = "demo/assetario/simplemaps_small.zip"
  source = "../resources/simplemaps_small.zip"
  acl    = "public-read"

  etag = filemd5("../resources/simplemaps_small.zip")
}