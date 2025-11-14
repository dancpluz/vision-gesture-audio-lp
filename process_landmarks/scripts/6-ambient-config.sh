#!/bin/bash

# Este script configura todo o ambiente para o projeto mediapipe-rs do zero.
# Ele instala dependências, clona o projeto, corrige os arquivos
# e instala a versão exata do WasmEdge que sabemos que funciona.
# Execute este script UMA VEZ.

# 'set -ex' faz o script parar se algum comando falhar (e)
# e imprime cada comando antes de executá-lo (x).
set -ex

# --- Variáveis de Configuração ---
# O nome da pasta para clonar o projeto
FLATC_VERSION="v22.10.26"
CARGO_ROOT="$(dirname -- "$0")/../"
# -------------------------------

echo "--- 1. Instalando Dependências do Sistema (curl, git, unzip) ---"
sudo apt-get update
sudo apt-get install -y curl git unzip wget

echo "--- 2. Instalando o Compilador Flatbuffers (flatc $FLATC_VERSION) ---"

# Instala a versão exata v22.10.26 manualmente para garantir a compatibilidade
if ! command -v flatc &> /dev/null || ! flatc --version | grep -q "22.10.26"; then
    echo "Instalando flatc $FLATC_VERSION..."
    wget "https://github.com/google/flatbuffers/releases/download/$FLATC_VERSION/Linux.flatc.binary.g++-10.zip" -O /tmp/flatc.zip
    unzip /tmp/flatc.zip -d /tmp/flatbuffers-v22
    sudo mv /tmp/flatbuffers-v22/flatc /usr/local/bin/flatc
    sudo chmod +x /usr/local/bin/flatc
    rm /tmp/flatc.zip
    rm -rf /tmp/flatbuffers-v22
    echo "flatc $FLATC_VERSION instalado com sucesso."
else
    echo "flatc $FLATC_VERSION já está instalado."
fi
# Verifica a instalação
flatc --version

echo "--- 3. Instalando o WasmEdge ---"
# Instala a v0.13.1 com o plugin tflite e a dist manylinux2014
# para evitar todos os erros 404.
if [ -d "$HOME/.wasmedge" ]; then
    echo "WasmEdge já parece estar instalado. Pulando."
else
    curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.13.1 --plugins wasi_nn-tensorflowlite --dist manylinux2014
fi

echo "--- 4. Corrigindo o 'Cargo.toml' para o flatbuffers v22 ---"

cd "$CARGO_ROOT"
# Usa o 'sed' para encontrar e substituir a linha do flatbuffers.
# Isso corrige o conflito entre o flatc v22 e o flatbuffers v23.
sed -i 's/^flatbuffers = .*/flatbuffers = "22.10.26"/' Cargo.toml

echo "Arquivo 'Cargo.toml' corrigido."

echo "✅ Ambiente configurado com sucesso!"
echo "IMPORTANTE:"
echo "Feche e reabra seu terminal agora, ou execute o comando abaixo:"
echo "  source \$HOME/.bashrc"

