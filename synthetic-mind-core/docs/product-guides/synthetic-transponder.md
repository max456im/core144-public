```markdown
# Synthetic Transponder: Руководство пользователя

## Назначение

Декомпозиция сырого диалога в:
1. Онтологические деревья (`meta-ontology-module`),
2. Инварианты (`Onto8`, `Onto16`),
3. Восприятийные сцены (`OnticGroup`).

## Сборка

```bash
# Без GPL (embedded)
cargo build --no-default-features --features embedded

# С GPL-анализом (desktop)
./scripts/build_desktop.sh
```

## Использование

```rust
let mut trans = SyntheticTransponder::new();
let tree = trans.ingest_dialogue("Он ушёл. Но я остался.");
for onto16 in tree {
    trans.project_scene(onto16);
}
```

## Диагностика

```bash
./scripts/run-diagnostic.sh synthetic-transponder
```

Вывод:
```
=== DIAGNOSTIC: Synthetic Transponder ===
Scene[0]: coherence = 0.82
Scene[1]: coherence = 0.35  ← искажение!
```

## Лицензия

Apache 2.0 — можно использовать в коммерческих продуктах.
```
