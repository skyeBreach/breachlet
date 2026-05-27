# ==================================================================================================================== #
# 
# 
#
# ==================================================================================================================== #
# 

variable "name_prefix" {
  description = "Prefix for all resource names"
  type        = string
}

variable "environment" {
  description = "Deployment environment name, used as a resource prefix"
  type        = string
  default     = "staging"
}

# ==================================================================================================================== #
