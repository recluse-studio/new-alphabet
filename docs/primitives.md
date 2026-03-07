# Primitives

Primitives define page geometry before semantics.

## Inventory

- `AppShell`
- `PageGrid`
- `Region`
- `Rail`
- `Stack`
- `Row`
- `ColumnGroup`
- `Panel`
- `Band`
- `SectionHeader`
- `Divider`

These live in `crates/new-alphabet-primitives`.

## Allowed combinations

- `P-001`: `AppShell -> PageGrid -> Region(main)`
- `P-002`: `PageGrid -> Rail -> Region(main)`
- `P-003`: `Region(main) -> Stack -> ColumnGroup`
- `P-004`: `Region(main) -> Stack -> Row`
- `P-005`: `Panel -> SectionHeader -> Divider -> content`
- `P-006`: `Band -> SectionHeader`

## Disallowed combinations

- `P-101`: `PageGrid` without `Region(main)`
- `P-102`: multiple rails on one side
- `P-103`: `Rail` inside `ColumnGroup`
- `P-104`: arbitrary spacing wrappers between `Stack` or `Row` children
- `P-105`: `Panel` using color or motion as its only hierarchy signal

## Example mapping

- `EditorialAnchorExample` proves the primary editorial field.
- `WorkflowStructureExample` proves rails, rows, and stack rhythm.
- `SettingsSurfaceExample` proves bounded panel state without ornamental chrome.

These examples are reused by validation, docs, and scaffolding rather than rewritten in parallel.
