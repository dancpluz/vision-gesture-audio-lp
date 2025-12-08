@echo off
echo === Configurando ambiente vcpkg/OpenCV ===

REM Configurar vcpkg
set VCPKG_ROOT=C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg
set PATH=%VCPKG_ROOT%;C:\Users\davi1\.cargo\bin;%PATH%

REM Configurar OpenCV via vcpkg
set OpenCV_DIR=%VCPKG_ROOT%\installed\x64-windows\share\opencv4
set OPENCV_LINK_PATHS=%VCPKG_ROOT%\installed\x64-windows\lib
set OPENCV_INCLUDE_PATHS=%VCPKG_ROOT%\installed\x64-windows\include

REM Adicionar DLLs ao PATH
set PATH=%VCPKG_ROOT%\installed\x64-windows\bin;%PATH%

echo Configuracao:
echo VCPKG_ROOT=%VCPKG_ROOT%
echo OpenCV_DIR=%OpenCV_DIR%
echo OPENCV_LINK_PATHS=%OPENCV_LINK_PATHS%
echo OPENCV_INCLUDE_PATHS=%OPENCV_INCLUDE_PATHS%
echo.

echo === Verificando vcpkg ===
if exist "%VCPKG_ROOT%\vcpkg.exe" (
    echo ✓ vcpkg encontrado
) else (
    echo ✗ vcpkg nao encontrado
)

echo.
echo === Verificando OpenCV no vcpkg ===
if exist "%OPENCV_LINK_PATHS%\opencv_world4110.lib" (
    echo ✓ Biblioteca OpenCV vcpkg encontrada
) else (
    echo ✗ Biblioteca nao encontrada, ainda instalando...
    echo Execute: %VCPKG_ROOT%\vcpkg install opencv4:x64-windows
)

echo.
echo === Executando programa ===
cargo run --release

pause