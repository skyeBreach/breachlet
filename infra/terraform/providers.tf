terraform {
  required_version = ">= 1.12"
  required_providers {
    aws = {
      source  = "opentofu/aws"
      version = "6.46.0"
    }
  }
}

provider "aws" {
  # Configuration options

}
