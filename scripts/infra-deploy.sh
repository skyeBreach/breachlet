#!/usr/bin/env bash
# ==================================================================================================================== #
# scripts/infra-deploy.sh - Infrastructure Deployment
#
# ==================================================================================================================== #
# Defaults

set -euo pipefail

source "$(realpath "$(dirname "$0")")/common.sh"

REGION="eu-west-2"
ENVIRONMENT="dev"
TF_DIR=infra/terraform

# ==================================================================================================================== #
# Helper Functions

_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --region <REGION>                       AWS Region (default: eu-west-2)"
    echo "  --environment <ENV: dev, staging, prod> Deployment Environment (default: dev)"
}

# ==================================================================================================================== #
# Arguments

_setArgs() {
    while [ "${1:-}" != '' ]; do
        case "$1" in
        '--region')
            _REGION="$2"
            shift 2
            ;;
        '--environment')
            _ENVIRONMENT="$2"
            shift 2
            ;;
        '-h' | '--help')
            _usage
            ;;
        *)
            echo "Unknown argument: $1"
            usage
            ;;
        esac
        shift
    done
}

_setArgs "$*"

# ==================================================================================================================== #
# Tf Init

print_status "Initialising OpenTofu in ${TF_DIR}..."
tofu -chdir="${TF_DIR}" init

# ==================================================================================================================== #
# TF Apply

print_status "Applying OpenTofu Configuration in ${TF_DIR}..."
tofu -chdir="${TF_DIR}" apply \
    -auto-approve \
    -var="region=${REGION}" \
    -var="environment=${ENVIRONMENT}"

# ==================================================================================================================== #
# Success

echo ""
print_success "Backend setup completed!"

# ==================================================================================================================== #
