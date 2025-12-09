#!/bin/bash
# Executar programa no WSL

set -e

echo "=== Executando Detector de Mãos ==="

# Encontrar diretório do script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$( dirname "$SCRIPT_DIR" )" )"
cd "$PROJECT_ROOT"

# Carregar configurações
if [ -f .env.wsl ]; then
    source .env.wsl
fi

# Verificar se OpenCV está instalado
if ! pkg-config --exists opencv4 2>/dev/null; then
    echo "OpenCV não encontrado. Executando setup..."
    "$SCRIPT_DIR/setup.sh"
fi

# Verificar webcam
echo "Verificando webcam..."
if ls /dev/video* 2>/dev/null; then
    echo "✅ Webcam detectada"
else
    echo "⚠️  Webcam não detectada"
    echo "   No WSL, você pode:"
    echo "   1. Usar IP Webcam do celular"
    echo "   2. make virtual-camera para configurar câmera virtual"
    echo "   3. Usar arquivo de vídeo de teste"
fi

# Compilar se necessário
if [ ! -f "target/release/aruco-audio-lp" ]; then
    echo "Compilando projeto..."
    cargo build --release
fi

echo ""
echo "🎬 Executando programa..."
echo "Controles:"
echo "  - S: Salvar frame"
echo "  - ESC: Sair"
echo ""

# Executar programa
./target/release/aruco-audio-lp