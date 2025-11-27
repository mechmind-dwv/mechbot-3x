#!/bin/bash
# Script de deployment Kubernetes para MechBot-3x

set -e

echo "☸️ Deploying MechBot-3x to Kubernetes..."

NAMESPACE="mechbot"
CONTEXT="${1:-default}"
IMAGE_TAG="${2:-latest}"

# Verificar kubectl
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl no encontrado. Por favor instala kubectl."
    exit 1
fi

# Configurar contexto
kubectl config use-context $CONTEXT

# Crear namespace
echo "📁 Creating namespace..."
kubectl apply -f k8s/namespace.yaml

# Esperar a que el namespace esté listo
kubectl wait --for=jsonpath='{.status.phase}'=Active namespace/$NAMESPACE --timeout=30s

# Actualizar imagen en deployment
echo "🔄 Updating deployment image..."
kubectl set image deployment/mechbot-3x mechbot=mechmind-dwv/mechbot-3x:$IMAGE_TAG -n $NAMESPACE --record

# Si no existe el deployment, crearlo
if ! kubectl get deployment mechbot-3x -n $NAMESPACE &> /dev/null; then
    echo "🚀 Creating new deployment..."
    kubectl apply -f k8s/deployment.yaml
fi

# Esperar a que el deployment esté listo
echo "⏳ Waiting for deployment to be ready..."
kubectl rollout status deployment/mechbot-3x -n $NAMESPACE --timeout=300s

# Mostrar información del deployment
echo "📊 Deployment status:"
kubectl get pods -n $NAMESPACE -l app=mechbot

echo "✅ MechBot-3x deployed successfully to Kubernetes!"
echo "🌐 API available at: http://<node-ip>:30080"
echo "🔌 WebSocket available at: ws://<node-ip>:30081"
