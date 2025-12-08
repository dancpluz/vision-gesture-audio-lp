# PowerShell script para configurar OpenCV com vcpkg
Write-Host "Configurando OpenCV com vcpkg..." -ForegroundColor Green

# Configurar variáveis de ambiente
$env:OPENCV_LINK_LIBS = "opencv_world4110"
$env:OPENCV_LINK_PATHS = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\lib"
$env:OPENCV_INCLUDE_PATHS = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\include"
$env:OpenCV_DIR = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\share\opencv4"

# Adicionar DLLs ao PATH
$env:PATH = "C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg\installed\x64-windows\bin;" + $env:PATH

Write-Host "OpenCV_DIR: $env:OpenCV_DIR"
Write-Host "PATH atualizado com DLLs do OpenCV"

# Compilar o projeto
Write-Host "Compilando projeto..." -ForegroundColor Yellow
cargo build --release