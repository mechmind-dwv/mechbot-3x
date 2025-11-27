#!/bin/bash
# Script de deployment completo para MechBot-3x

set -e

echo "🚀 MechBot-3x Deployment Script"
echo "==============================="

# Variables
ENVIRONMENT=${1:-development}
IMAGE_TAG=${2:-latest}

case $ENVIRONMENT in
    development)
        echo "🔧 Deploying to Development..."
        docker-compose up -d
        ;;
    
    production)
        echo "🏭 Deploying to Production..."
        ./scripts/docker-build.sh $IMAGE_TAG mechmind-dwv --push
        ./scripts/k8s-deploy.sh production $IMAGE_TAG
        ;;
    
    staging)
        echo "🧪 Deploying to Staging..."
        ./scripts/docker-build.sh $IMAGE_TAG mechmind-dwv
        ./scripts/k8s-deploy.sh staging $IMAGE_TAG
        ;;
    
    *)
        echo "❌ Unknown environment: $ENVIRONMENT"
        echo "Usage: $0 [development|staging|production] [image-tag]"
        exit 1
        ;;
esac

echo "✅ Deployment to $ENVIRONMENT completed successfully!"
