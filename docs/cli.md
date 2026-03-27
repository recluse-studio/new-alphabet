# CLI

The CLI name is `new-alphabet`.

## Global behavior

- success exits with code `0`,
- command or validation errors exit with code `2`,
- failure to read the current working directory exits with code `1`,
- write commands print exact file writes,
- read-only commands do not mutate project state,
- all command behavior is network-free.

## `new-alphabet init`

- **Status**: complete
- **Syntax**: `new-alphabet init [--path <path>] [--name <project-name>]`
- **Arguments**
  - `--path <path>` writes into another directory instead of the current one
  - `--name <project-name>` sets `project_name` in `new-alphabet.json`
- **Outputs**
  - prints `init: wrote ...` with the exact files created or updated
  - prints `init: no file changes` when re-run without needing updates
- **File writes**
  - `new-alphabet.json`
  - `src/new_alphabet/mod.rs`
  - `src/new_alphabet/components/mod.rs`
  - `src/new_alphabet/recipes/mod.rs`
- **What it does not do**
  - does not edit `Cargo.toml`
  - does not register modules in your app entrypoint
  - does not pick routes or generate arbitrary product structure
- **Example**

```bash
new-alphabet init --name greenfield-blog
```

```text
init: wrote new-alphabet.json, src/new_alphabet/mod.rs, src/new_alphabet/components/mod.rs, src/new_alphabet/recipes/mod.rs
```

- **Failure cases**
  - `init requires a value after --path`
  - `init requires a value after --name`
  - `init accepts only --path and --name`

## `new-alphabet add recipe <name>`

- **Status**: complete
- **Syntax**: `new-alphabet add recipe <name>`
- **Arguments**
  - `<name>` must be a canonical recipe id or a normalized match such as `blog-index` or `review-queue`
- **Outputs**
  - prints `add recipe: wrote ...` with the scaffold and manifest updates
  - prints `add recipe: no file changes` if the scaffold and manifest entry already exist
- **File writes**
  - `src/new_alphabet/recipes/<module>.rs`
  - `src/new_alphabet/recipes/mod.rs`
  - `new-alphabet.json` when the surface is newly added
- **Behavior**
  - writes a recipe scaffold that exposes required regions, optional regions, required primitives, required components, reference examples, and documentation paths
  - adds a contract-backed surface entry to `new-alphabet.json`
- **Example**

```bash
new-alphabet add recipe blog-index
```

```text
add recipe: wrote src/new_alphabet/recipes/blog_index.rs, src/new_alphabet/recipes/mod.rs, new-alphabet.json
```

- **Failure cases**
  - `unsupported recipe; use one of blog-index, article-shell, docs-shell, search-results-workspace, review-queue, settings-workspace, or dashboard-shell`
  - `new-alphabet.json` missing or unreadable

## `new-alphabet add component <name>`

- **Status**: complete
- **Syntax**: `new-alphabet add component <name>`
- **Arguments**
  - `<name>` must be a canonical component id or normalized match such as `button` or `detail-pane`
- **Outputs**
  - prints `add component: wrote ...` with the scaffold files it changed
  - prints `add component: no file changes` when re-run against the same scaffold
- **File writes**
  - `src/new_alphabet/components/<module>.rs`
  - `src/new_alphabet/components/mod.rs`
- **Behavior**
  - writes a semantic component scaffold with required states, primitive ancestry, foundation bindings, example ids, and documentation paths
- **Example**

```bash
new-alphabet add component button
```

```text
add component: wrote src/new_alphabet/components/button.rs, src/new_alphabet/components/mod.rs
```

- **Failure cases**
  - `new-alphabet.json is missing; run \`new-alphabet init\` first`
  - `unsupported component; use one of ...`

## `new-alphabet explain <item>`

- **Status**: complete
- **Syntax**: `new-alphabet explain <item>`
- **Arguments**
  - `<item>` may be a foundation family, foundation token, primitive, component, recipe, composition rule, validation rule, or anti-pattern id
- **Outputs**
  - prints a structured explanation in framework terms
- **File writes**
  - none
- **Example**

```bash
new-alphabet explain BlogIndex
```

```text
recipe BlogIndex
purpose: Archive-first publication surface for essays and notes.
required_regions: main
optional_regions: support
primitives: AppShell, PageGrid, Region, Panel, Stack, SectionHeader
components: EmptyState, NavIndex
adaptation_limits: Taxonomy and archive support remain optional. | The surface must read as archive or publication rather than marketing landing page.
examples: BlogIndexExample, BlogIndexMinimalExample
```

- **Failure cases**
  - `explain requires a primitive, component, recipe, token, or rule id`
  - `No matching primitive, component, recipe, rule, or token was found.`

## `new-alphabet inspect [path]`

- **Status**: complete
- **Syntax**
  - `new-alphabet inspect`
  - `new-alphabet inspect <path>`
- **Arguments**
  - `<path>` may be a directory containing `new-alphabet.json` or a direct path to a manifest file
- **Outputs**
  - prints project name, schema version, surfaces, recipes, primitives, components, and validation counts
  - prints each surface with regions, primitives, components, and active violations
- **File writes**
  - none
- **Example**

```bash
new-alphabet inspect
```

- **Failure cases**
  - manifest path missing
  - manifest unreadable or invalid JSON

## `new-alphabet validate [path]`

- **Status**: complete
- **Syntax**
  - `new-alphabet validate`
  - `new-alphabet validate <path>`
  - `new-alphabet validate --path <path>`
- **Arguments**
  - `<path>` may be a directory containing `new-alphabet.json` or a direct path to a manifest file
- **Outputs**
  - prints one line per `error`, `warning`, or `note`
  - prints repair guidance for each failing message
  - prints a summary line with total counts
- **File writes**
  - none
- **Example**

```bash
new-alphabet validate
```

```text
note [N-001] blog-index: blog-index conforms to the current recipe, state, accessibility, and anti-pattern rules.
note [summary] /tmp/new-alphabet-proof-blog/new-alphabet.json: 0 errors, 0 warnings, 1 notes.
```

- **Failure cases**
  - `validate requires a value after --path`
  - manifest unreadable or invalid JSON

## `new-alphabet export context`

- **Status**: complete
- **Syntax**
  - `new-alphabet export context`
  - `new-alphabet export context --output <path>`
- **Arguments**
  - `--output <path>` writes the exported bundle to disk instead of stdout
- **Outputs**
  - prints the full JSON bundle to stdout when `--output` is omitted
  - prints `export context wrote <path>` when an output path is given
- **File writes**
  - only the path passed to `--output`
- **Example**

```bash
new-alphabet export context --output ./context.json
```

```text
export context wrote /absolute/path/to/context.json
```

- **Failure cases**
  - `export currently supports only \`new-alphabet export context\``
  - `export context requires a value after --output`

## `new-alphabet plan <intent>`

- **Status**: usable
- **Syntax**: `new-alphabet plan <intent>`
- **Arguments**
  - `<intent>` is free text describing the product or surface you want to build
- **Outputs**
  - prints a recipe-first plan with required regions, primitives, recommended components, reference examples, steps, and validation focus
- **File writes**
  - none
- **Current behavior**
  - matches the prompt-intent catalog heuristically from token overlap
  - selects the best current recipe match from the published bundle
  - does not synthesize new recipes or inspect your codebase
- **Example**

```bash
new-alphabet plan "Create a dense review queue for moderation decisions with detail pane"
```

```text
plan prompt.review_workspace_dense
recipe: ReviewQueue
required_regions: action_band, main, detail
required_primitives: AppShell, PageGrid, Rail, Region, Panel, Stack, Band
recommended_components: CommandBar, InlineAlert, Table, FilterRail, DetailPane, NavIndex
reference_examples: ReviewQueueExample, ReviewQueueLoadingExample, ReviewQueueEmptyExample, ReviewQueueUnavailableDetailExample
steps:
- Choose ReviewQueue because the prompt explicitly names navigation, results, detail, and action structure.
- Keep the action strip in action_band, results in main, and dirty feedback adjacent to the detail path rather than decorative chrome.
- Validate loading, unavailable, focus, and state-change coverage before exporting the scaffold.
validation_focus: composition, state_coverage, accessibility, naming
```

- **Failure cases**
  - `plan requires a product intent string`
  - no prompt intents available in the bundle
- `new-alphabet add recipe <name>`
  - writes an approved recipe scaffold
  - includes required regions, primitives, components, examples, and documentation paths
  - updates the manifest with a contract-backed surface entry
- `new-alphabet add component <name>`
  - writes a semantic component scaffold
  - keeps state coverage, primitive ancestry, foundation bindings, and documentation paths explicit in the generated file
- `new-alphabet explain <item>`
  - explains a flavor, primitive, component, recipe, token, validation rule, or anti-pattern in framework terms
  - cites the structural inventory and reference examples behind the item
- `new-alphabet inspect [path]`
  - inspects a manifest and reports its surfaces, recipes, regions, primitives, and component usage
  - includes active rule violations in framework language
- `new-alphabet validate [path]`
  - runs constitutional validation
  - reports `error`, `warning`, and `note`
  - includes repair guidance
- `new-alphabet generate --prompt <intent> [--path path] [--name name]`
  - chooses the closest prompt intent from the contract bundle
  - writes `new-alphabet.json`
  - writes `new-alphabet-site.json` as the editable site source
  - renders a viewable site under `site/`
  - emits `site/assets/new-alphabet.css` from the foundation runtime
- `new-alphabet render [path]`
  - re-renders `site/` from `new-alphabet-site.json`
  - keeps HTML and CSS deterministic
  - allows agents to edit JSON and rebuild without touching framework internals
- `new-alphabet export context [--output path]`
  - exports the canonical contract bundle
  - stays network-free
  - the repo currently checks in the exported results under `schemas/`
- `new-alphabet plan <intent>`
  - emits a structured recipe-first plan from the prompt intent catalog
  - includes the selected runtime flavor plus the required regions, primitives, components, and reference examples for the selected recipe

## Write policy

- write commands say exactly what they changed,
- write commands prefer local additive files,
- generated files are valid starting points rather than placeholders,
- read-only commands do not mutate project state,
- generated sites keep editable source in JSON and render output under `site/`.
- the CLI never depends on the network for core behavior.
