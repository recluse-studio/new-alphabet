# New Alphabet Agent Contract

## Thesis

New Alphabet is a Crouwelian alternative to vibe-coded web products.

The doctrine is primary.
The tooling is downstream.
The code must reflect the product:

- severe,
- explicit,
- constrained,
- modern,
- typographic,
- and quiet.

If the code feels generic, clever, bloated, or ornamental, it is wrong.

## Session Start

Read these files at the start of work:

1. `AGENTS.md`
2. `documentation.md`
3. `prd.json`
4. `progress.txt`

If `progress.txt` does not exist, create it.

## Recursive Learning

The agent does not learn by improvising doctrine.
It learns by recording mistakes, discoveries, and completed work in repo memory.

`progress.txt` is required project memory.
Treat it as append-only unless a factual correction is needed, and record corrections as new lines.

Every completed PRD item requires a new one-sentence entry.
Every important mistake requires a new one-sentence entry.
Every important discovery or clarified constraint requires a new one-sentence entry.

Do not keep this learning in chat only.
Write it to `progress.txt`.

## Progress Format

Use this exact line shape:

`YYYY-MM-DD | TYPE | TARGET | One sentence only.`

Allowed `TYPE` values:

- `PRD`
- `MILESTONE`
- `DISCOVERY`
- `MISTAKE`
- `DECISION`

`TARGET` should be the smallest clear identifier available:

- a backlog item such as `B3`
- a milestone such as `M1`
- a workstream such as `WS2`
- a document such as `documentation.md`
- a subsystem such as `primitives`

Keep entries factual, terse, and cumulative.
No paragraphs.
No vague status notes.
No repeated lines.

## Default Posture

- Choose the narrowest correct solution.
- Use the fewest concepts.
- Prefer deletion to addition.
- Prefer static structure to dynamic configuration.
- Make invalid states impossible.
- Make the law visible in the code.
- When in doubt, choose the more severe option.

Terse is good.
Clever compression is not.

## Stack

- Rust first.
- Leptos for web UI.
- SSR and hydration first.
- JSON for schemas, contracts, and exported context.
- Rust CLI.
- Public web standards only.

Do not add another framework layer to hide the stack.

## Rust

- Prefer structs, enums, modules, and free functions.
- Prefer concrete types over traits and generics.
- Add generics only when real duplication exists now.
- Use traits for real polymorphism, not taste.
- Avoid trait objects unless required at a boundary.
- Avoid macros unless they remove repetition without hiding behavior.
- Avoid builder patterns unless construction is genuinely multi-step.
- Keep modules shallow.
- Keep functions short.
- Keep types small.
- Return `Result`. Do not panic in library code.
- Derive only what is needed.
- Name things plainly.
- Comments are rare. Add them only when the rule is not obvious from the code.

## Leptos

- Compose UI in this order: foundation -> primitives -> components -> recipes.
- Keep components small and semantic.
- Keep view bodies shallow.
- Keep prop surfaces narrow and finite.
- Prefer enums over boolean-prop sprawl.
- Use semantic HTML first.
- Make loading, empty, error, disabled, and success states explicit where relevant.
- SSR safety comes first.
- Hydration must not change meaning.
- Do not add ad hoc style escape hatches.

## Design System Implementation

- Tokens only.
- No arbitrary spacing, type, or color values.
- The grid governs layout.
- Typography carries hierarchy.
- Color is secondary.
- One-off visual exceptions are bugs.
- Adapt through approved structures, not open-ended styling.

If a surface looks like generic SaaS, tighten the system.

## CLI, Schema, and Agent Surface

- Schemas are versioned JSON.
- Exports are deterministic.
- Validation is explicit and repair-oriented.
- CLI writes are intentional and visible.
- Core behavior must not depend on the network.
- The agent contract must expose the real doctrine, not a simplified fake one.

Agentic coding is a consequence of explicit law.
Do not weaken the law to make generation easier.

## Forbidden

- future-proofing abstractions
- trait-heavy architecture
- generic wrappers with one caller
- service locators
- hidden global state
- dynamic theming at V0
- runtime style improvisation
- builder chains for simple values
- macros that hide business logic
- helper layers that only rename existing logic
- comments that narrate obvious code
- code written to impress rather than clarify

## Required For New Primitives, Components, and Recipes

- explicit place in the layer model
- narrow API
- state coverage
- schema parity
- example usage
- documentation

## Final Gate

Before shipping any change, ask:

- Is this the smallest correct change?
- Did this reduce or increase concepts?
- Is the API narrower?
- Is the structure more visible?
- Does the code reflect the aesthetic?
- Would another agent learn the system from this code?
- Did I append the required `progress.txt` entry if I completed work, found a mistake, or clarified the doctrine?

If the answer is no, revise.
