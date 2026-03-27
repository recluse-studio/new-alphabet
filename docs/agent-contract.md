# Agent Contract

The exported contract bundle format is `0.1.0`.

The thin agent helper crate lives in `crates/new-alphabet-agent` and mirrors the checked-in bootstrap, artifact, and verification paths used by coding sessions.

Use `docs/agent-quickstart.md` for the minimum session-start workflow.

## Required sections

- `doctrine`
- `foundations`
- `flavors`
- `primitives`
- `components`
- `recipes`
- `composition_rules`
- `state_contracts`
- `anti_patterns`
- `validation_rules`
- `schemas`
- `examples`
- `prompt_intents`
- `generation_sequence`
- `repair_sequence`

## Schema set

The current schema crate exports documents for:

- the full contract bundle,
- foundations,
- runtime flavors,
- primitives,
- components,
- recipes,
- and the project manifest.

The checked-in export set lives in `schemas/` and currently includes:

- `context-bundle-0.1.0.json`
- `foundations.json`
- `flavors.json`
- `primitives.json`
- `components.json`
- `recipes.json`
- `anti-patterns.json`
- `examples.json`
- `prompt-intents.json`
- the split `*.schema.json` documents

## Validation categories

- `composition`
- `spacing`
- `state_coverage`
- `accessibility`
- `naming`
- `anti_pattern_usage`

## Prompt and repair model

- generation chooses an explicit runtime flavor when the task names a host stack or platform boundary,
- generation chooses a recipe or allowed primitive composition before writing UI code,
- prompt intents remain finite and tied to real recipes,
- repair prefers deleting invalid structure before adding abstraction,
- explanations cite the same rules used by validation and scaffolding.

## Required session context

- `AGENTS.md`
- `documentation.md`
- `prd.json`
- `progress.txt`
- `schemas/context-bundle-0.1.0.json`

Load targeted layer docs only when the task touches that layer:

- `docs/foundations.md`
- `docs/flavors.md`
- `docs/primitives.md`
- `docs/components.md`
- `docs/recipes.md`
- `docs/cli.md`

## Session bootstrap materials

The checked-in prompt bootstrap set lives in `prompts/` and currently includes:

- `session-bootstrap.md`
- `sparse-blog.md`
- `moderate-review-workspace.md`
- `high-review-workspace.md`

These files are intended to be copy-paste ready in a coding session and to stay in parity with `schemas/prompt-intents.json`.

## Public support artifacts

Reference examples, exported schemas, prompt examples, and refresh scripts are published at:

- `examples/README.md`
- `schemas/README.md`
- `prompts/README.md`
- `scripts/README.md`
- `apps/demo-blog/site/index.html`
- `apps/demo-saas/site/index.html`

## Export path

Use `new-alphabet export context` to emit the current bundle. The exported data is versioned and does not depend on the network.
