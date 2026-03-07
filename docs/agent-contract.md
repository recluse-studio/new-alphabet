# Agent Contract

The exported contract bundle format is `0.1.0`.

## Required sections

- `doctrine`
- `foundations`
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
- primitives,
- components,
- recipes,
- and the project manifest.

The checked-in export set lives in `schemas/` and currently includes:

- `context-bundle-0.1.0.json`
- `foundations.json`
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

- generation chooses a recipe or allowed primitive composition before writing UI code,
- prompt intents remain finite and tied to real recipes,
- repair prefers deleting invalid structure before adding abstraction,
- explanations cite the same rules used by validation and scaffolding.

## Export path

Use `new-alphabet export context` to emit the current bundle. The exported data is versioned and does not depend on the network.
