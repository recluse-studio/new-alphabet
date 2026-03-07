# Contributing

Contributions must preserve the doctrine before they broaden the system.

## Working rules

- choose the narrowest correct change,
- keep the layer model visible,
- prefer deletion to addition,
- keep APIs finite,
- do not add visual escape hatches,
- do not add abstractions with one caller,
- do not drift from the recipe and primitive grammar.

## Required workflow

1. Read `AGENTS.md`, `documentation.md`, `prd.json`, and `progress.txt`.
2. Implement the full story, not a placeholder.
3. Update `prd.json` for the completed story.
4. Append factual repo memory to `progress.txt`.
5. Run the relevant verification commands.
6. Commit and push with the story id in the message.

Common repo scripts:

- `scripts/validate-workspace.sh`
- `scripts/build-demo-sites.sh`
- `scripts/export-public-artifacts.sh`

## Review posture

- treat drift as constitutional breakage, not style preference,
- verify state coverage and accessibility,
- protect family resemblance across editorial and workflow surfaces,
- document real changes in the public docs when behavior or scope changes.

## Anti-broadening rule

If a change makes New Alphabet feel more generic, more decorative, or more optional, it is probably wrong.
