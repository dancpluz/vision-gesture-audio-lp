#!/bin/bash

# Uso:
#   ./run-example.sh <nome-do-exemplo> [argumentos-do-exemplo...]
#
# Exemplo para image_classification:
#   ./run-example.sh image_classification ./assets/models/image_classification/efficientnet_lite0_fp32.tflite ./assets/testdata/img/burger.jpg
#
# Exemplo para gesture_recognition:
#   ./run-example.sh gesture_recognition ./assets/models/gesture_recognition/gesture_recognizer.task ./assets/testdata/img/gesture_recognition_google_samples/victory.jpg
#

set -ex

# Pega o primeiro argumento como o nome do exemplo (ex: "image_classification")
EXAMPLE_NAME=$1

# Verifica se o nome do exemplo foi fornecido
if [ -z "$EXAMPLE_NAME" ]; then
    echo "Erro: Nenhum nome de exemplo foi fornecido."
    echo "Uso: ./run-example.sh <nome-do-exemplo> [argumentos...]"
    exit 1
fi

# Remove o primeiro argumento (o nome do exemplo) da lista.
# O que sobrar em "$@" serão os argumentos para o programa wasm.
shift
EXAMPLE_ARGS="$@"

echo "--- 1. Ativando o Ambiente WasmEdge (v0.13.1) ---"
# Precisamos ter certeza de que o WasmEdge está no PATH
source "$HOME/.wasmedge/env"

echo "--- 2. Compilando o Exemplo: $EXAMPLE_NAME (target wasm32-wasip1) ---"
cargo build --release --example "$EXAMPLE_NAME" --target=wasm32-wasip1

echo "--- 3. Executando o Exemplo: $EXAMPLE_NAME ---"
# Constrói o caminho para o arquivo .wasm
WASM_FILE="target/wasm32-wasip1/release/examples/$EXAMPLE_NAME.wasm"

# Executa o comando
wasmedge --dir .:. "$WASM_FILE" $EXAMPLE_ARGS

echo "✅ Execução concluída."

