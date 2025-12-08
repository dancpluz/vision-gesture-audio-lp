# PowerShell script para build do projeto OpenCV em Rust
# Execute como Administrador!

Write-Host "Configurando ambiente OpenCV..." -ForegroundColor Green

# Configurar variáveis de ambiente
$env:OpenCV_DIR = "C:\tools\opencv\build\x64\vc16"
$env:OPENCV_INCLUDE_PATHS = "C:\tools\opencv\build\include"
$env:OPENCV_LINK_PATHS = "C:\tools\opencv\build\x64\vc16\lib"
$env:OPENCV_LINK_LIBS = "opencv_world4110"

# Adicionar DLLs ao PATH
$env:PATH = "C:\tools\opencv\build\x64\vc16\bin;" + $env:PATH

Write-Host "OpenCV_DIR: $env:OpenCV_DIR"
Write-Host "PATH atualizado com DLLs do OpenCV"

# Encontrar e executar vcvarsall.bat
$vsPath = @(
    "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
)

$vcvarsall = $vsPath | Where-Object { Test-Path $_ } | Select-Object -First 1

if ($vcvarsall) {
    Write-Host "Configurando ambiente Visual Studio..." -ForegroundColor Yellow
    & cmd.exe /c "`"$vcvarsall`" x64 && set"

    Write-Host "Compilando projeto Rust..." -ForegroundColor Green
    cargo build
} else {
    Write-Host "ERRO: Visual Studio Build Tools não encontrado!" -ForegroundColor Red
    Write-Host "Por favor, instale o Visual Studio Build Tools 2022 com as ferramentas C++." -ForegroundColor Yellow
    exit 1
}

Write-Host "Build concluído!" -ForegroundColor Green