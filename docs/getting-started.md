# Getting Started

This is the canonical adoption path for New Alphabet.

It is written for:

- Rust and Leptos builders who want a narrow UI constitution instead of a broad theming system,
- AI-assisted builders who need explicit rules, schemas, and repair loops,
- editorial and workflow products that should stay in one severe typographic family.

## Prerequisites

- a current stable Rust toolchain with edition `2024` support
- `cargo` and `rustc`
- a Leptos app or a blank Rust project you are willing to wire to Leptos
- a local clone of this repository

The crates in this repo currently depend on:

- `leptos = { version = "0.8.17", features = ["ssr"] }`

The CLI install path verified during this pass is:

```bash
cargo install --path /absolute/path/to/new-alphabet/crates/new-alphabet-cli --force
```

If you do not want to install the binary, you can run the same commands through Cargo:

```bash
cargo run -q -p new-alphabet-cli --manifest-path /absolute/path/to/new-alphabet/Cargo.toml -- <command>
```

## Greenfield path

This is the smallest tested happy path for a new editorial project.

### 1. Install the CLI

```bash
cargo install --path /absolute/path/to/new-alphabet/crates/new-alphabet-cli --force
```

### 2. Create a blank project

```bash
cargo new greenfield-blog
cd greenfield-blog
```

### 3. Add the runtime dependency you need for the first recipe

Replace `Cargo.toml` with:

```toml
[package]
name = "greenfield-blog"
version = "0.1.0"
edition = "2024"

[dependencies]
leptos = { version = "0.8.17", features = ["ssr"] }
new-alphabet-recipes = { path = "/absolute/path/to/new-alphabet/crates/new-alphabet-recipes" }
```

### 4. Wire the app entrypoint

Replace `src/main.rs` with:

```rust
mod new_alphabet;

use leptos::prelude::*;
use new_alphabet::recipes::blog_index::BlogIndexSurface;

fn main() {
    let _ = view! { <BlogIndexSurface /> };
}
```

### 5. Initialize New Alphabet

```bash
new-alphabet init --name greenfield-blog
```

Expected writes:

- `new-alphabet.json`
- `src/new_alphabet/mod.rs`
- `src/new_alphabet/components/mod.rs`
- `src/new_alphabet/recipes/mod.rs`

### 6. Scaffold the first recipe

```bash
new-alphabet add recipe blog-index
```

Expected writes:

- `src/new_alphabet/recipes/blog_index.rs`
- `src/new_alphabet/recipes/mod.rs`
- `new-alphabet.json`

The scaffold is explicit. It records:

- the chosen recipe id,
- required and optional regions,
- required primitives,
- required components,
- reference examples,
- documentation paths.

### 7. Validate the manifest

```bash
new-alphabet validate
```

Expected output:

```text
note [N-001] blog-index: blog-index conforms to the current recipe, state, accessibility, and anti-pattern rules.
note [summary] /absolute/path/to/greenfield-blog/new-alphabet.json: 0 errors, 0 warnings, 1 notes.
```

### 8. Compile the scaffold

```bash
cargo check
```

### 9. Inspect or explain the structure

```bash
new-alphabet inspect
new-alphabet explain BlogIndex
```

### What `init` does

- creates or updates `new-alphabet.json`
- creates `src/new_alphabet/`
- ensures `components/mod.rs` and `recipes/mod.rs` exist
- keeps writes additive and idempotent

### What `init` does not do

- it does not edit `Cargo.toml`
- it does not add Leptos or New Alphabet dependencies
- it does not register `mod new_alphabet;` in your app
- it does not choose routes or mount points
- it does not generate arbitrary recipes outside the canonical inventory

### How to scaffold a first component

If you need a semantic component scaffold in the same project, add the component crate first:

```toml
new-alphabet-components = { path = "/absolute/path/to/new-alphabet/crates/new-alphabet-components" }
```

Then run:

```bash
new-alphabet add component button
```

Expected writes:

- `src/new_alphabet/components/button.rs`
- `src/new_alphabet/components/mod.rs`

## Existing-project path

This is the additive path for an existing Leptos app.

### 1. Add the New Alphabet crate dependencies you actually need

For recipe scaffolding:

```toml
new-alphabet-recipes = { path = "/absolute/path/to/new-alphabet/crates/new-alphabet-recipes" }
```

For component scaffolding:

```toml
new-alphabet-components = { path = "/absolute/path/to/new-alphabet/crates/new-alphabet-components" }
```

Keep your existing `leptos` dependency aligned with the repo baseline:

```toml
leptos = { version = "0.8.17", features = ["ssr"] }
```

### 2. Initialize in place

```bash
new-alphabet init --path .
```

This creates `new-alphabet.json` and `src/new_alphabet/` without touching the rest of your app.

### 3. Add a governed surface

```bash
new-alphabet add recipe review-queue
```

### 4. Wire the generated module into your existing app

Add:

```rust
mod new_alphabet;
```

Then import the generated surface where you want to mount it:

```rust
use new_alphabet::recipes::review_queue::ReviewQueueSurface;
```

### 5. Validate after integration

```bash
new-alphabet validate
cargo check
```

### Common constraints in an existing app

- keep the scaffolded recipe surface as the top-level governed surface instead of editing it into a generic page shell
- do not add custom spacing values to `new-alphabet.json`
- do not rename surfaces with decorative terms
- do not bypass the generated recipe by composing unsupported regions

## First-run examples

Two checked-in downstream proofs back this document:

- `examples/greenfield-blog/`
  - recipe: `BlogIndex`
  - intent: editorial
  - validation: passing
  - compile proof: `cargo check`
- `examples/greenfield-workspace/`
  - recipe: `ReviewQueue`
  - intent: workflow
  - validation: passing
  - compile proof: `cargo check`

These examples are smaller than the demo apps and closer to what a real consumer starts with.

## Common failure cases

### Missing dependencies

Symptoms:

- `cargo check` fails because `new_alphabet_recipes` or `new_alphabet_components` is unresolved

Repair:

- add the matching path dependency to `Cargo.toml`
- keep `leptos` aligned with `0.8.17` and `features = ["ssr"]`

### Invalid structure

Symptoms:

- `new-alphabet validate` reports missing required regions or primitives

Repair:

- re-run `new-alphabet add recipe <name>` if the scaffold is missing
- restore the canonical regions and primitives in `new-alphabet.json`

### Unsupported composition

Symptoms:

- validation reports an invalid region such as `hero` in a workflow recipe
- validation reports multiple start-side rails

Repair:

- delete the unsupported region
- move extra structure into `main`
- switch to the recipe that owns the structure you need

### Incomplete state handling

Symptoms:

- validation reports missing states for components such as `Table` or `Button`

Repair:

- restore the required states in the manifest
- if the component does not belong on the surface, remove it instead of inventing variants

### Schema drift

Symptoms:

- validation reports `V-000` because `schema_version` does not match the current bundle format

Repair:

- re-run `new-alphabet init`
- keep the manifest aligned with the checked-in bundle format

## What success looks like

You should end up with:

- a `new-alphabet.json` manifest
- `src/new_alphabet/`
- at least one recipe scaffold under `src/new_alphabet/recipes/`
- `new-alphabet validate` reporting `0 errors, 0 warnings`
- `cargo check` succeeding after dependency and entrypoint wiring

For a known-good reference, compare your result with:

- `examples/greenfield-blog/`
- `examples/greenfield-workspace/`
