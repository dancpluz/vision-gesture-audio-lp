# Script para instalar OpenCV
Write-Host "Instalando OpenCV..." -ForegroundColor Green

# Baixar OpenCV
$url = "https://github.com/opencv/opencv/releases/download/4.11.0/opencv-4.11.0-windows.exe"
$output = "$PSScriptRoot\opencv-installer.exe"

Write-Host "Baixando OpenCV de: $url"
Invoke-WebRequest -Uri $url -OutFile $output

# Extrair
Write-Host "Extraindo OpenCV para C:\opencv..."
Start-Process -FilePath $output -ArgumentList "/S", "/D=C:\opencv" -Wait

# Configurar variáveis de ambiente
Write-Host "Configurando variáveis de ambiente..."
[Environment]::SetEnvironmentVariable("OpenCV_DIR", "C:\opencv\build", "User")
[Environment]::SetEnvironmentVariable("PATH", "C:\opencv\build\x64\vc16\bin;" + [Environment]::GetEnvironmentVariable("PATH"), "User")

Write-Host "OpenCV instalado com sucesso!" -ForegroundColor Green
Write-Host "Por favor, reinicie seu terminal ou VS Code para atualizar as variáveis de ambiente."