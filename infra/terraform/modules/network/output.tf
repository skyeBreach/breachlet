# ==================================================================================================================== #
# Identification

output "vpc_id" {
  description = "ID of the VPC"
  value       = aws_vpc.main.id
}

output "cidr_block" {
  description = "CIDR block of the VPC"
  value       = aws_vpc.main.cidr_block
}

# ==================================================================================================================== #
# Subnets

output "public_subnet_ids" {
  description = "Identifiers for the public subnets"
  value       = aws_subnet.public[*].id
}

output "private_subnet_ids" {
  description = "Identifiers for the private subnets"
  value       = aws_subnet.private[*].id

}

# ==================================================================================================================== #
