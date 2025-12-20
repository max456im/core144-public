```bash
#!/bin/bash
# Сборка desktop-продуктов с поддержкой диагностики и GPL

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

PROFILE=${1:-release}

echo "💻 Сборка desktop-продуктов ($PROFILE)..."

# Сначала соберём GPL-CLI
"$SCRIPT_DIR/build-gpl-clis.sh"

# 1. synthetic-transponder (с GPL)
echo "📦 synthetic-transponder (desktop)"
cd products/synthetic-transponder
cargo build --"$PROFILE" --features "gpl-analysis diagnostics"

# 2. technical-evangelist (GPL-продукт)
echo "📦 technical-evangelist"
cd ../technical-evangelist
cargo build --"$PROFILE"

# 3. minimal-agent (desktop-режим, для тестов)
echo "📦 minimal-agent (desktop)"
cd ../minimal-agent
cargo build --"$PROFILE" --features "std diagnostics"

echo "✅ Все desktop-продукты собраны в target/$PROFILE/"
```
