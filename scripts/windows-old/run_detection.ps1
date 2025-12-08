# PowerShell script para executar detecção de mãos
Write-Host "=== Configurando ambiente OpenCV ===" -ForegroundColor Green

# Configurar variáveis de ambiente
$env:OpenCV_DIR = "C:\tools\opencv\build"
$env:PATH = "C:\tools\opencv\build\x64\vc16\bin;C:\Users\davi1\.cargo\bin;" + $env:PATH
$env:OPENCV_LINK_PATHS = "C:\tools\opencv\build\x64\vc16\lib"
$env:OPENCV_INCLUDE_PATHS = "C:\tools\opencv\build\include"
$env:OPENCV_LINK_LIBS = "opencv_world4110"

Write-Host "✓ OpenCV_DIR: $env:OpenCV_DIR" -ForegroundColor Green
Write-Host "✓ PATH configurado com DLLs OpenCV" -ForegroundColor Green
Write-Host ""

Write-Host "=== Compilando e executando programa ===" -ForegroundColor Yellow

# Limpar builds anteriores para forçar recompilação
Write-Host "Limpando builds anteriores..." -ForegroundColor Gray
Remove-Item -Recurse -Force target -ErrorAction SilentlyContinue

# Compilar e executar
cargo run --release

Write-Host ""
Write-Host "Programa finalizado." -ForegroundColor Cyan
Read-Host "Pressione Enter para fechar"