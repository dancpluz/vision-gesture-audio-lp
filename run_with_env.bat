@echo off
echo === Configurando ambiente para OpenCV ===

REM Limpar variáveis problemáticas
set OPENCV_DISABLE_PROBES=
set PKG_CONFIG_PATH=

REM Configurar ambiente OpenCV
set OpenCV_DIR=C:\tools\opencv\build
set OPENCV_LINK_LIBS=opencv_world4110
set OPENCV_LINK_PATHS=C:\tools\opencv\build\x64\vc16\lib
set OPENCV_INCLUDE_PATHS=C:\tools\opencv\build\include
set PATH=C:\tools\opencv\build\x64\vc16\bin;C:\Users\davi1\.cargo\bin;%PATH%

echo Configuracao:
echo OpenCV_DIR=%OpenCV_DIR%
echo OPENCV_LINK_LIBS=%OPENCV_LINK_LIBS%
echo OPENCV_LINK_PATHS=%OPENCV_LINK_PATHS%
echo OPENCV_INCLUDE_PATHS=%OPENCV_INCLUDE_PATHS%
echo.

echo === Verificando arquivos ===
if exist "C:\tools\opencv\build\x64\vc16\lib\opencv_world4110.lib" (
    echo ✓ Biblioteca OpenCV encontrada
) else (
    echo ✗ Biblioteca nao encontrada em C:\tools\opencv\build\x64\vc16\lib\
)

if exist "C:\tools\opencv\build\include\opencv2\core.hpp" (
    echo ✓ Headers OpenCV encontrados
) else (
    echo ✗ Headers nao encontrados
)
echo.

echo === Executando programa ===
cargo run --release

pause