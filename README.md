# New Alphabet

New Alphabet is a Crouwelian design constitution for the web.

It is a Rust-first, Leptos-native framework project for building severe, typographic, grid-governed products without collapsing into generic SaaS defaults or prompt drift. The doctrine is primary. The framework, CLI, and agent contract are downstream expressions of that doctrine.

## Status

This repository is in its initial definition phase.

The current contents are the governing documents:

- `documentation.md` defines the human-readable doctrine.
- `prd.json` defines the machine-readable product requirements.
- `AGENTS.md` defines the repository contract for agents working here.
- `progress.txt` is append-only project memory.
- `crates/new-alphabet-foundation` contains the first runtime implementation of the constitutional foundation layer.
- `crates/new-alphabet-primitives` contains the initial structural primitives for page shell and region geometry.
- `crates/new-alphabet-components` contains the first semantic action components built on the primitive layer.
- `examples/primitive-composition.json` contains the current primitive composition rules and reusable example index.

The foundation runtime now exists. The structural primitive layer includes `AppShell`, `PageGrid`, `Region`, `Rail`, `Stack`, `Row`, `ColumnGroup`, `Panel`, `Band`, `SectionHeader`, and `Divider`, with reusable composition maps and SSR-rendered examples. Semantic component work now includes `Button`, `LinkAction`, `TextField`, `Textarea`, `Select`, `Checkbox`, `RadioGroup`, and `Switch`; the rest of the component inventory, recipes, CLI, and validation are still to be implemented.

Current local verification is `cargo test --workspace`.

## Thesis

Modern web products drift because they are built from loose taste, broad component libraries, and prompt-by-prompt improvisation. New Alphabet rejects that posture.

The project is built on a small set of laws:

- order before ornament
- the grid as operating law
- information over personality
- constraint over styling freedom
- systems over pages
- explicit structure over decorative cleverness
- agents need rules, not vibes

## Naming Posture

New Alphabet names by role, structure, and intent.

- foundations use plain rule and scale names
- primitives use structural names such as `PageGrid` and `Rail`
- components use semantic names such as `Button` and `StatusBadge`
- recipes use surface names such as `ArticleShell` and `ReviewQueue`

If a name sounds decorative, branded, or mood-driven, it is outside the doctrine.

Foundation tokens follow the same rule. Their canonical identifiers are lowercase, dot-separated, and role-based, using finite families such as `layout`, `spacing`, `type`, `density`, `color`, `border`, `motion`, and `state`.

The page grid is likewise fixed by law: `compact` uses 4 columns, `medium` uses 8, and `wide` uses 12, with rails and detail regions collapsing by breakpoint rather than through ad hoc layout invention.

Typography is shared across surfaces through explicit `display`, `heading`, `body`, `annotation`, and `data` roles, with finite `calm`, `regular`, and `dense` density modes instead of per-screen taste decisions.

Color, borders, motion, and state are likewise semantic and finite: color cannot carry meaning alone, focus-visible emphasis is required, and reduced-motion behavior is part of the system rather than an afterthought.

## V0 Shape

New Alphabet aims to ship three connected surfaces:

1. A Rust and Leptos framework with foundations, primitives, components, and recipes.
2. A CLI named `new-alphabet` for scaffolding, validation, explanation, and export.
3. A versioned agent contract that lets coding agents work from the real system rather than approximating it.

V0 is intended to prove one grammar across both editorial and workflow-heavy product surfaces.

## What New Alphabet Is Not

New Alphabet is not:

- a broad theming engine
- a bag of interchangeable UI components
- a generic SaaS starter
- a loose visual homage without structural law
- a prompt-friendly wrapper around arbitrary layout invention

Its explicit anti-patterns include arbitrary spacing fixes, decorative wrappers, style-based naming, boolean-prop sprawl, and component-library defaults treated as product identity.

## Repository Direction

The project favors:

- Rust first
- Leptos first
- SSR and hydration first
- JSON schemas and deterministic exports
- narrow APIs
- explicit naming
- finite composition rules

If a solution adds optionality, abstraction, or ornament without strengthening the doctrine, it is likely wrong.

## Near-Term Work

The initial implementation path is:

1. Freeze the doctrine and naming model.
2. Implement the foundation runtime.
3. Implement structural primitives.
4. Implement the core semantic component set.
5. Prove the grammar on editorial and SaaS recipes.
6. Ship the CLI and agent contract surface.

## License Target

The intended license target is `MIT OR Apache-2.0`.
