@echo off
echo Setting up OpenCV environment...

REM Set OpenCV paths
set OpenCV_DIR=C:\tools\opencv\build\x64\vc16
set OPENCV_INCLUDE_PATHS=C:\tools\opencv\build\include
set OPENCV_LINK_PATHS=C:\tools\opencv\build\x64\vc16\lib
set OPENCV_LINK_LIBS=opencv_world4110

REM Add OpenCV DLLs to PATH
set PATH=C:\tools\opencv\build\x64\vc16\bin;%PATH%

REM Try to find and run vcvarsall.bat for Visual Studio tools
if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" (
    call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
) else if exist "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" (
    call "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
) else (
    echo Visual Studio Build Tools not found. Please install them first.
    pause
    exit /b 1
)

echo Building Rust project...
cargo build

pause