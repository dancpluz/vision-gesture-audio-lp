@echo off
echo ============================================================
echo     STATUS DETALHADO DA INSTALAÇÃO vcpkg/OpenCV
echo ============================================================
echo.
echo Hora: %date% %time%
echo.

:: Verificar se vcpkg está rodando
echo [1/5] Verificando processo vcpkg...
tasklist /FI "IMAGENAME eq vcpkg.exe" 2>NUL | find /I "vcpkg.exe" >NUL
if %ERRORLEVEL% EQU 0 (
    echo ✓ vcpkg está rodando
) else (
    echo ✗ vcpkg não está rodando
    echo.
    echo Para iniciar a instalação manualmente:
    echo   cd vcpkg
    echo   vcpkg install opencv4:x64-windows
    echo.
    pause
    exit /b 1
)

echo.
echo [2/5] Verificando dependências já instaladas...

:: Contar bibliotecas instaladas
set LIB_COUNT=0
if exist "vcpkg\installed\x64-windows\lib\zlib.lib" (
    echo   ✓ zlib
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\libpng16.lib" (
    echo   ✓ libpng
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\libjpeg-turbo.lib" (
    echo   ✓ libjpeg-turbo
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\libwebp.lib" (
    echo   ✓ libwebp
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\protobuf.lib" (
    echo   ✓ protobuf
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\tiff.lib" (
    echo   ✓ tiff
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\lzma.lib" (
    echo   ✓ lzma
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\abseil_dll.lib" (
    echo   ✓ abseil
    set /a LIB_COUNT+=1
)
if exist "vcpkg\installed\x64-windows\lib\flatbuffers.lib" (
    echo   ✓ flatbuffers
    set /a LIB_COUNT+=1
)

echo.
echo [3/5] Verificando OpenCV...
if exist "vcpkg\installed\x64-windows\lib\opencv_world4110.lib" (
    echo   ✓ OpenCV4 - INSTALADO COM SUCESSO!
    set /a LIB_COUNT=15
) else (
    echo   ⏳ OpenCV4 - aguardando compilação...
)

echo.
echo [4/5] Resumo do Progresso:
echo   • Bibliotecas instaladas: %LIB_COUNT%/15
echo   • Progresso estimado: %%LIB_COUNT%%%%%%
echo.

:: Calcular tempo estimado
set /a REMAINING=15-%LIB_COUNT%
set /a MINUTES_ESTIMATE=%REMAINING%*2
echo   • Tempo restante estimado: ~%MINUTES_ESTIMATE% minutos

echo.
echo [5/5] Próximos passos:
echo   1. Aguardar instalação terminar
echo   2. Executar: run_final.bat
echo.

echo ============================================================
echo Pressione F5 para atualizar o status
echo ============================================================
pause