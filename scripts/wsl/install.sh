#!/bin/bash
# Instalação completa para WSL

set -e

echo "=== Instalação Completa para WSL ==="

# Encontrar diretório do script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$( dirname "$SCRIPT_DIR" )" )"
cd "$PROJECT_ROOT"

# Instalar Rust se não estiver instalado
if ! command -v cargo &> /dev/null; then
    echo "Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Executar setup
chmod +x "$SCRIPT_DIR/setup.sh"
"$SCRIPT_DIR/setup.sh"

# Compilar projeto
echo "Compilando projeto..."
cargo build --release

echo ""
echo "✅ Instalação completa concluída!"
echo ""
echo "Para executar: make run"