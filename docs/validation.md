# Validation

Validation is the visible enforcement layer for New Alphabet.

Run it with:

```bash
new-alphabet validate
```

or:

```bash
new-alphabet validate /path/to/new-alphabet.json
```

## What validation checks

Validation is currently manifest-based. It reads `new-alphabet.json` and checks the declared surface against the published contract bundle.

| Category | What is checked | Typical rule ids |
| --- | --- | --- |
| Schema parity | `schema_version` matches the current bundle format | `V-000` |
| Inventory | recipes, primitives, and components are canonical and required primitives exist | `AP-011`, `V-009` |
| Composition rules | required regions exist, invalid regions are rejected, bounded side structure is preserved | `V-001`, `V-002`, `P-102` |
| Spacing | spacing tokens are canonical and custom spacing values are rejected | `V-003`, `P-104` |
| Naming | ids stay plain and names stay role-based instead of decorative | `V-004`, `AP-010` |
| State coverage | required component states are present in the manifest | `V-005` |
| Accessibility | semantic HTML, focus-visible behavior, color-independent meaning, accessible names, keyboard support, and text equivalents | `V-006`, `V-007` |
| Anti-patterns | style escape hatches, invented layout, and open-ended variant props are rejected | `V-008`, `AP-008`, `AP-006`, `AP-005`, `AP-009` |

## Severity model

The current severity model is explicit:

- `error`: the manifest is outside the current contract
- `warning`: the manifest is drifting and should be repaired before it becomes normalized
- `note`: the surface passes or the report is summarizing results

Validation output is repair-oriented. Failing messages include a `repair:` line.

## Valid example

Use `examples/greenfield-blog/new-alphabet.json` as a passing example.

Its checked-in expected validation output is:

```text
note [N-001] blog-index: blog-index conforms to the current recipe, state, accessibility, and anti-pattern rules.
note [summary] /absolute/path/to/examples/greenfield-blog/new-alphabet.json: 0 errors, 0 warnings, 1 notes.
```

That example proves:

- schema parity with bundle format `0.1.0`
- canonical editorial recipe usage
- required primitives present
- canonical spacing tokens only
- explicit accessibility snapshot
- no style escape hatch
- no invented layout

## Invalid example

Use `examples/validation/invalid-review-queue.json` as a failing example.

Its checked-in expected output lives in `examples/validation/invalid-review-queue.txt` and includes:

```text
error [V-000] invalid-proof: Manifest schema_version 0.0.0 does not match the current bundle format 0.1.0.
  repair: Regenerate the manifest from the current contract bundle before validating again.
error [V-001] MagicReviewQueue!: ReviewQueue is missing Region(main), so the page grid no longer has a named primary field.
  repair: Add Region(main) before adjusting rails, detail panes, or local composition.
error [V-003] MagicReviewQueue!: spacing.custom.13 is not a canonical New Alphabet spacing token.
  repair: Replace the unknown spacing token with a foundation token or a Stack or Row option.
warning [V-004] MagicReviewQueue!: Magic Review Queue uses decorative language instead of role-based naming.
  repair: Rename the surface by structure or task rather than mood, magic, or ornament.
error [V-005] MagicReviewQueue!: Table is missing required states: loading, empty, error.
  repair: Add the missing state coverage or narrow the component usage until the contract is true.
error [V-006] MagicReviewQueue!: The surface declares semantic_html = false, which violates the accessibility law.
  repair: Restore semantic HTML before refining the surface design.
error [AP-006] MagicReviewQueue!: The manifest declares invented_layout = true, which means layout was improvised outside the grammar.
  repair: Choose a valid recipe or primitive combination before generating more code.
note [summary] /absolute/path/to/examples/validation/invalid-review-queue.json: 25 errors, 3 warnings, 0 notes.
```

## Repair model

Repair should move the project back into the published grammar.

Use this order:

1. fix schema version drift first
2. restore the canonical recipe, regions, and primitives
3. remove invalid spacing values or invented regions
4. restore required states and accessibility declarations
5. remove escape hatches and custom variant props
6. re-run validation

When a surface fails, prefer:

- deleting invalid structure,
- re-entering the published recipe,
- and narrowing the manifest

over inventing new wrappers, variants, or layout names.

## Accessibility checks

Validation currently enforces:

- surface-level `semantic_html = true`
- surface-level `focus_visible = true`
- surface-level `color_independent_meaning = true`
- component accessible names where required
- component keyboard support where required
- component text equivalents for status meaning where required

Component accessibility rules come from `crates/new-alphabet-components/src/accessibility.rs`.

## Known limitations

Validation is real, but it is not omniscient.

It currently does **not**:

- parse arbitrary downstream Rust source,
- inspect rendered HTML or runtime accessibility trees,
- verify route wiring,
- verify hydration behavior,
- detect every possible misuse outside `new-alphabet.json`.

Those limits are intentional. The current validator is deterministic, explicit, and repair-oriented rather than speculative.
