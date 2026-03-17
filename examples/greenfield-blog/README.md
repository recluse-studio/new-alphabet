# Greenfield Blog Proof

This directory is a minimal downstream consumer proof for an editorial surface.

## Chosen recipe

- `BlogIndex`

## Depends on

- `new-alphabet-recipes`
- `leptos`

## Generated New Alphabet files

- `new-alphabet.json`
- `src/new_alphabet/mod.rs`
- `src/new_alphabet/components/mod.rs`
- `src/new_alphabet/recipes/mod.rs`
- `src/new_alphabet/recipes/blog_index.rs`

## App wiring files

- `Cargo.toml`
- `src/main.rs`

## Resulting structure

```text
greenfield-blog/
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
            ├── blog_index.rs
            └── mod.rs
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

- canonical recipe inventory
- canonical region set
- canonical primitive inventory
- required component state coverage
- explicit accessibility snapshot
- no custom spacing values
- no style escape hatch
- no invented layout

## Why this reflects the doctrine

The surface is recipe-first, not theme-first. The manifest names real regions, primitives, and components instead of decorative abstractions, and the scaffold renders the checked-in editorial example rather than inventing local chrome.
