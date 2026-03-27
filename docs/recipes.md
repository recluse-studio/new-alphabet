# Recipes

Recipes are full-surface compositions built from foundations, primitives, and semantic components.

Runtime flavor chooses the host stack.
Recipe chooses the surface geometry.

## Editorial recipes

- `BlogIndex`
  - required: `main`
  - optional: `support`
- `ArticleShell`
  - required: `lead`, `main`
  - optional: `support`
- `DocsShell`
  - required: `rail`, `main`
  - optional: `detail`

Editorial surfaces stay archive-led, typographic, and quiet. They should not collapse into marketing or nostalgia.

## Workflow recipes

- `SearchResultsWorkspace`
  - required: `rail`, `main`
  - optional: `action_band`, `detail`
- `ReviewQueue`
  - required: `action_band`, `main`, `detail`
  - optional: `rail`
- `SettingsWorkspace`
  - required: `rail`, `main`
  - optional: `action_band`, `detail`
- `DashboardShell`
  - required: `main`
  - optional: `action_band`, `detail`

Workflow surfaces stay recognizably New Alphabet and do not widen into generic admin templates.

## Example mapping

- editorial examples live in `crates/new-alphabet-recipes/src/editorial.rs`
- workflow examples live in `crates/new-alphabet-recipes/src/workflow.rs`
- validation and exported context point back to those same source paths

## Adaptation rule

If a surface no longer fits a recipe, the first move is to narrow or switch recipes, not invent free layout.
