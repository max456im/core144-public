```bash
#!/bin/bash
# Установка GPL-CLI в ~/.local/bin

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILDDIR="$SCRIPT_DIR/../target/gpl-clis"
INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"

if [ ! -d "$BUILDDIR" ]; then
    echo "❌ Сначала запустите: ./scripts/build-gpl-clis.sh"
    exit 1
fi

for CLI in "$BUILDDIR"/*; do
    if [ -f "$CLI" ]; then
        CLI_NAME=$(basename "$CLI")
        cp "$CLI" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/$CLI_NAME"
        echo "🔗 Установлен: $CLI_NAME"
    fi
done

echo "💡 Добавьте в ~/.bashrc или ~/.zshrc:"
echo "export PATH=\"\$PATH:\$HOME/.local/bin\""
```