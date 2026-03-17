# Implementation Status

This matrix is truth-first.

Status values are:

- **complete**: implemented and backed by proof artifacts in this repo
- **usable**: real and useful now, but with explicit limits that matter downstream
- **partial**: present, but not yet enough to treat as the primary path
- **planned**: named in doctrine or roadmap, not yet delivered as a current surface

## Foundations

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Foundation token families and contracts | complete | `crates/new-alphabet-foundation/`, `docs/foundations.md`, `schemas/foundations.json` | none in current V0 scope | - |

## Primitives

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Structural primitive inventory (`AppShell`, `PageGrid`, `Region`, `Rail`, `Stack`, `Row`, `ColumnGroup`, `Panel`, `Band`, `SectionHeader`, `Divider`) | complete | `crates/new-alphabet-primitives/src/lib.rs`, `docs/primitives.md` | none in current V0 scope | - |

## Components

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Semantic component inventory | complete | `crates/new-alphabet-components/src/`, `docs/components.md`, `crates/new-alphabet-components/src/accessibility.rs` | none in current V0 scope | - |

## Recipes

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Editorial recipes (`BlogIndex`, `ArticleShell`, `DocsShell`) | complete | `crates/new-alphabet-recipes/src/editorial.rs`, `docs/recipes.md`, `apps/demo-blog/site/`, `apps/docs/site/` | none in current V0 scope | - |
| Workflow recipes (`SearchResultsWorkspace`, `ReviewQueue`, `SettingsWorkspace`, `DashboardShell`) | complete | `crates/new-alphabet-recipes/src/workflow.rs`, `docs/recipes.md`, `apps/demo-saas/site/` | none in current V0 scope | - |

## CLI commands

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| `init` | complete | `crates/new-alphabet-cli/src/lib.rs:204-259`, `crates/new-alphabet-cli/src/lib.rs:976-999`, `examples/greenfield-blog/` | does not edit `Cargo.toml` or app entrypoints by design | - |
| `add recipe <name>` | complete | `crates/new-alphabet-cli/src/lib.rs:261-302`, `crates/new-alphabet-cli/src/lib.rs:1001-1032`, `examples/greenfield-blog/`, `examples/greenfield-workspace/` | scaffold-only by design; downstream wiring remains explicit | - |
| `add component <name>` | complete | `crates/new-alphabet-cli/src/lib.rs:304-332`, `crates/new-alphabet-cli/src/lib.rs:1049-1067`, `docs/cli.md` | requires `new-alphabet.json` and downstream component crate wiring | - |
| `explain <item>` | complete | `crates/new-alphabet-cli/src/lib.rs:334-479`, `crates/new-alphabet-cli/src/lib.rs:1076-1084`, `docs/cli.md` | text output only | - |
| `inspect [path]` | complete | `crates/new-alphabet-cli/src/lib.rs:481-584`, `crates/new-alphabet-cli/src/lib.rs:1086-1121`, `docs/cli.md` | manifest-based only | - |
| `validate [path]` | complete | `crates/new-alphabet-cli/src/lib.rs:586-631`, `crates/new-alphabet-schema/src/validate.rs`, `docs/validation.md` | manifest-based only | - |
| `export context` | complete | `crates/new-alphabet-cli/src/lib.rs:633-655`, `crates/new-alphabet-cli/src/lib.rs:1123-1139`, `schemas/context-bundle-0.1.0.json` | writes only JSON bundle export | - |
| `plan <intent>` | usable | `crates/new-alphabet-cli/src/lib.rs:657-689`, `crates/new-alphabet-cli/src/lib.rs:1142-1172`, `schemas/prompt-intents.json` | recipe selection is heuristic prompt-intent matching, not deep codebase planning | deepen matching only when it can stay narrow and explicit |

## Schema bundle

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Canonical bundle and split exports | complete | `schemas/context-bundle-0.1.0.json`, `schemas/*.json`, `schemas/*.schema.json` | none in current V0 scope | - |

## Docs site

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Generated manual index | usable | `apps/docs/src/main.rs`, `apps/docs/site/index.html` | index surface only; source of truth remains the documents in `docs/` and root docs | expand browsing only if it stays subordinate to the source docs |
| Operator docs set | usable | `README.md`, `docs/getting-started.md`, `docs/status.md`, `docs/cli.md`, `docs/validation.md`, `docs/agent-quickstart.md` | docs are file-based rather than packaged as a separate site experience | - |

## Demo apps

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Editorial reference app | complete | `apps/demo-blog/`, `apps/demo-blog/site/` | larger than a downstream starting point by design | - |
| Workflow reference app | complete | `apps/demo-saas/`, `apps/demo-saas/site/` | larger than a downstream starting point by design | - |
| Downstream consumer proofs | complete | `examples/greenfield-blog/`, `examples/greenfield-workspace/` | path-dependency-first, not packaged as standalone release artifacts | - |

## Agent prompts and context exports

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Session bootstrap prompts | usable | `prompts/session-bootstrap.md`, `prompts/sparse-blog.md`, `prompts/moderate-review-workspace.md`, `prompts/high-review-workspace.md` | prompt library is intentionally small and finite | add new prompts only when they map to real recipe inventory |
| Agent contract and quickstart | usable | `docs/agent-contract.md`, `docs/agent-quickstart.md`, `AGENTS.md` | workflow expects agents to load repo files directly from the source tree | - |

## Validation coverage

| Item | Status | Proof artifact | Known gaps | Next required work if partial |
| --- | --- | --- | --- | --- |
| Manifest schema version, inventory, regions, spacing, naming, state coverage, accessibility, and anti-pattern checks | usable | `crates/new-alphabet-schema/src/validate.rs`, `docs/validation.md`, `examples/validation/invalid-review-queue.json`, `examples/validation/invalid-review-queue.txt` | does not parse arbitrary Rust source, rendered HTML, hydration behavior, or route wiring | only broaden coverage when the rules remain deterministic and repair-oriented |
