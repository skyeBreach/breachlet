# ==================================================================================================================== #
# Environment Configuration

variable "project_name" {
  description = "Name of the project"
  type        = string
  default     = "breachlet"
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "dev"
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be on of the following: dev, staging, prod."
  }
}

variable "region" {
  description = "Primary region for resources"
  type        = string
  default     = "eu-west-2"
}

# ==================================================================================================================== #
# Access

variable "aws_access_key" {
  description = "AWS access key"
  type        = string
  default     = ""
  sensitive   = true
}

variable "aws_secret_key" {
  description = "AWS secret key"
  type        = string
  default     = ""
  sensitive   = true
}

# ==================================================================================================================== #
# Security

variable "bastion_ssh_cidr" {
  description = "CIDR blocks allowed SSH access to the bastion"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

# ==================================================================================================================== #
# Metaconfig

variable "tags" {
  description = "Tags to apply to all resources"
  type        = map(string)
  default = {
    Environment = "dev"
    Project     = "breachlet"
    ManagedBy   = "opentofu"
    Owner       = "skyeBreach"
  }
}

# ==================================================================================================================== #
