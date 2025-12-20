```bash
#!/bin/bash
# Сборка всех GPL-компонентов как CLI

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR/.."
GPL_MODULES_DIR="$PROJECT_ROOT/gpl-modules"
BUILDDIR="$PROJECT_ROOT/target/gpl-clis"

mkdir -p "$BUILDDIR"

echo "🔍 Сборка GPL-CLI из $GPL_MODULES_DIR..."

for MODULE_DIR in "$GPL_MODULES_DIR"/*/; do
    [ -d "$MODULE_DIR" ] || continue
    MODULE_NAME=$(basename "$MODULE_DIR")
    CARGO_TOML="$MODULE_DIR/Cargo.toml"

    if [ ! -f "$CARGO_TOML" ]; then
        echo "⚠️  Пропуск $MODULE_NAME: нет Cargo.toml"
        continue
    fi

    echo "📦 Сборка $MODULE_NAME..."
    (cd "$MODULE_DIR" && cargo build --release)

    # Имя бинарника: <module_name>-cli
    CLI_NAME="${MODULE_NAME}-cli"
    BIN_PATH="$MODULE_DIR/target/release/$CLI_NAME"

    if [ -f "$BIN_PATH" ]; then
        cp "$BIN_PATH" "$BUILDDIR/"
        echo "✅ $CLI_NAME"
    else
        echo "❌ Не найден бинарник: $BIN_PATH"
        exit 1
    fi
done

echo "✨ Все GPL-CLI в $BUILDDIR"
```