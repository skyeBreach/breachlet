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
  name_prefix = local.name_prefix
  cidr_block  = "10.0.0.0/16"
}

# ==================================================================================================================== #
# Security

module "security" {
  source = "./modules/security"

  # Variables
  name_prefix = local.name_prefix
  vpc_id      = module.network.vpc_id
}

# ==================================================================================================================== #

