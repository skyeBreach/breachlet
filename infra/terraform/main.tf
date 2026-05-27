# ==================================================================================================================== #
# Locals

locals {
  name_prefix = "${var.project_name}-${var.environment}"
  common_tags = merge(var.tags, {
    Environment = var.environment
    Project     = var.project_name
    ManagedBy   = "OpenTofu"
    Owner       = "skyeBreach"
  })
}

# ==================================================================================================================== #
# Network

module "network" {
  source = "./modules/network"

  # Variables
  cidr_block = "10.0.0.0/16"
}

# ==================================================================================================================== #

