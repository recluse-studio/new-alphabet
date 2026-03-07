# Contributing

New Alphabet accepts narrow, doctrine-preserving changes.

## Before you change anything

Read:

1. `AGENTS.md`
2. `documentation.md`
3. `prd.json`
4. `progress.txt`

## Working posture

- choose the smallest correct change,
- keep the layer model visible,
- prefer deletion to addition,
- keep APIs finite,
- do not add decorative escape hatches,
- do not broaden the system to satisfy one local case,
- do not ship placeholder scaffolds or partial story work.

## Required workflow

1. Complete the whole story you are working on.
2. Update `prd.json` when the story is actually done.
3. Append factual memory to `progress.txt`.
4. Run the relevant verification commands.
5. Commit with the story id in the message.
6. Push before starting the next story.

## Verification

Current baseline verification is:

```bash
cargo test --workspace
```

If a change affects public docs, examples, scaffolds, or exported context, update those surfaces in the same story.

## Review standard

Treat each change as a constitutional system change.

Review for:

- architecture and layer fit,
- API narrowing or drift,
- state coverage,
- accessibility,
- schema and runtime parity,
- tests,
- documentation,
- and public-release impact.

If the result feels more generic, more decorative, or more optional than before, revise it.
