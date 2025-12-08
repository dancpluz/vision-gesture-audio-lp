#!/bin/bash
# Configuração básica do ambiente WSL

set -e

echo "=== Configurando ambiente WSL ==="

# Encontrar diretório do script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$( dirname "$SCRIPT_DIR" )" )"
cd "$PROJECT_ROOT"

# Verificar se está no WSL
if ! grep -q "Microsoft" /proc/version 2>/dev/null; then
    echo "⚠️  Este script é otimizado para WSL"
    echo "   Continuando mesmo assim..."
fi

echo "Instalando dependências do sistema..."
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libopencv-dev \
    libgtk-3-dev \
    libavcodec-dev \
    libavformat-dev \
    libswscale-dev \
    libv4l-dev \
    libxvidcore-dev \
    libx264-dev \
    libjpeg-dev \
    libpng-dev \
    libtiff-dev \
    gfortran \
    libatlas-base-dev \
    libtbbmalloc2 \
    libtbb-dev

echo "Configurando variáveis de ambiente..."
cat > .env.wsl << EOF
# Configurações WSL para OpenCV
export OPENCV_DIR="/usr"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:\$PKG_CONFIG_PATH"
export LD_LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:\$LD_LIBRARY_PATH"
EOF

source .env.wsl

echo "✅ Configuração concluída!"
echo ""
echo "Próximos passos:"
echo "  1. make build  # Compilar projeto"
echo "  2. make run    # Executar programa"