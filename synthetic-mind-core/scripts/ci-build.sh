```bash
#!/bin/bash
# Единая команда для CI/CD

set -e

case "${1:-all}" in
  embedded)
    ./scripts/build_no_std.sh
    ;;
  desktop)
    ./scripts/build_desktop.sh
    ;;
  gpl)
    ./scripts/build-gpl-clis.sh
    ;;
  test)
    cargo test --all
    ;;
  all)
    ./scripts/build_no_std.sh
    ./scripts/build_desktop.sh
    cargo test --all
    ;;
  *)
    echo "Использование: $0 [embedded|desktop|gpl|test|all]"
    exit 1
    ;;
esac
```