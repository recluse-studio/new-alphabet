# Workbench Policy

This document defines the default screen-building policy for dense New Alphabet work surfaces.

The target is a calm, compact, moderately styled, highly legible desktop application for real work.

It is not:

- a landing page,
- a concept piece,
- a dribbble shot,
- or a marketing site.

It is not a runtime flavor.
It does not replace the constitution.
It does not override host-specific flavor law.

Use it when:

- the task is a dense desktop work surface,
- the stack is unspecified,
- or the host flavor does not already provide a stricter shell or density rule.

If a runtime flavor conflicts with this document, the constitution stays primary and the flavor may only tighten the rule, never loosen it.

## Design target

The surface should be:

- calm,
- compact,
- moderately styled,
- highly legible,
- information-dense without feeling cramped,
- obvious in hierarchy,
- immediate in access to primary actions,
- readable at a glance.

## Default density module

Before any widgets, create a density token module and use only those values:

- `body_text = 13`
- `meta_text = 12`
- `section_title = 16`
- `page_title = 20`
- `control_height = 32`
- `toolbar_height = 36`
- `sidebar_width = 248`
- `inspector_width = 300`
- `gap_x = 8`
- `gap_y = 6`
- `outer_padding = 8`
- `panel_padding = 8`

These defaults are already close to New Alphabet dense law.
Use them as the shell baseline unless a named flavor requires a tighter host-specific binding.

## Build order

Design the shell before the screen and the screen before the widgets.

Build in this order:

1. app shell
2. navigation rail or sidebar
3. toolbar
4. main work surface
5. optional inspector
6. only then leaf widgets

Do not start by styling cards, charts, fields, or decorative wrappers.

## First viewport law

At `1440x900`, the first viewport must show:

- navigation,
- toolbar,
- primary content,
- primary actions.

The user should see real work immediately.

Keep all primary actions visible without scrolling at `1440x900`.
Do not allow decorative framing to push important work below the fold.

## Preferred patterns

Prefer:

- split panes,
- lists,
- tables,
- compact forms,
- inspectors,
- tabs,
- one-row toolbars,
- list/detail layouts,
- table/detail layouts.

Cards are allowed only when the card itself is the interaction.
Routine data should live in plain surfaces, tables, lists, inspectors, or compact forms.

## Banned patterns

Do not ship:

- hero headers,
- welcome banners,
- oversized cards,
- stacked full-width cards for routine data,
- decorative empty space,
- nested containers that each add padding,
- giant page titles,
- icons larger than the text beside them,
- dashboard-card mosaics,
- decorative gradients behind routine work surfaces,
- floating badges, promo chips, or ornamental widgets.

## Density rules

- show `10-14` list or table rows on desktop before vertical scrolling where applicable,
- keep toolbars to one row,
- keep sidebars compact,
- reduce empty space aggressively,
- use the default spacing scale `4, 8, 12, 16`,
- avoid `24+` except for outer shell spacing when true shell separation demands it,
- keep default control heights in the `32-36px` range,
- avoid oversized icons and illustrations in product screens.

## Typography rules

- base text stays in the `13-14px` range,
- secondary labels and meta text stay in the `12-13px` range,
- section headers stay in the `15-17px` range,
- page titles stay in the `20-22px` range and should usually remain at the lower end,
- never use display-style text in product UI,
- keep contrast strong and line lengths readable,
- prefer clear utility labels over clever wording.

## Visual and content rules

- use one accent color only,
- keep chrome minimal,
- prefer thin dividers over heavy borders,
- use utility copy rather than marketing copy,
- headings must describe the area or action plainly,
- every region must earn its space.

## Framework rules

- use the idioms of the chosen Rust framework,
- do not inherit oversized defaults from the host toolkit,
- override theme constants globally to achieve compact spacing, compact controls, and restrained typography,
- favor reusable layout primitives and predictable component sizing.

## Self-audit

Before final output, report:

- visible row count at `1440x900`,
- toolbar height,
- sidebar width,
- body text size,
- title size,
- control height,
- any widget using fill or stretch behavior and why.

Revise until all of these are true:

1. At `1440x900`, nothing important is pushed below the fold by oversized headers or padding.
2. The screen reads clearly in three seconds.
3. Fonts are compact and legible rather than dramatic.
4. Spacing is tight enough for work and loose enough to scan.
5. Primary actions are visible immediately.
6. No region feels like a decorative card unless it is interactive.
7. The result looks like a serious desktop work app rather than a marketing mockup.

Revise until the screen fits this contract and still reads as severe, explicit, compact, and calm under New Alphabet law.
