# Agent Quickstart

This is the smallest practical session-start path for a coding agent working in New Alphabet.

## Load these files first

Always load:

1. `AGENTS.md`
2. `documentation.md`
3. `prd.json`
4. `progress.txt`
5. `schemas/context-bundle-0.1.0.json`

Then load targeted layer docs only if the task touches that layer:

- `docs/foundations.md`
- `docs/primitives.md`
- `docs/components.md`
- `docs/recipes.md`
- `docs/cli.md`
- `docs/validation.md`
- `docs/getting-started.md`

## Use the context bundle

`schemas/context-bundle-0.1.0.json` is the machine-readable source of truth for:

- foundations
- primitives
- components
- recipes
- composition rules
- state contracts
- anti-patterns
- validation rules
- examples
- prompt intents

Use the split files in `schemas/` only when you need a narrower load.

## Use the prompt set in `prompts/`

Start with:

- `prompts/session-bootstrap.md`

Then choose the guidance level that fits the task:

- `prompts/sparse-blog.md`
- `prompts/moderate-review-workspace.md`
- `prompts/high-review-workspace.md`

Treat the prompts as session-start accelerators, not doctrine replacements.

## Map user intent to the layer model

Use this order:

1. identify whether the intent is **editorial** or **workflow**
2. choose the nearest canonical **recipe**
3. use the recipe to determine required **regions**
4. use the recipe and bundle to determine required **primitives**
5. use the recipe and component contracts to determine allowed **components**
6. scaffold or repair inside that published structure

Do not start from isolated components or ad hoc layout invention when a recipe already exists.

### Current recipe map

- editorial archive or publication index -> `BlogIndex`
- editorial reading surface -> `ArticleShell`
- docs/manual surface -> `DocsShell`
- search-heavy workspace -> `SearchResultsWorkspace`
- approval or moderation queue -> `ReviewQueue`
- settings and preferences -> `SettingsWorkspace`
- metrics and summary workspace -> `DashboardShell`

## Validate generated output

The minimum loop is:

1. scaffold with `new-alphabet init` and `new-alphabet add ...`
2. keep `new-alphabet.json` aligned with the generated surface
3. run `new-alphabet validate`
4. repair toward the published recipe, primitives, components, and rules
5. re-run validation

If the task includes downstream app wiring, also run:

```bash
cargo check
```

or the repo baseline:

```bash
cargo test --workspace
```

## Avoid doctrine drift

Do:

- choose the narrowest valid recipe
- keep names role-based
- keep spacing token-based
- keep states explicit
- prefer deletion to additive abstraction
- cite the bundle, docs, and examples when explaining decisions

Do not:

- invent new regions for an existing recipe
- add style escape hatches
- add custom spacing values
- widen props into freeform variants
- rename surfaces with decorative language
- bypass foundations -> primitives -> components -> recipes

## What an agent must never do in this repo

- weaken the doctrine to satisfy a local request
- treat prompts as more authoritative than the checked-in contract bundle
- document aspirational behavior as if it exists now
- add broad theming or generic starter-kit behavior
- bypass validation when generated output drifts
- omit `progress.txt` updates when work, discovery, or mistakes need to be recorded
