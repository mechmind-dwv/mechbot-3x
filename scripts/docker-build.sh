#!/bin/bash
# Script de construcción y deployment Docker para MechBot-3x

set -e

echo "🐳 Building MechBot-3x Docker image..."

# Variables
IMAGE_NAME="mechbot-3x"
TAG="${1:-latest}"
REGISTRY="${2:-mechmind-dwv}"

# Build de la imagen
docker build -t $REGISTRY/$IMAGE_NAME:$TAG .

# Test de la imagen
echo "🧪 Testing Docker image..."
docker run --rm $REGISTRY/$IMAGE_NAME:$TAG --version

# Push opcional a registry
if [ "$3" = "--push" ]; then
    echo "📤 Pushing to registry..."
    docker push $REGISTRY/$IMAGE_NAME:$TAG
fi

echo "✅ Docker image built successfully: $REGISTRY/$IMAGE_NAME:$TAG"
