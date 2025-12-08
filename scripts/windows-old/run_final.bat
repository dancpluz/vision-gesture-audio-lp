@echo off
echo ========================================
echo    Detector de Mãos - Execução Final
echo ========================================

REM Limpar builds anteriores com problemas
echo Limpando cache de compilação anterior...
if exist "target" rmdir /s /q target
if exist "C:\Users\davi1\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\opencv-0.97.2\target" rmdir /s /q "C:\Users\davi1\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\opencv-0.97.2\target"

REM Configurar ambiente completo
set PATH=C:\tools\opencv\build\x64\vc16\bin;C:\Users\davi1\.cargo\bin;%PATH%
set OpenCV_DIR=C:\tools\opencv\build
set OPENCV_LINK_LIBS=opencv_world4110
set OPENCV_LINK_PATHS=C:\tools\opencv\build\x64\vc16\lib
set OPENCV_INCLUDE_PATHS=C:\tools\opencv\build\include
set OPENCV_DISABLE_PROBES=pkg_config
set PKG_CONFIG_PATH=
set VCPKG_ROOT=C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg

REM Configurar variáveis do Rust/Cargo
set CARGO_TARGET_DIR=%CD%\target
set RUST_BACKTRACE=1

echo.
echo Configuração do ambiente:
echo - OpenCV_DIR: %OpenCV_DIR%
echo - Bibliotecas: %OPENCV_LINK_PATHS%
echo - Headers: %OPENCV_INCLUDE_PATHS%
echo.

echo Verificando arquivos OpenCV...
if exist "C:\tools\opencv\build\x64\vc16\lib\opencv_world4110.lib" (
    echo [OK] Biblioteca OpenCV encontrada
) else (
    echo [ERRO] Biblioteca OpenCV não encontrada!
    echo Verifique se o OpenCV está instalado em C:\tools\opencv
    pause
    exit /b 1
)

if exist "C:\tools\opencv\build\include\opencv2\core.hpp" (
    echo [OK] Headers OpenCV encontrados
) else (
    echo [ERRO] Headers OpenCV não encontrados!
    pause
    exit /b 1
)

echo.
echo Iniciando compilação e execução...
echo Isso pode levar alguns minutos na primeira execução.
echo.

REM Executar com tratamento de erros
cargo run --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ========================================
    echo ERRO na compilação!
    echo ========================================
    echo.
    echo Soluções possíveis:
    echo 1. Aguarde a instalação do vcpkg terminar:
    echo    cd vcpkg && vcpkg install opencv4:x64-windows
    echo.
    echo 2. Use o programa de demonstração:
    echo    rustc src/simple_test.rs && simple_test.exe
    echo.
    pause
) else (
    echo.
    echo ========================================
    echo Programa executado com sucesso!
    echo ========================================
)

pause