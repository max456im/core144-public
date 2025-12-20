```powershell
# PowerShell: сборка embedded (Windows)

param(
  [string]$Target = "thumbv7em-none-eabihf",
  [string]$Profile = "release"
)

Write-Host "🔧 Сборка embedded-продуктов для $Target ($Profile)..." -ForegroundColor Cyan

# minimal-agent
Set-Location products/minimal-agent
cargo build --target $Target --$Profile --no-default-features --features embedded

# shadow-subarchitecture
Set-Location ../shadow-subarchitecture
cargo build --target $Target --$Profile --no-default-features --features embedded

# synthetic-transponder
Set-Location ../synthetic-transponder
cargo build --target $Target --$Profile --no-default-features --features embedded

Write-Host "✅ Готово." -ForegroundColor Green
```