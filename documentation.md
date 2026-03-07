# New Alphabet

_A Crouwelian design-system engine for Rust, Leptos, and agentic product building._

This document is the human-readable reference for New Alphabet as defined by `prd.json`. It preserves the PRD's intent, expands the areas that are still terse, and adds clearly marked working interpretations where the PRD is directional but not yet fully specified.

## Document Purpose

This document exists to answer four questions:

1. What is New Alphabet actually trying to be?
2. What doctrine governs its design and code?
3. What must ship at V0?
4. How should a human or agent work inside the system without diluting it?

The short answer is:

- New Alphabet is a strict alternative to vibe-coded web products.
- It uses a Crouwelian design constitution rather than loose taste.
- It is implemented as a Rust-first, Leptos-native framework.
- It is distributed as framework code, a CLI, and an agent-readable contract surface.

## Status

| Field | Value |
| --- | --- |
| Project name | `New Alphabet` |
| Namesake | Wim Crouwel's _New Alphabet_ |
| Category | Rust-first web framework |
| Current status | proposed |
| Open source intent | yes |
| Primary release channel | GitHub |
| License target | `MIT OR Apache-2.0` |
| Repo strategy | monorepo |
| Default branch | `main` |
| PRD version | `1.0.0` |

## Executive Summary

New Alphabet exists because too much modern web work is built without a governing language. It is assembled from trend residue, flexible component libraries, scattered prompt output, and local taste decisions. The result is visual drift, generic SaaS sameness, and systems that get less coherent as they grow.

New Alphabet's answer is not just another component library. It is a design constitution:

- grid-governed,
- typographic,
- severe,
- constrained,
- modern,
- and explicit enough for both humans and LLMs to use without guessing.

The product shape has three parts:

- a Rust and Leptos framework with foundations, primitives, components, and recipes,
- a CLI named `new-alphabet`,
- and a machine-readable agent contract surface for planning, validation, explanation, and scaffold generation.

The doctrine is primary. The tooling exists to preserve and scale that doctrine.

## The Problem

The PRD identifies several interlocking problems:

- broad UI libraries optimize for breadth and themeability rather than coherence,
- styling freedom encourages improvisation instead of composition,
- product surfaces drift over time because nothing governs them structurally,
- AI-assisted implementation amplifies that drift when there is no installable design constitution,
- and teams end up with interfaces that are vaguely contemporary but not truly legible, distinctive, or repeatable.

### What "vibe-coded" means here

In this project, "vibe-coded" does not simply mean "AI-generated." It means work produced without a durable governing system.

Typical signs include:

- inconsistent spacing and hierarchy,
- arbitrary visual exceptions,
- local component decisions that do not add up to a family,
- interchangeable "tasteful SaaS" screens,
- prompt-driven layout invention,
- and code that reflects trend-following rather than law.

New Alphabet is a direct rejection of that posture.

## The Solution

New Alphabet proposes a stricter alternative:

- one constitutional grammar,
- one layered architecture,
- one monorepo where framework, CLI, schema, demos, docs, and agent surface evolve together,
- and one set of explicit rules that can be used by both humans and agents.

The point is not to eliminate variety. The point is to make variety emerge from composition, density, content, and region structure rather than from arbitrary styling freedom.

## Why Now

The PRD gives four reasons this project is timely:

1. Rust and Leptos now make a serious full-stack Rust web framework viable.
2. AI coding sessions are becoming normal, but they still lack durable design constitutions.
3. Teams need repeatable systems for both content-heavy and workflow-heavy products.
4. A sharply opinionated OSS offering is more memorable than another generic component library.

## Release Thesis

New Alphabet should launch as both:

- a real framework for building blog and SaaS surfaces, and
- a live system an LLM can load into a coding session to plan, scaffold, implement, validate, and explain product UIs with low drift.

The critical order matters:

- first, a real doctrine,
- then a real implementation,
- then a real contract surface that exposes that implementation to agents.

## Core Thesis

New Alphabet is a Crouwelian design constitution for the web: a strict, typographic, grid-governed alternative to vibe-coded products.

That thesis has three parts:

1. The failure in modern product UI is not just poor taste; it is the absence of a governing language.
2. The answer is not more options; it is stronger law.
3. Once the law is explicit, it can be installed into coding sessions and used by agents without the system collapsing into generic prompt output.

## Goals

### Primary goals

- Build a Rust-first framework for coherent web blogs and web-based SaaS.
- Use Leptos as the core application framework for SSR, hydration, and shared Rust logic.
- Encode a strict Crouwelian design grammar across foundation, primitives, semantic components, and recipes.
- Provide a CLI that can scaffold, inspect, validate, and explain New Alphabet structures.
- Provide an agent-facing contract surface that an LLM can load into a coding session and build from.
- Ship a serious first public OSS release with strong docs, examples, and release discipline.

### Secondary goals

- Demonstrate one grammar powering both editorial and SaaS surfaces.
- Support both sparse prompts and highly directed requests.
- Make the framework feel like a school of design rather than a generic toolkit.
- Prepare the system for future agent validation and recipe synthesis.

### Working interpretation of the goals

The primary goals describe what must become concrete. The secondary goals describe what must become obvious in use.

If New Alphabet ships code but does not feel like a coherent school of design, it will have missed part of its own thesis.

## Non-Goals

New Alphabet does not aim to:

- compete with broad ecosystems on component count,
- support unlimited styling freedom at V0,
- imitate only Crouwel's look while ignoring his logic,
- use AI as a source of decorative randomness,
- ship native mobile or desktop runtimes initially,
- build multi-framework adapters before Leptos is proven,
- or claim a full autonomous design agent at V0.

### Practical reading of the non-goals

These non-goals should actively narrow implementation choices.

For example:

- if a feature mainly exists to increase optionality, it is probably out of scope,
- if a component exists only because another library has one, it is probably out of scope,
- if a design choice makes the system feel more generic in exchange for more "flexibility," it is probably wrong.

## What New Alphabet Is

New Alphabet is:

- a Rust-first, Leptos-native framework for web blogs, documentation sites, and web-based SaaS,
- a strict visual and compositional doctrine, not just a package of components,
- a design grammar that can generate both editorial and operational surfaces,
- a CLI for scaffolding, explanation, inspection, and validation,
- and a machine-readable contract surface for agentic use.

## What New Alphabet Is Not

New Alphabet is not:

- a bag of optional components,
- a generic design-token package,
- a broad theming engine,
- a purely stylistic homage to mid-century modernism,
- a "tasteful SaaS starter" with no doctrine,
- or an excuse to let agents improvise without constraints.

## Anti-Vibe-Coded Position

New Alphabet rejects:

- layout invention from scratch on every page,
- stylistic drift caused by prompt-to-prompt variability,
- component-library default aesthetics masquerading as product identity,
- endless local taste decisions,
- and code that is visually or structurally disconnected from the design constitution.

It favors:

- stable grammar over improvisation,
- system memory over local preference,
- composition over decoration,
- finite APIs over open-ended styling,
- and a clear family resemblance across many outputs.

## Why Wim Crouwel Matters Here

The project borrows from Wim Crouwel's method more than from his visual surface.

The relevant lessons are:

1. Order before ornament.
2. The grid is operating law, not decoration.
3. Information outranks personality.
4. Constraint produces stronger work than arbitrary choice.
5. Systems scale better than isolated compositions.
6. The medium's constraints are part of the design.

New Alphabet should inherit Crouwel's rigor, proportion, discipline, and logic. It should not collapse into retro poster imitation.

## Aesthetic Position

The framework should feel:

- modern,
- severe,
- typographic,
- structured,
- calm,
- exact,
- and recognizably itself.

It should not feel:

- nostalgic,
- ornamental,
- cute,
- trend-chasing,
- or interchangeable with generic contemporary SaaS tooling.

### Working aesthetic test

If a screen could be mistaken for a standard component-library demo with a few custom tokens, the system is being applied too loosely.

## Core Philosophy

The PRD defines the following constitutional principles:

| Principle | Meaning |
| --- | --- |
| Order before ornament | Structure clarifies meaning before style appears |
| Grid as operating law | Layout, rhythm, spacing, and hierarchy come from the grid |
| Information over personality | Relationships are more important than visual self-expression |
| Constraint generates freedom | Reduced options produce stronger composition |
| Systems over pages | The real artifact is a reusable language, not a single beautiful screen |
| Medium truth | Loading, responsiveness, accessibility, and maintainability are part of design |
| Agents need rules, not vibes | The system must be explicit enough to be loaded and enforced |

### What these principles mean in practice

- Every serious page starts with layout law.
- Every visual deviation should be justified by the system, not by taste.
- Every component should express a role, not just a shape.
- Every recipe should feel like a member of a larger family.
- Every agent interaction should expose the real rules rather than simulate tastefulness.

## Who This Is For

### Primary personas

| Persona | Need | Why New Alphabet fits |
| --- | --- | --- |
| Solo Rust Builder | A strong system without designing from scratch | It gives immediate discipline, examples, and scaffolds |
| Product Engineer | Predictable APIs and low design drift | It uses explicit composition and narrow interfaces |
| AI-Assisted Builder | Installable design context and validation | It turns doctrine into schemas, recipes, and rules |

### Secondary persona

| Persona | Need | Why it matters |
| --- | --- | --- |
| Design-Minded OSS Contributor | Clear philosophy and contribution rules | The system should feel like a real school, not a loose project |

## Use Cases

### Editorial

- personal or studio blog,
- documentation site,
- article and essay archive,
- knowledge publication site.

### SaaS and workflow

- admin console,
- review queue workspace,
- search and filter-heavy back-office tools,
- settings and account surfaces,
- analytics and operations dashboards.

### Agentic

- attach New Alphabet to a coding session,
- scaffold a product from sparse or specific intent,
- validate whether generated code conforms to the system,
- explain which recipe or primitive should be used,
- transform natural-language product descriptions into framework-native implementation tasks.

### Strong-fit interpretation

New Alphabet is a strong fit when the product needs:

- coherence across many screens,
- dense information design,
- a durable visual law,
- controlled adaptability,
- or AI-assisted implementation with low drift.

It is a weak fit when the product mainly needs:

- brand-theater marketing surfaces,
- unlimited theming,
- novelty-driven styling,
- or constant per-page visual reinvention.

## Product Shape

New Alphabet is designed as three surfaces shipped together.

| Surface | Purpose | Why it exists |
| --- | --- | --- |
| Framework | Real Rust and Leptos implementation | The doctrine must exist in code |
| CLI | Human and agent entry point | The doctrine must be easy to apply |
| Agent contract | Machine-readable schemas and rules | The doctrine must be loadable and enforceable |

### Important hierarchy

- The framework and doctrine are primary.
- The CLI operationalizes the doctrine.
- The agent contract exposes the doctrine to coding sessions.

The product fails if this order is reversed.

## Installation Story

### Human path

1. Create a new Leptos app or add New Alphabet to an existing project.
2. Use the CLI to scaffold recipes or components.
3. Use docs and examples to remain inside the grammar.
4. Use validation to detect drift early.

### LLM path

1. Load the New Alphabet repo or exported context bundle.
2. Inspect foundations, primitives, components, recipes, anti-patterns, and examples.
3. Map user intent onto existing structures.
4. Generate or repair code inside those constraints.
5. Validate and explain the output in framework terms.

## Technical Stack

| Concern | Choice |
| --- | --- |
| Language | Rust |
| Web framework | Leptos |
| Rendering | SSR, hydration, CSR where appropriate |
| Styling strategy | framework-controlled tokens and class generation with strict composition rules |
| Schema format | JSON, with mirrored or generated Rust types where useful |
| Documentation site | built with New Alphabet where possible |

### Why Rust

Rust supports the same values the design system claims:

- explicit structure,
- strict boundaries,
- low ambiguity,
- disciplined composition,
- and correctness under growth.

### Why Leptos

Leptos fits the thesis because it supports:

- full-stack Rust,
- SSR and hydration,
- shared logic between client and server,
- and a component model that can express the layered system directly.

## Layer Model

New Alphabet is structured in five layers.

| Layer | Role | Owns | Does not own |
| --- | --- | --- | --- |
| Foundation | Constitutional design rules | tokens, scales, grid, density, color roles, motion, borders, state rules | semantic meaning and page-specific composition |
| Structural primitives | Screen geometry | shells, rails, regions, stacks, rows, panels, bands | domain-specific UI or product logic |
| Semantic components | Meaning-bearing UI | buttons, fields, tables, alerts, detail panes, badges | full-screen layout law |
| Recipes | Approved page compositions | blog shells, review workspaces, settings surfaces | low-level tokens or arbitrary styling |
| Agent contract surface | Formal system exposure | schemas, maps, rules, anti-patterns, validation logic | hidden or fake rules that diverge from implementation |

### How the layers should behave

- Foundations should be stable and few.
- Primitives should feel structural rather than decorative.
- Components should encode meaning with narrow APIs.
- Recipes should prove that the grammar scales.
- The contract surface should mirror the real system exactly.

### How a new feature should be built

Working interpretation:

1. Start from recipe fit, not visual improvisation.
2. If no recipe fits, compose from primitives.
3. Use semantic components rather than inventing new visual units.
4. Define state behavior before polish.
5. Validate the composition against spacing, accessibility, and anti-pattern rules.

## Primitive Composition Maps

Primitive composition is finite and queryable. The canonical machine-readable map lives at `examples/primitive-composition.json`.

### Allowed core combinations

| Rule | Composition | Why it is allowed |
| --- | --- | --- |
| `P-001` | `AppShell -> PageGrid -> Region(main)` | every serious screen starts with a named primary region |
| `P-002` | `PageGrid -> Rail -> Region(main)` | side structure may support the primary region without replacing it |
| `P-003` | `Region(main) -> Stack -> ColumnGroup` | editorial subdivision stays inside the primary region |
| `P-004` | `Region(main) -> Stack -> Row` | workflow alignment stays local and token-driven |
| `P-005` | `Panel -> SectionHeader -> Divider -> content` | bounded surfaces remain explicit and quiet |
| `P-006` | `Band -> SectionHeader` | full-width framing surfaces stay structural and typographic |

### Disallowed core combinations

| Rule | Composition | Why it is disallowed |
| --- | --- | --- |
| `P-101` | `PageGrid` without `Region(main)` | the grid becomes decorative scaffolding |
| `P-102` | multiple rails on one side | side structure stops being bounded |
| `P-103` | `Rail` inside `ColumnGroup` | page structure is being collapsed into local layout |
| `P-104` | arbitrary spacing wrappers between `Stack` or `Row` children | rhythm is being repaired locally instead of governed by tokens |
| `P-105` | `Panel` using color or motion as its only hierarchy signal | structure is being replaced by decoration |

### Primitive example set

The current example components are:

- `EditorialAnchorExample`
- `WorkspaceAnchorExample`
- `EditorialStructureExample`
- `WorkflowStructureExample`
- `EditorialSurfaceExample`
- `SettingsSurfaceExample`

These examples are intended to be reused by later docs, validation rules, and the agent contract rather than rewritten in parallel.

## Monorepo Shape

The PRD proposes the following layout.

| Path | Purpose |
| --- | --- |
| `crates/new-alphabet-core` | shared types, tokens, schema models, low-level utilities |
| `crates/new-alphabet-foundation` | grid, spacing, type, density, color roles, motion, borders |
| `crates/new-alphabet-primitives` | structural primitives implemented in Leptos |
| `crates/new-alphabet-components` | semantic components built on primitives |
| `crates/new-alphabet-recipes` | opinionated page and workflow recipes |
| `crates/new-alphabet-schema` | canonical JSON schemas and validation models |
| `crates/new-alphabet-cli` | `init`, `add`, `inspect`, `explain`, `validate`, `export`, and later `plan` |
| `crates/new-alphabet-agent` | agent-facing helpers and future protocol integration |
| `apps/docs` | documentation site and manifesto |
| `apps/demo-blog` | editorial reference implementation |
| `apps/demo-saas` | SaaS reference implementation |
| `examples` | focused examples and integration paths |
| `schemas` | published machine-readable contracts |
| `prompts` | session bootstrap and prompt examples |
| `scripts` | release, export, validation, and contributor automation |

### Working repo rule

Each crate should own one layer or one cross-cutting concern clearly. If responsibilities blur, the system will get harder to explain and harder for agents to load correctly.

## Naming Law

Names in New Alphabet are constitutional labels, not brand theater.

### Foundation naming

- Use plain names for rules, scales, and roles.
- Name by structural responsibility rather than visual flavor.
- Do not invent branded token families.
- Do not name tokens after palettes, moods, or decorative metaphors.

### Primitive naming

- Use structural names such as `PageGrid`, `Region`, `Rail`, `Stack`, or `Panel`.
- Name a primitive by the geometry or containment rule it owns.
- Do not name primitives after a finished screen, business domain, or visual style.

### Component naming

- Use semantic names such as `Button`, `StatusBadge`, `InlineAlert`, or `DetailPane`.
- Name by product role and interaction meaning rather than visual treatment.
- Do not encode color, ornament, or trend language into component names.

### Recipe naming

- Use product-surface names such as `ArticleShell`, `BlogIndex`, `ReviewQueue`, or `SettingsWorkspace`.
- A recipe name must imply intent and usage context clearly.
- Do not name recipes after atmosphere, palette, or campaign language.

### Naming tests

- If a name sounds decorative, it is probably wrong.
- If a name could apply to many unrelated responsibilities, it is too vague.
- If a name hides structure, state, or intent, it weakens the doctrine.

## Foundation Token Taxonomy

Canonical token identifiers use lowercase dot-separated segments.

- The first segment is the family.
- Middle segments describe structural role, relation, or mode.
- Final segments may define state, emphasis, or density when needed.
- Token names never encode palette marketing, mood language, or ornamental aliases.

### Token families

| Family | Owns | Example identifiers |
| --- | --- | --- |
| `layout` | page geometry and region behavior | `layout.page.columns`, `layout.region.main.span`, `layout.rail.default.width` |
| `spacing` | rhythm and separation | `spacing.stack.tight`, `spacing.region.default`, `spacing.panel.content` |
| `type` | typographic hierarchy | `type.display.large`, `type.body.default`, `type.annotation.strong` |
| `density` | calm, regular, and dense operating modes | `density.calm`, `density.regular`, `density.dense` |
| `color` | semantic surface, text, border, accent, and status roles | `color.canvas.base`, `color.text.primary`, `color.status.error.fg` |
| `border` | structural separation and emphasis | `border.region.default`, `border.control.focus`, `border.panel.strong` |
| `motion` | state change and region transition behavior | `motion.transition.fast`, `motion.region.enter`, `motion.reduced.none` |
| `state` | shared loading, focus, success, error, and disabled treatment | `state.focus.ring`, `state.loading.muted`, `state.disabled.alpha` |

### Taxonomy rules

- A token name must answer what role it serves before how it looks.
- Families stay finite and constitutional.
- New tokens extend an existing family before creating a new one.
- If two tokens differ only by decoration, one of them is probably invalid.

### Rust and schema reading

- Rust implementations should mirror these families with plain enums, structs, or constants.
- JSON schema and export surfaces should preserve the same segment order.
- Human docs, code, and exported context must describe the same token families with the same names.

## Grid And Responsive Region System

This grid spec turns the doctrine into fixed page geometry for `PageGrid`, `Region`, and `Rail`.

### Breakpoints

| Breakpoint | Columns | Role |
| --- | --- | --- |
| `compact` | `4` | single reading or workflow flow |
| `medium` | `8` | main region plus one secondary structure |
| `wide` | `12` | full editorial or workflow composition |

### Region spans

| Region class | `compact` | `medium` | `wide` |
| --- | --- | --- | --- |
| `full` | `4` | `8` | `12` |
| `main` | `4` | `6` | `8` |
| `support` | `4` | `2` | `4` |
| `rail` | stacked full width | `2` | `2` or `3` |
| `detail` | stacked full width | `8` or below `main` | `3` or `4` |
| `action_band` | `4` | `8` | `12` |

### Rail widths

| Rail width token | `medium` | `wide` |
| --- | --- | --- |
| `layout.rail.narrow.width` | `2` columns | `2` columns |
| `layout.rail.default.width` | `2` columns | `3` columns |
| `layout.rail.broad.width` | `3` columns | `4` columns |

### Responsive rules

- `compact` permits one major vertical flow only.
- On `compact`, rails, detail panes, and support regions collapse into stacked full-width regions.
- `medium` permits `main` plus one adjacent `rail` or one adjacent `support` region.
- On `medium`, a second side structure must collapse below `main`.
- `wide` permits `rail + main`, `main + detail`, or `rail + main + detail` when a recipe explicitly allows it.
- `action_band` remains full span at every breakpoint.
- Region order may change only through named breakpoint rules, never through local CSS improvisation.

### Hard forbiddances

- Free-positioned major regions are forbidden at V0.
- Major layout may not be defined outside named grid spans.
- Floating utility sidecars, arbitrary offsets, and per-page breakpoint exceptions are forbidden.
- If a surface needs a new major region pattern, the recipe or primitive contract must be expanded first.

## Type Scale And Density Modes

One typographic grammar serves both editorial reading and dense workflow surfaces.

### Type roles

| Role family | Canonical tokens | Use |
| --- | --- | --- |
| `display` | `type.display.large`, `type.display.default` | page openers, manifesto lines, major surface titles |
| `heading` | `type.heading.1`, `type.heading.2`, `type.heading.3` | section hierarchy and structural breaks |
| `body` | `type.body.reading`, `type.body.default`, `type.body.compact` | longform prose, default interface copy, compact support copy |
| `annotation` | `type.annotation.default`, `type.annotation.strong` | metadata, labels, helper text, timestamps |
| `data` | `type.data.label`, `type.data.cell`, `type.data.metric` | tabular labels, dense cells, metrics, and operational values |

### Type rules

- `display` is rare and structural, never decorative.
- `body.reading` owns longform editorial flow.
- `body.default` is the base interface voice for forms, settings, and workflow surfaces.
- `body.compact` and `data.*` support dense operational surfaces without inventing a separate visual family.
- `annotation` clarifies secondary information and must remain legible at every density.

### Density modes

| Mode | Spacing behavior | Type behavior | Table behavior |
| --- | --- | --- | --- |
| `density.calm` | widest rhythm in the token scale | prefers `body.reading` and larger headings | wrap before truncating; favors comprehension over scan speed |
| `density.regular` | baseline rhythm for most surfaces | defaults to `body.default` and standard headings | balanced wrapping and truncation |
| `density.dense` | compressed rhythm with no arbitrary exceptions | prefers `body.compact` and `data.*` roles | explicit truncation, tighter rows, stronger alignment discipline |

### Density rules

- Density is chosen by recipe, page region, or major component family, not by local whim.
- Editorial reading surfaces default to `density.calm` or `density.regular`.
- Tables, queues, and detail-heavy workflows may use `density.dense`.
- Changing density must alter spacing, type choice, and table rhythm together.
- Density modes remain finite; per-screen density invention is forbidden.

## Color Roles, Borders, Motion, And State Tokens

Color, separation, and motion are structural signals. They do not replace hierarchy.

### Color roles

| Role family | Canonical tokens | Use |
| --- | --- | --- |
| `canvas` | `color.canvas.base`, `color.canvas.elevated`, `color.canvas.inset` | page fields, panels, inset surfaces |
| `text` | `color.text.primary`, `color.text.secondary`, `color.text.inverse` | reading flow, support copy, inverse surfaces |
| `accent` | `color.accent.primary`, `color.accent.muted` | action emphasis and navigational orientation |
| `border` | `color.border.default`, `color.border.strong`, `color.border.focus` | region separation, panel edges, focus emphasis |
| `status` | `color.status.info`, `color.status.success`, `color.status.warning`, `color.status.error` | contextual feedback and workflow state |
| `emphasis` | `color.emphasis.subtle`, `color.emphasis.strong` | restrained hierarchy support where type and spacing are already doing the main work |

### Color rules

- Color roles are semantic and finite.
- Color must never be the sole carrier of meaning.
- Status surfaces must combine label, structure, or iconography with color.
- Accent use is restrained; it should orient action, not decorate the page.

### Border rules

| Token | Use |
| --- | --- |
| `border.region.default` | separate major page regions |
| `border.panel.default` | define bounded surfaces and content groups |
| `border.panel.strong` | mark stronger hierarchy transitions without ornamental chrome |
| `border.control.default` | input and control boundaries |
| `border.control.focus` | focus-visible emphasis when interaction state changes |

- Border weight is finite and hierarchical.
- Borders separate regions or controls; they do not decorate empty space.

### Motion rules

| Token | Use |
| --- | --- |
| `motion.transition.fast` | small state changes |
| `motion.transition.default` | standard control or panel transitions |
| `motion.region.enter` | region reveal when structure changes |
| `motion.region.exit` | region removal when structure changes |
| `motion.reduced.none` | reduced-motion fallback for nonessential movement |

- Motion exists to clarify change, not to add atmosphere.
- Decorative loops and attention-seeking movement are forbidden.
- Reduced-motion mode removes nonessential movement and keeps meaning through layout, opacity, or direct state change.

### State tokens

| Token | Use |
| --- | --- |
| `state.focus.ring` | focus-visible emphasis |
| `state.loading.muted` | low-noise loading treatment |
| `state.success.emphasis` | success confirmation without celebratory excess |
| `state.error.emphasis` | error emphasis tied to explicit messaging |
| `state.disabled.alpha` | disabled-state reduction with preserved legibility |

- Focus-visible emphasis is mandatory for interactive elements.
- Loading, success, error, and disabled treatment must remain legible in calm and dense modes.
- State tokens are shared across components so unfinished states do not invent their own visual language.

## Design Rules

### Layout law

- Serious pages are composed from primitives before semantic embellishment.
- Arbitrary spacing values are forbidden outside the token scale.
- Components must align to the grid and declared density mode.
- Recipes may constrain which region combinations are valid.
- Layout should clarify meaning before expressive styling appears.

### API discipline

- Component props must stay narrow.
- Variants must be finite and explicit.
- Invalid combinations should be impossible or loudly rejected.
- Slot names should be stable and semantically clear.
- Boolean-prop sprawl should be treated as a smell.

### Typographic law

- Hierarchy should come from type, spacing, and alignment first.
- Color and ornament are secondary signals.
- Editorial and SaaS surfaces should share one typographic grammar.
- Typography should do structural work, not decorative work.

### State law

- Serious components must define `default`, `loading`, `empty`, `error`, and `success` where relevant.
- Interactive components must define `hover`, `focus_visible`, `active`, and `disabled`.
- Dense surfaces must make truncation, wrapping, and overflow behavior explicit.

### Accessibility law

- Use semantic HTML where possible.
- Keyboard navigation is first-class.
- Accessible names and descriptions are required.
- Color must never be the sole carrier of meaning.
- Focus order and focus visibility are system-level concerns, not afterthoughts.

## Anti-Patterns

The following patterns are explicitly outside the New Alphabet doctrine.

- Ad hoc layout rules outside named grid behavior.
- Arbitrary spacing values used to repair local compositions.
- Decorative wrappers with no structural responsibility.
- Components named after style, palette, or mood.
- Open-ended variant and boolean-prop sprawl.
- Prompt-to-prompt layout invention without recipe or primitive fit.
- Component-library default aesthetics treated as product identity.
- Per-screen exceptions that weaken family resemblance.
- Color or ornament doing work that hierarchy, type, or structure should do.
- AI-generated output that simulates taste instead of enforcing law.

### Anti-pattern reading

When one of these appears, the first repair move is to delete or narrow the invalid structure before adding anything new.

## State Model

### Required global states

- `default`
- `loading`
- `empty`
- `error`
- `success`

### Interactive states

- `hover`
- `focus_visible`
- `active`
- `disabled`

### Dense-data rules

- Tables and lists must define truncation and wrapping rules.
- Detail panes must define loading and unavailable states.
- Filter structures must define zero-result states.

### Why the state model matters

The system is not only a composition language for finished screens. It must also govern unfinished, loading, failing, empty, and disabled states. If those are undefined, the design constitution is incomplete.

## Initial Inventory

### Foundations

- `grid`
- `spacing_scale`
- `type_scale`
- `density_modes`
- `color_roles`
- `border_rules`
- `motion_rules`
- `breakpoints`
- `z_index_and_layering_rules`
- `state_tokens`

### Structural primitives

- `AppShell`
- `PageGrid`
- `Region`
- `Rail`
- `Stack`
- `Row`
- `ColumnGroup`
- `Panel`
- `Band`
- `SectionHeader`
- `Divider`

### Semantic components

- `Button`
- `LinkAction`
- `TextField`
- `Textarea`
- `Select`
- `Checkbox`
- `RadioGroup`
- `Switch`
- `StatusBadge`
- `InlineAlert`
- `EmptyState`
- `Table`
- `MetricBlock`
- `NavIndex`
- `CommandBar`
- `FilterRail`
- `DetailPane`
- `Pagination`

### Recipes

- `BlogIndex`
- `ArticleShell`
- `DocsShell`
- `DashboardShell`
- `SearchResultsWorkspace`
- `ReviewQueue`
- `SettingsWorkspace`

### Inventory reading

This inventory is intentionally limited. The goal is not completeness by count. The goal is a minimum set that can prove the grammar across both editorial and SaaS surfaces.

## CLI Surface

The CLI name is `new-alphabet`.

### Initial commands

| Command | Purpose |
| --- | --- |
| `new-alphabet init` | Create a new project or add New Alphabet to an existing Leptos project |
| `new-alphabet add recipe <name>` | Scaffold a recipe such as `blog-index`, `article-shell`, or `review-queue` |
| `new-alphabet add component <name>` | Add a semantic component with required states and foundation bindings |
| `new-alphabet plan <intent>` | Turn natural language into a framework-native plan |
| `new-alphabet explain <item>` | Explain a primitive, component, recipe, or rule in framework terms |
| `new-alphabet inspect <path>` | Inspect project usage and identify which primitives, components, and recipes are present |
| `new-alphabet validate` | Validate schema, composition, state, spacing, and accessibility rules |
| `new-alphabet export context` | Generate an agent-readable bundle for a coding session |

### Working interpretation of command behavior

- `init` should create deterministic file changes and clear project wiring.
- `add` should scaffold approved structures, not arbitrary code.
- `plan` should emit a structured implementation plan, not design prose.
- `explain` should cite the system's rules and composition logic.
- `inspect` should map concrete code back to framework concepts.
- `validate` should report both violations and suggested repairs.
- `export context` should produce a versioned machine-readable bundle.

None of these commands should become an escape hatch from the constitution. They should make the doctrine easier to apply, not easier to bypass.

### Future agent-facing tools

The PRD names a future tool surface that may include:

- `get_foundation_rules`
- `list_primitives`
- `list_components`
- `list_recipes`
- `plan_from_intent`
- `validate_plan`
- `generate_scaffold`

These should only ship once they expose the real system faithfully.

## Agent Model

The PRD's agent model is simple: New Alphabet should allow a model to work from either sparse or highly specific prompts because the framework carries the formal language underneath.

### Supported guidance levels

| Level | Example |
| --- | --- |
| Sparse | "Build me a blog." |
| Moderate | "Build me a content-heavy SaaS for reviewing submissions." |
| High | "Build a dense review workspace with left navigation rail, center results table, right detail pane, persistent action strip, loading and dirty states, and a calm editorial tone." |

### Required session context

- foundations
- primitives
- components
- recipes
- composition rules
- state contracts
- anti-patterns
- examples

### Ideal agent outcomes

- choose an appropriate recipe from user intent,
- scaffold valid Leptos code from framework rules,
- explain why a chosen composition is valid,
- validate or repair drift from the constitution.

### Working interpretation of the agent contract

The PRD names the contract surface but does not yet define its exact file format. A practical V0 interpretation is:

- versioned JSON schema files for each layer,
- recipe and component examples bound to those schemas,
- named validation rules with stable identifiers,
- anti-pattern definitions that tooling can report against,
- and a bundle format that a coding agent can load without reading the whole repo.

The key rule is this: agentic coding is a downstream advantage of explicit doctrine. The doctrine should not be weakened to make generation easier.

## Agent Roles

The PRD defines a set of working roles that clarify how the project should be thought about.

| Role | Responsibility |
| --- | --- |
| Constitution Architect | define philosophy, naming, grid, spacing, type, density, color roles, and anti-patterns |
| Foundation Implementer | implement tokens and low-level rules in Rust |
| Primitive Builder | build structural primitives and constrain layout composition |
| Semantic Component Builder | build semantic components with explicit states and accessibility support |
| Recipe Composer | create full-screen compositions for blog and SaaS use cases |
| Contract and CLI Builder | implement schema export, validation, and CLI commands |
| Quality Critic | run design, API, accessibility, documentation, and state completeness gates |
| OSS Steward | prepare public docs, roadmap, releases, and contribution materials |

### Why these roles matter

They are less about org structure than about project completeness. New Alphabet is not done when it has code. It is done when the doctrine, runtime, demos, CLI, contract surface, and public release posture all cohere.

## Example User Prompts

The PRD includes example prompts that should work once the system is real:

- "Build me a blog for longform essays and notes using New Alphabet."
- "Build me a B2B admin workspace for reviewing submissions with a left rail, dense table, and right-side detail pane."
- "Use New Alphabet to scaffold a settings workspace with section navigation and editable panels."
- "Explain which recipe I should use for a documentation site with article pages and archive navigation."
- "Validate whether this page structure conforms to New Alphabet's grid and state rules."

These examples matter because they test three things at once:

- whether the language is installable,
- whether recipe selection is intelligible,
- and whether validation can operate on real structures rather than loose taste.

## V0 Scope

### Must ship

- foundation layer
- primitive layout layer
- core semantic components
- demo blog
- demo SaaS workspace
- CLI `init`, `add`, `explain`, `validate`, and `export`
- canonical schema bundle
- docs site
- public repo

### Should ship

- `plan` command from natural-language intent
- anti-pattern validator
- recipe generator from constrained intents

### Later

- protocol-native MCP adapter
- interactive recipe browser
- multiple theme families
- cross-framework renderers
- autonomous multi-agent orchestration

### Reading the scope correctly

V0 should be judged by sharpness, not by breadth. If the system becomes broad before it becomes exact, it will lose the very discipline that differentiates it.

## Milestones

| Milestone | Name | Main outcomes |
| --- | --- | --- |
| `M0` | Constitution Freeze | philosophy, naming, grid, type, density, color-role rules, and anti-patterns are fixed for V0 |
| `M1` | Foundation Runtime | foundation crates compile, primitives render, and SSR and hydration baseline works |
| `M2` | Core Component Set | minimum semantic component inventory ships with state and accessibility coverage |
| `M3` | Dual Demonstrator Proof | demo blog and demo SaaS prove one grammar across two product shapes |
| `M4` | CLI and Context Export | CLI commands work and an agent-readable context bundle can be generated |
| `M5` | Public OSS Launch | repo is public, docs are coherent, release process exists, and V0 tag is published |

### Why the milestones are sequenced this way

The order forces the project to earn its abstractions:

- first law,
- then runtime,
- then semantic inventory,
- then proofs,
- then tooling,
- then public release.

## Workstreams

| Workstream | Objective | Deliverables |
| --- | --- | --- |
| `WS1` Constitution and Naming | define the visual and conceptual laws | manifesto, naming rules, grid spec, spacing scale, typography scale, density model, color roles, anti-pattern registry |
| `WS2` Foundations and Primitives | implement the base grammar in Rust and Leptos | foundation crates, primitive layout layer, SSR-safe baseline, primitive examples |
| `WS3` Semantic Component Layer | build the smallest useful semantic inventory | inputs, status elements, data display components, state coverage, accessibility coverage |
| `WS4` Recipes and Demonstrators | prove one grammar across editorial and SaaS | demo blog, demo SaaS, recipe docs, example outputs |
| `WS5` CLI and Agent Contract Surface | make the system installable and queryable | CLI, schema export, validation engine, context bundle format, prompt examples |
| `WS6` Documentation and OSS Release | ship a serious public offering | README, docs site, contributing guide, roadmap, release notes template, issue templates, license selection |

### Working interpretation of the workstreams

These workstreams are not parallel abstractions. They are the minimum set of bodies of work required for New Alphabet to read as a coherent public system rather than an internal experiment.

## Seed Backlog

The PRD seeds the following initial backlog:

| Backlog item | Layer | Meaning |
| --- | --- | --- |
| `B1` Write constitution manifesto | foundation | define the doctrine in plain language |
| `B2` Implement token and grid system | foundation | make the law concrete |
| `B3` Implement `AppShell`, `PageGrid`, `Region`, `Rail`, `Stack`, `Row`, `ColumnGroup`, `Panel` | primitives | make screen geometry real |
| `B4` Implement `Button`, `TextField`, `Select`, `StatusBadge`, `InlineAlert`, `EmptyState`, `Table` | components | establish the minimum semantic layer |
| `B5` Build `BlogIndex` and `ArticleShell` recipes | recipes | prove editorial coherence |
| `B6` Build `SearchResultsWorkspace` and `SettingsWorkspace` recipes | recipes | prove SaaS coherence |
| `B7` Implement schema bundle and validation engine | agent contract | make the doctrine enforceable |
| `B8` Implement CLI `init`, `add`, `explain`, `validate`, `export` | CLI | make the system installable |
| `B9` Build docs site using New Alphabet | docs | make the system public in its own language |
| `B10` Prepare OSS launch materials | release | complete the public offering |

### Suggested execution reading

The backlog implies a natural order:

1. constitution,
2. foundation runtime,
3. primitives,
4. semantic minimum,
5. editorial proof,
6. SaaS proof,
7. validation and contract,
8. CLI,
9. docs site,
10. launch materials.

That order matches the thesis: the system should become real before it becomes widely surfaced.

## Decision Log

The PRD already records several decisive choices.

| Decision | Reason |
| --- | --- |
| Use a monorepo | framework, CLI, schemas, demos, docs, and agent surface must evolve together |
| Use Rust and Leptos first | preserve one-language coherence while targeting the web seriously |
| Support both blog and SaaS at V0 | prove that one grammar can power both content and software |
| Include CLI and context export at V0 | installability into human and agent workflows is core to the idea |

### Why the decision log matters

These decisions should be treated as constraints, not historical notes. Any future proposal that weakens them should be understood as a thesis-level change rather than a small implementation preference.

## Quality Requirements

### Performance

- no unnecessary runtime styling churn,
- reasonable SSR performance for demo surfaces,
- modest hydration cost for editorial surfaces.

### Reliability

- strict typing for contracts and component APIs,
- schema validation integrated into development workflow,
- deterministic examples and scaffold outputs where feasible.

### Security

- CLI must not execute arbitrary untrusted instructions by default,
- scaffold and export commands must be explicit about file writes,
- no network dependency for basic framework use.

### Maintainability

- layer boundaries remain clear,
- schema and runtime stay in parity,
- recipes do not bypass foundations and primitives.

### Working quality reading

Quality is not separate from doctrine. A design constitution that only works in happy-path screenshots is not real.

## Testing Strategy

### Unit tests

- foundation token and rule tests,
- schema serialization and validation tests,
- component prop and state tests.

### Integration tests

- recipe composition tests,
- SSR and hydration tests,
- CLI command tests.

### Snapshot or visual tests

- reference output tests for recipes,
- visual regression for key examples.

### Conformance tests

- schema parity checks,
- accessibility checks,
- spacing checks,
- anti-pattern checks.

### Testing philosophy

Tests should prove not only that the code runs, but that the doctrine survives contact with real implementation.

## Documentation Requirements

The PRD says the public documentation must cover:

- what New Alphabet is,
- what it is not,
- core philosophy,
- foundation rules,
- primitives,
- semantic components,
- recipes,
- CLI usage,
- agent-session usage,
- contributing,
- roadmap,
- and release notes.

### Documentation style

Documentation should be:

- exact and restrained,
- example-driven,
- low jargon,
- systemic rather than promotional.

### What that means

The writing should sound like a constitution and a reference, not a marketing page. It should clarify law, boundaries, usage, and rationale with as little inflation as possible.

## Acceptance Gates

New Alphabet is only done when it passes all of these gates.

### Design gate

- every primitive aligns to the declared grid and spacing rules,
- typography and hierarchy remain coherent across editorial and SaaS examples,
- recipes visibly read as one family.

### Aesthetic gate

- surfaces should be recognizably New Alphabet,
- the system should not collapse into generic contemporary SaaS styling,
- Crouwelian logic should be visible even when the surface is modernized.

### API gate

- component APIs are narrow and documented,
- variants are finite and explicit,
- unsupported prop combinations are blocked or rejected.

### State gate

- required states are implemented and documented,
- no serious production component ships with undefined empty, error, or loading behavior where relevant.

### Accessibility gate

- keyboard use is verified,
- focus-visible behavior exists,
- semantics and accessible naming are validated.

### Platform gate

- SSR works,
- hydration works,
- responsive behavior is defined for required recipes.

### Agent gate

- schemas exist for foundations, primitives, core components, and recipes,
- CLI can export a context bundle,
- at least three prompt-to-plan examples are demonstrated.

### Docs gate

- README explains philosophy and quick start clearly,
- docs site covers foundations, primitives, components, recipes, CLI, and agent workflows,
- examples are runnable and mapped to docs.

### OSS gate

- license is selected,
- contribution guidance exists,
- issue and PR templates exist,
- release notes discipline is established.

## Definition of Done

### Project done when

- a developer can create or add New Alphabet to a Leptos project through the CLI,
- a developer or LLM can scaffold at least one blog surface and one SaaS surface,
- foundations, primitives, core components, and recipes are documented and schema-backed,
- validation can detect meaningful drift from the framework's rules,
- the repository reads as a coherent public offering with docs, examples, roadmap, and release structure.

### V0 done when

- milestones `M0` through `M5` are complete,
- acceptance gates pass,
- the demo blog and demo SaaS clearly prove the thesis,
- agent context export works and is documented.

### Working reading of done

New Alphabet is not done when the code merely exists. It is done when the runtime, the doctrine, the demos, the contract surface, and the public documentation all tell the same story.

## Risks

| Risk | Mitigation |
| --- | --- |
| Over-abstraction | keep V0 narrow and implementation-led |
| Over-expansion | treat component count as secondary to grammar quality |
| Pastiche | test for Crouwel's logic rather than only his aesthetic aura |
| Adoption friction | provide scaffolds, examples, and clear docs |
| Agent fantasy without executable rules | ship schemas and export early, defer protocol complexity |

### Risk reading

The most dangerous failure would be to become a generic framework that still speaks in strict language. The system must stay narrow enough that its doctrine remains visible.

## Areas That Still Need Formal Specification

The PRD is strong on intent and structure, but several areas still need concrete, versioned spec documents.

### 1. Foundation spec

Needed next:

- token names and values,
- grid math,
- spacing scale,
- type scale,
- density modes,
- color-role definitions,
- motion rules,
- border rules.

### 2. Primitive API spec

Needed next:

- exact props,
- slot definitions,
- allowed compositions,
- responsive rules,
- invalid combinations.

### 3. Component state matrices

Needed next:

- required states by component,
- keyboard behavior,
- accessibility expectations,
- loading and empty-state behavior.

### 4. Recipe contract

Needed next:

- recipe intent,
- required regions,
- optional regions,
- adaptation limits,
- example compositions.

### 5. CLI contract

Needed next:

- argument shapes,
- output formats,
- exit codes,
- file-write guarantees,
- validation severity model.

### 6. Agent context format

Needed next:

- bundle schema,
- versioning strategy,
- minimum required files,
- schema identifiers,
- example session bootstrap format.

### 7. Roadmap and success metrics

Needed next:

- adoption signals,
- demo completion criteria,
- validation coverage targets,
- documentation coverage targets,
- release readiness metrics.

## Recommended Next Documentation Set

To make the repo legible quickly, the next docs should be:

1. `README.md` for quick start and thesis.
2. `docs/foundations.md` for constitutional design rules.
3. `docs/primitives.md` for the layout grammar.
4. `docs/components.md` for semantic APIs and state matrices.
5. `docs/recipes.md` for full-screen compositions.
6. `docs/cli.md` for command contracts.
7. `docs/agent-contract.md` for schema and export format.
8. `docs/contributing.md` for contribution posture and doctrine-preserving change rules.

## Short Thesis

New Alphabet is a strict, grid-governed, typographic design framework for Rust and Leptos. It exists to replace vibe-coded websites and web apps with a coherent, adaptable, constrained modern alternative and to make that alternative explicit enough that both humans and coding agents can build inside it without drift.
