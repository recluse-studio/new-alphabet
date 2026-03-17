# New Alphabet

New Alphabet is a Crouwelian design constitution for Rust and Leptos: a severe, grid-governed, typographic system for building editorial and workflow products without drifting into generic SaaS defaults, decorative escape hatches, or prompt-by-prompt improvisation. The doctrine is primary. The framework crates, CLI, schemas, demos, prompts, and docs exist to make that doctrine installable, executable, and verifiable by humans and coding agents.

## Status

New Alphabet is a **V0 preview**.

- doctrine: stable
- implementation: usable
- adoption posture: ready for evaluation in greenfield or near-blank Leptos work
- release posture: path-dependency-first; the repo is the source of truth

Use `docs/status.md` for the proof-backed implementation matrix.

## Quickstart

The shortest happy path is:

```bash
git clone https://github.com/recluse-studio/new-alphabet.git
cd new-alphabet
cargo install --path crates/new-alphabet-cli --force

cd /path/to/your/project
new-alphabet init --name your-project
new-alphabet add recipe blog-index
new-alphabet validate
```

That path creates `new-alphabet.json`, `src/new_alphabet/`, and a first recipe scaffold. It does **not** wire your `Cargo.toml`, routes, or app entrypoint. Use `docs/getting-started.md` for the full greenfield and existing-project flows, including the tested `Cargo.toml` and `main.rs` wiring needed to compile the scaffold.

## What is implemented now

- **Foundations**: canonical token families for layout, spacing, type, density, color, border, motion, and state in `crates/new-alphabet-foundation`, with matching docs and schema exports.
- **Primitives**: `AppShell`, `PageGrid`, `Region`, `Rail`, `Stack`, `Row`, `ColumnGroup`, `Panel`, `Band`, `SectionHeader`, and `Divider` in `crates/new-alphabet-primitives`.
- **Components**: the V0 semantic component set in `crates/new-alphabet-components`, including actions, fields, choices, status, data display, and workflow chrome.
- **Recipes**: editorial `BlogIndex`, `ArticleShell`, `DocsShell` and workflow `SearchResultsWorkspace`, `ReviewQueue`, `SettingsWorkspace`, `DashboardShell` in `crates/new-alphabet-recipes`.
- **CLI**: `init`, `add recipe`, `add component`, `explain`, `inspect`, `validate`, `export context`, and `plan` in `crates/new-alphabet-cli`.
- **Schemas**: checked-in contract bundle, split exports, and JSON schemas in `schemas/`.
- **Demos**: `apps/demo-blog`, `apps/demo-saas`, and `apps/docs`, with checked-in static outputs under each `site/` directory.
- **Docs**: doctrine in `documentation.md`, public operator docs in `docs/`, and focused downstream proofs in `examples/`.
- **Prompts and agent context**: reusable bootstrap material in `prompts/` and exported context in `schemas/context-bundle-0.1.0.json`.

## What is not complete yet

- `plan` is **usable**, but its matching is still heuristic prompt-intent selection rather than deep planning.
- validation is **repair-oriented and real**, but it is manifest-based; it does not inspect arbitrary downstream Rust source, rendered HTML, or route wiring.
- `init` and `add` keep writes explicit and additive; they do not edit `Cargo.toml`, register modules in your app entrypoint, or choose routes for you.
- the docs site in `apps/docs` is a generated manual index, not a full documentation browser.
- the repo is the release vehicle today; the workflow is not yet packaged as a crates.io-first distribution.

## How to use this repo

Use this repository in four ways:

1. **Framework source**
   - depend on the crates directly from this repo,
   - scaffold a governed surface with `new-alphabet`,
   - keep your project manifest aligned with the generated structure.
2. **Docs reference**
   - read the layer docs in `docs/`,
   - use `documentation.md` for the full human-readable doctrine,
   - use `docs/status.md` when you need truth before inference.
3. **Agent context source**
   - load `AGENTS.md`, `documentation.md`, `prd.json`, `progress.txt`, and `schemas/context-bundle-0.1.0.json`,
   - use `docs/agent-quickstart.md` for the minimum viable session-start path.
4. **Validation and constitution source**
   - scaffold from the canonical inventory,
   - run `new-alphabet validate`,
   - repair toward the published recipes, primitives, components, and rules rather than improvising around them.

## Canonical links

- [Getting started](docs/getting-started.md)
- [Implementation status](docs/status.md)
- [Foundations](docs/foundations.md)
- [Primitives](docs/primitives.md)
- [Components](docs/components.md)
- [Recipes](docs/recipes.md)
- [CLI](docs/cli.md)
- [Validation](docs/validation.md)
- [Agent quickstart](docs/agent-quickstart.md)
- [Agent contract](docs/agent-contract.md)
- [Roadmap](ROADMAP.md)
- [Examples](examples/README.md)

## Verification

Current baseline verification is:

```bash
cargo test --workspace
```
