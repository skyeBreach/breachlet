# ==================================================================================================================== #
# Environment Configuration

variable "name_prefix" {
  description = "Name of the project"
  type        = string
}

# ==================================================================================================================== #
# Network

variable "vpc_id" {
  description = "Identifier for the VPC to attach the security groups."
  type        = string
}

variable "bastion_ssh_cidr" {
  description = "CIDR blocks allowed SSH access to the bastion"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

# ==================================================================================================================== #

