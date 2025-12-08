# PowerShell script para executar detecção de mãos
Write-Host "=== Detecção de Mãos em Tempo Real ===" -ForegroundColor Green

# Configurar variáveis de ambiente para vcpkg
$env:VCPKG_ROOT = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg"

# Adicionar DLLs ao PATH
$vcpkg_bin = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\bin"
if (Test-Path $vcpkg_bin) {
    $env:PATH = "$vcpkg_bin;$env:PATH"
    Write-Host "✓ PATH configurado com DLLs vcpkg" -ForegroundColor Green
}

Write-Host "Verificando instalação do OpenCV..." -ForegroundColor Yellow

# Verificar se OpenCV foi instalado
$opencv_lib = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\lib"
if (Test-Path "$opencv_lib\opencv_world4110.lib") {
    Write-Host "✓ OpenCV encontrado via vcpkg" -ForegroundColor Green
} else {
    Write-Host "⚠ OpenCV ainda não foi instalado via vcpkg" -ForegroundColor Yellow
    Write-Host "Aguardando instalação... Execute ./vcpkg/vcpkg install opencv4:x64-windows" -ForegroundColor Yellow
    exit 1
}

# Configurar ambiente OpenCV
$env:OpenCV_DIR = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\share\opencv4"
$env:OPENCV_LINK_PATHS = $opencv_lib
$env:OPENCV_INCLUDE_PATHS = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\include"

Write-Host "Compilando projeto..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Compilação bem-sucedida!" -ForegroundColor Green
    Write-Host "Executando detecção de mãos..." -ForegroundColor Green
    Write-Host ""
    Write-Host "Controles:" -ForegroundColor Cyan
    Write-Host "  - ESPAÇO: Calibrar fundo" -ForegroundColor Cyan
    Write-Host "  - S: Salvar frame atual" -ForegroundColor Cyan
    Write-Host "  - ESC: Sair" -ForegroundColor Cyan
    Write-Host ""

    cargo run --release
} else {
    Write-Host "✗ Falha na compilação" -ForegroundColor Red
    exit 1
}