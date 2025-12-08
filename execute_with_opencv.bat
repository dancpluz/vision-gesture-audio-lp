@echo off
echo === Configurando ambiente OpenCV ===
set OpenCV_DIR=C:\tools\opencv\build
set PATH=C:\tools\opencv\build\x64\vc16\bin;%PATH%
set PATH=C:\Users\davi1\.cargo\bin;%PATH%
set OPENCV_LINK_PATHS=C:\tools\opencv\build\x64\vc16\lib
set OPENCV_INCLUDE_PATHS=C:\tools\opencv\build\include

echo OpenCV_DIR=%OpenCV_DIR%
echo PATH configurado com DLLs OpenCV
echo.
echo === Compilando e executando programa ===

cargo run --release

pause