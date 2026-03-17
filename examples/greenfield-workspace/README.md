# Greenfield Workspace Proof

This directory is a minimal downstream consumer proof for a workflow surface.

## Chosen recipe

- `ReviewQueue`

## Depends on

- `new-alphabet-recipes`
- `leptos`

## Generated New Alphabet files

- `new-alphabet.json`
- `src/new_alphabet/mod.rs`
- `src/new_alphabet/components/mod.rs`
- `src/new_alphabet/recipes/mod.rs`
- `src/new_alphabet/recipes/review_queue.rs`

## App wiring files

- `Cargo.toml`
- `src/main.rs`

## Resulting structure

```text
greenfield-workspace/
├── Cargo.toml
├── README.md
├── new-alphabet.json
└── src
    ├── main.rs
    └── new_alphabet
        ├── components
        │   └── mod.rs
        ├── mod.rs
        └── recipes
            ├── mod.rs
            └── review_queue.rs
```

## Validation

Run:

```bash
new-alphabet validate
```

Expected result:

- `0 errors`
- `0 warnings`
- `1 note`

See `validate.txt`.

## Rules being enforced

- canonical workflow recipe inventory
- required action, main, and detail regions
- required primitive inventory for bounded workspace structure
- required component state coverage
- explicit accessibility snapshot
- no custom spacing values
- no style escape hatch
- no invented layout

## Why this reflects the doctrine

The scaffold keeps queue, action, detail, and optional rail structure explicit. It does not widen into a generic admin shell, and it points back to the checked-in workflow examples rather than creating a one-off local pattern.
