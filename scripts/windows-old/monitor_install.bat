@echo off
setlocal enabledelayedexpansion

:: Configurações
set VCPKG_ROOT=C:\Users\davi1\Documents\codigos\computervisionLP\vcpkg
set TOTAL_PACKAGES=15
set LOG_FILE=%TEMP%\vcpkg_install.log

:: Iniciar monitoramento
echo ===============================================
echo    MONITORADOR DE INSTALAÇÃO vcpkg/OpenCV
echo ===============================================
echo.
echo Data/Hora Inicio: %date% %time%
echo.

:: Iniciar vcpkg em background se não estiver rodando
tasklist /FI "IMAGENAME eq vcpkg.exe" 2>NUL | find /I "vcpkg.exe" >NUL
if %ERRORLEVEL% NEQ 0 (
    echo Iniciando instalacao do vcpkg...
    start "vcpkg install" /MIN cmd /c "cd /d %VCPKG_ROOT% && vcpkg install opencv4:x64-windows > %LOG_FILE% 2>&1"
    timeout /t 3 >nul
)

:: Loop de monitoramento
:monitor
cls
echo ===============================================
echo    MONITORADOR DE INSTALAÇÃO vcpkg/OpenCV
echo ===============================================
echo.
echo Progresso da Instalacao:
echo.

:: Verificar se o vcpkg ainda está rodando
tasklist /FI "IMAGENAME eq vcpkg.exe" 2>NUL | find /I "vcpkg.exe" >NUL
if %ERRORLEVEL% NEQ 0 (
    echo [CONCLUIDO] A instalacao foi finalizada!
    echo.
    goto :check_completion
)

:: Contar pacotes instalados
set INSTALLED=0
if exist "%VCPKG_ROOT%\installed\x64-windows\lib\opencv_world4110.lib" (
    set /a INSTALLED=%TOTAL_PACKAGES%
) else (
    :: Verificar pacotes individuais
    for %%f in (
        "%VCPKG_ROOT%\installed\x64-windows\lib\zlib.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\libpng16.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\libjpeg-turbo.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\libwebp.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\protobuf.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\tiff.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\lzma.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\abseil_dll.lib"
        "%VCPKG_ROOT%\installed\x64-windows\lib\flatbuffers.lib"
    ) do (
        if exist "%%f" set /a INSTALLED+=1
    )
)

:: Calcular percentual
set /a PERCENTAGE=%INSTALLED%*100/%TOTAL_PACKAGES%

:: Desenhar barra de progresso
set /a FILLED=%PERCENTAGE%/2
set /a EMPTY=50-%FILLED%
set BAR=
for /L %%i in (1,1,%FILLED%) do set BAR=!BAR!█
for /L %%i in (1,1,%EMPTY%) do set BAR=!BAR!░

:: Exibir progresso
echo   [!BAR!] %PERCENTAGE%%%
echo.
echo Pacotes instalados: %INSTALLED%/%TOTAL_PACKAGES%
echo.

:: Mostrar tempo decorrido
if defined START_TIME (
    call :GetDuration "!START_TIME!" "!time!" DURATION
    echo Tempo decorrido: !DURATION!
) else (
    set START_TIME=%time%
    echo Iniciando contagem...
)

echo.
echo Status: Compilando pacotes...
echo.
echo Pacote atual:
findstr /I "Installing\|Building" %LOG_FILE% 2>NUL | tail -1 2>NUL
echo.
echo Atualizado em: %time%

:: Atualizar a cada 5 segundos
timeout /t 5 >nul
goto :monitor

:check_completion
echo ===============================================
echo Verificando instalacao concluida...
echo ===============================================
echo.

if exist "%VCPKG_ROOT%\installed\x64-windows\lib\opencv_world4110.lib" (
    echo [SUCESSO] OpenCV instalado com sucesso!
    echo.
    echo Arquivos encontrados:
    dir "%VCPKG_ROOT%\installed\x64-windows\lib\opencv_world*.lib" /b
    echo.
    echo Para executar o programa:
    echo   run_final.bat
) else (
    echo [ERRO] OpenCV nao foi instalado corretamente.
    echo Verifique o log em: %LOG_FILE%
)

echo.
pause
goto :eof

:GetDuration
setlocal
set "start=%~1"
set "end=%~2"

:: Converter tempo para segundos
set /a start_h=1%start:~0,2%-100
set /a start_m=1%start:~3,2%-100
set /a start_s=1%start:~6,2%-100
set /a start_total=%start_h%*3600+%start_m%*60+%start_s%

set /a end_h=1%end:~0,2%-100
set /a end_m=1%end:~3,2%-100
set /a end_s=1%end:~6,2%-100
set /a end_total=%end_h%*3600+%end_m%*60+%end_s%

set /a duration=%end_total%-%start_total%
set /a hours=%duration%/3600
set /a minutes=(%duration%%%3600)/60
set /a seconds=%duration%%%60

if %hours% GTR 0 (
    set "result=%hours%h %minutes%m %seconds%s"
) else if %minutes% GTR 0 (
    set "result=%minutes%m %seconds%s"
) else (
    set "result=%seconds%s"
)
endlocal & set "%~2=%result%"
goto :eof