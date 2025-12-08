#!/bin/bash
# Script para configurar ambiente OpenCV com vcpkg

# Definir variáveis de ambiente
export OPENCV_LINK_LIBS="opencv_world4110"
export OPENCV_LINK_PATHS="C:/Users/davi1/Documents/codigos/computervisionLP/vcpkg/installed/x64-windows/lib"
export OPENCV_INCLUDE_PATHS="C:/Users/davi1/Documents/codigos/computervisionLP/vcpkg/installed/x64-windows/include"
export OpenCV_DIR="C:/Users/davi1/Documents/codigos/computervisionLP/vcpkg/installed/x64-windows/share/opencv4"

# Adicionar DLLs ao PATH
export PATH="C:/Users/davi1/Documents/codigos/computervisionLP/vcpkg/installed/x64-windows/bin:$PATH"

echo "Configurando OpenCV com vcpkg..."
echo "OpenCV_DIR: $OpenCV_DIR"
echo "PATH atualizado com DLLs do OpenCV"

# Compilar o projeto
cargo build --release