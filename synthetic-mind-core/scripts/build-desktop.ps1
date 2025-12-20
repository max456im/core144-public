```powershell
param([string]$Profile = "release")

Write-Host "💻 Сборка desktop-продуктов..." -ForegroundColor Cyan

# Собрать GPL-CLI
& "$PSScriptRoot/build-gpl-clis.ps1"

# synthetic-transponder
Set-Location products/synthetic-transponder
cargo build --$Profile --features "gpl-analysis diagnostics"

# technical-evangelist
Set-Location ../technical-evangelist
cargo build --$Profile

Write-Host "✅ Готово." -ForegroundColor Green
```