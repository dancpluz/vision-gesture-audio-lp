#!/bin/bash
# Configurar câmera virtual no WSL

set -e

echo "=== Configurando Câmera Virtual ==="

# Instalar v4l2loopback
echo "Instalando v4l2loopback..."
sudo apt-get update
sudo apt-get install -y v4l2loopback-dkms v4l2loopback-utils

# Carregar módulo
echo "Configurando módulo..."
sudo modprobe v4l2loopback devices=1 video_nr=10 card_label="WSL Virtual Camera"

echo ""
echo "✅ Câmera virtual configurada em /dev/video10"
echo ""
echo "Para usar:"
echo "  1. Instale 'IP Webcam' no celular"
echo "  2. Execute:"
echo "     ffmpeg -i http://[IP_CELULAR]:8080/video -f v4l2 /dev/video10"