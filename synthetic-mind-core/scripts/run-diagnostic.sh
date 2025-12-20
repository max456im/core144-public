```bash
#!/bin/bash
# Запуск встроенного диагностического режима

set -e

PRODUCT=${1:-synthetic-transponder}
PROFILE=${2:-debug}

if [ ! -d "products/$PRODUCT" ]; then
    echo "❌ Продукт не найден: $PRODUCT"
    exit 1
fi

echo "🔬 Запуск диагностики для $PRODUCT ($PROFILE)..."

cd "products/$PRODUCT"
cargo run --"$PROFILE" --features diagnostics
```