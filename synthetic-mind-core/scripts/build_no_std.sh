```bash
#!/bin/bash
# Сборка embedded-продуктов (no_std, без GPL)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/.."

TARGET=${1:-thumbv7em-none-eabihf}
PROFILE=${2:-release}

echo "🔧 Сборка embedded-продуктов для $TARGET ($PROFILE)..."

# 1. minimal-agent
echo "📦 minimal-agent"
cd products/minimal-agent
cargo build --target "$TARGET" --"$PROFILE" --no-default-features --features embedded

# 2. shadow-subarchitecture
echo "📦 shadow-subarchitecture"
cd ../shadow-subarchitecture
cargo build --target "$TARGET" --"$PROFILE" --no-default-features --features embedded

# 3. synthetic-transponder (в embedded-режиме)
echo "📦 synthetic-transponder (embedded)"
cd ../synthetic-transponder
cargo build --target "$TARGET" --"$PROFILE" --no-default-features --features embedded

echo "✅ Все embedded-продукты собраны в target/$TARGET/$PROFILE/"
```