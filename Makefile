.PHONY: help setup run install clean test build virtual-camera

help:
	@echo "Comandos disponíveis:"
	@echo "  make setup         - Configurar ambiente (instalar dependências)"
	@echo "  make install       - Instalação completa"
	@echo "  make build         - Compilar projeto"
	@echo "  make run           - Executar programa"
	@echo "  make test          - Executar testes"
	@echo "  make clean         - Limpar builds"
	@echo "  make virtual-camera - Configurar câmera virtual (WSL)"
	@echo ""
	@echo "Para WSL: make setup && make run"
	@echo "Para Windows: use scripts/windows/"

setup:
	@chmod +x scripts/wsl/setup.sh
	@./scripts/wsl/setup.sh

install:
	@chmod +x scripts/wsl/install.sh
	@./scripts/wsl/install.sh

build:
	cargo build --release

run:
	@chmod +x scripts/wsl/run.sh
	@./scripts/wsl/run.sh

test:
	cargo test

clean:
	cargo clean
	rm -rf target/

virtual-camera:
	@chmod +x scripts/wsl/virtual_camera.sh
	@./scripts/wsl/virtual_camera.sh

# Windows targets (se necessário)
windows-setup:
	@echo "Para Windows, execute os scripts em scripts/windows/"
	@echo "ou use PowerShell: ./scripts/windows/setup.ps1"

.DEFAULT_GOAL := help