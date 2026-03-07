# CLI

The CLI name is `new-alphabet`.

## Commands

- `new-alphabet init`
  - creates `new-alphabet.json`
  - creates `src/new_alphabet/`
  - is idempotent when the project is already wired
  - keeps writes additive and explicit
- `new-alphabet add recipe <name>`
  - writes an approved recipe scaffold
  - includes required regions, primitives, components, examples, and documentation paths
  - updates the manifest with a contract-backed surface entry
- `new-alphabet add component <name>`
  - writes a semantic component scaffold
  - keeps state coverage, primitive ancestry, foundation bindings, and documentation paths explicit in the generated file
- `new-alphabet explain <item>`
  - explains a primitive, component, recipe, token, validation rule, or anti-pattern in framework terms
- `new-alphabet inspect [path]`
  - inspects a manifest and reports its surfaces, recipes, regions, and component usage
- `new-alphabet validate [path]`
  - runs constitutional validation
  - reports `error`, `warning`, and `note`
  - includes repair guidance
- `new-alphabet export context [--output path]`
  - exports the canonical contract bundle
  - stays network-free
  - the repo currently checks in the exported results under `schemas/`
- `new-alphabet plan <intent>`
  - emits a structured recipe-first plan from the prompt intent catalog

## Write policy

- write commands say what they changed,
- write commands prefer local additive files,
- read-only commands do not mutate project state,
- generated files are valid starting points rather than placeholders.
