#!/bin/bash

# Este script compila e executa o exemplo 'image_classification'.
# Execute-o sempre que quiser rodar o projeto.

set -ex

echo "--- 1. Ativando o Ambiente WasmEdge (v0.13.1) ---"
# Precisamos ter certeza de que o WasmEdge está no PATH
source "$HOME/.wasmedge/env"

echo "--- 2. Compilando o Exemplo (target wasm32-wasip1) ---"
cargo build --release --example image_classification --target=wasm32-wasip1

echo "--- 3. Executando o Exemplo de Classificação de Imagem ---"
wasmedge --dir .:. target/wasm32-wasip1/release/examples/image_classification.wasm ./assets/models/image_classification/efficientnet_lite0_fp32.tflite ./assets/testdata/img/burger.jpg

echo "✅ Execução concluída."
