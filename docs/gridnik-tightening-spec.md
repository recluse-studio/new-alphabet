# Gridnik Tightening Spec

This document is a corrective spec.

Its purpose is to pull the recently added desktop workbench guidance closer to the non-negotiable New Alphabet constitution and closer to a stricter Gridnik posture:

- grid-governed,
- typographic,
- severe,
- explicit,
- reduced,
- and quiet.

It does not create a new aesthetic.
It narrows the current additions so they stop drifting toward toolkit-shaped convenience.

## Why this exists

The recent flavor and workbench additions improved host coverage, but they also introduced two forms of looseness that are not constitutional:

1. broad runtime buckets that hide the real host stack boundary,
2. approximate density rules that weaken token law.

Those moves make the system easier to prompt against, but they make it less exact.
New Alphabet should become harder to drift, not easier.

## Target posture

Everything added for dense desktop work surfaces should move toward these traits:

- grid before widget,
- structure before styling,
- typography before iconography,
- exact dense defaults before host-specific variation,
- plain surfaces before cards,
- explicit hierarchy before expressive decoration,
- host-specific law only where the host actually requires it.

## Non-negotiable corrections

### 1. Exact token law beats approximate density

Where a cross-stack rule exists, it must bind to exact values, not soft ranges.

Required tightening:

- default workbench policy should use exact dense values as its primary law,
- ranges may exist only as host-specific constraints inside a runtime flavor when the host toolkit truly forces a tolerance band,
- phrases such as "around", "mostly", "about", and "usually" should be removed from normative rules,
- if a value is not exact, the spec must explain why the host boundary prevents exactness.

### 2. Runtime flavor must bind a real host boundary

A runtime flavor must name one concrete runtime boundary or one tightly coupled shell-plus-frontend boundary.

Required tightening:

- split or remove broad flavor buckets that cover multiple unrelated runtimes,
- move cross-stack shell guidance into workbench policy rather than flavor contracts,
- do not let a flavor become a surrogate for design policy.

### 3. Grid law must become visible again

The current additions bias toward desktop density, but they do not bind strongly enough back to grid-governed composition.

Required tightening:

- every workbench flavor should state its shell in region terms first,
- every workbench flavor should declare how left rail, main surface, toolbar, and inspector map back to recipe geometry,
- shell rules should prefer explicit pane structure over free composition primitives,
- all dense desktop guidance should treat grid and region alignment as first-class rather than implied.

### 4. Card abolition should be stricter

Routine product work should default to rows, panels, dividers, lists, tables, and inspectors.

Required tightening:

- "card" may appear only when the card itself is the interaction surface,
- flavors and policy should ban dashboard mosaics, stacked routine cards, framed wrapper pyramids, and ornamental badges in one place with no ambiguity,
- workbench examples should visually read as bands, panes, rows, and tables rather than collections of units.

### 5. Typography must outrank display

Gridnik-leaning New Alphabet should feel typographic before it feels branded.

Required tightening:

- compact body size stays primary,
- section titles stay structural rather than promotional,
- page titles stay restrained and rare,
- icons never dominate labels,
- headings describe the area or action plainly,
- no display-text logic should appear in product workbench guidance.

### 6. Chrome budget must be explicit

The current rules ban many bad patterns, but they do not yet define a chrome budget clearly enough.

Required tightening:

- one toolbar row by default,
- one accent color only,
- thin dividers over framed boxes,
- local scrolling only where the content itself requires it,
- wrapper depth should be audited and capped,
- regions that do not change task comprehension should be removed.

## Required repo changes

### A. Reclassify `DesktopHtmlWorkbench`

Current problem:

- it spans multiple runtimes and a shell concept at once,
- it acts like policy disguised as flavor,
- it weakens the meaning of explicit host-stack binding.

Required action:

- remove it as a flavor, or split it into concrete bindings,
- keep cross-stack HTML-semantics guidance in `docs/workbench-policy.md`,
- if selection guidance is needed, record it separately as runtime flavor selection policy rather than flavor law.

### B. Tighten `docs/workbench-policy.md`

Current problem:

- it mixes exact defaults with soft ranges,
- it allows a little too much host-shaped variation,
- it still talks about desktop density partly in prose rather than wholly in law.

Required action:

- keep the exact default density module as the governing rule,
- demote secondary ranges into exception notes or remove them,
- add an explicit grid and region law section,
- add an explicit chrome budget section,
- keep the seven-point review, but rewrite it as pass or fail acceptance criteria.

### C. Normalize flavor wording

Current problem:

- several flavors use approximate terms like "around", "mostly", or tolerance-heavy prose,
- that weakens the constitutional preference for explicit, finite rules.

Required action:

- rewrite flavor docs and schema entries to use exact defaults where possible,
- keep tolerance only where the host toolkit forces it,
- add one sentence in each flavor explaining why any remaining tolerance exists.

### D. Add drift checks

Required action:

- add validation rules or lint-style checks for approximate normative words in policy docs and flavor law,
- add validation for forbidden broad runtime buckets,
- add validation for workbench policy drift such as hero, banner, card mosaic, decorative gradient, and wrapper-depth violations.

## Acceptance criteria

The tightening work is complete only when all of these are true:

1. No cross-stack design policy is mislabeled as a runtime flavor.
2. Dense workbench defaults are exact before they are flexible.
3. Every host flavor maps back to shell and region structure explicitly.
4. No product workbench rule relies on display-style rhetoric or marketing metaphors.
5. Card usage is exceptional rather than routine.
6. Typography, grid, and hierarchy do more work than chrome.
7. A generated workbench surface reads as a serious desktop product in three seconds.

## Implementation order

Apply this tightening in this order:

1. reclassify or remove broad flavor buckets,
2. tighten workbench policy to exact defaults,
3. normalize all flavor wording,
4. add drift checks,
5. regenerate public artifacts,
6. review the docs site and example outputs for visual severity and quietness.
