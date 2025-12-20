```powershell
$ProjectRoot = Resolve-Path "$PSScriptRoot/../.."
$GplDir = "$ProjectRoot/gpl-modules"
$BuildDir = "$ProjectRoot/target/gpl-clis"

if (!(Test-Path $BuildDir)) { New-Item -ItemType Directory -Path $BuildDir }

Write-Host "🔍 Сборка GPL-CLI..." -ForegroundColor Cyan

Get-ChildItem $GplDir -Directory | ForEach-Object {
  $module = $_.Name
  Write-Host "📦 $module"
  Set-Location $_.FullName
  cargo build --release

  $cliName = "$module-cli.exe"
  $binPath = "$_.FullName\target\release\$cliName"
  if (Test-Path $binPath) {
    Copy-Item $binPath $BuildDir
    Write-Host "✅ $cliName"
  } else {
    Write-Error "❌ Бинарник не найден: $binPath"
    exit 1
  }
}

Write-Host "✨ Все CLI в $BuildDir" -ForegroundColor Green
```