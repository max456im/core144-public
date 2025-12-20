```markdown
# Скрипты проекта

## Unix/Linux/macOS

- `./build_no_std.sh [target] [profile]` — сборка embedded (по умолчанию: `thumbv7em-none-eabihf`, `release`)
- `./build_desktop.sh [profile]` — сборка с GPL и диагностикой
- `./build-gpl-clis.sh` — сборка всех GPL-CLI
- `./install-gpl-clis.sh` — установка CLI в `~/.local/bin`
- `./run-diagnostic.sh <product> [profile]` — запуск диагностики
- `./ci-build.sh [embedded|desktop|gpl|test|all]` — для CI

## Windows (PowerShell)

Откройте PowerShell в корне проекта:

```powershell
# Примеры
.\scripts\win\build-no-std.ps1
.\scripts\win\build-desktop.ps1
.\scripts\win\build-gpl-clis.ps1
```

> 💡 Убедитесь, что у вас установлены:
> - Rust ≥ 1.78 (`rustup`),  
> - Целевые платформы для embedded (`rustup target add ...`),  
> - `cargo` в PATH.
```