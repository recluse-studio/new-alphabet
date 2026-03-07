# Components

Components are semantic surfaces built on primitives.

## Inventory

Actions:

- `Button`
- `LinkAction`

Fields:

- `TextField`
- `Textarea`
- `Select`
- `Checkbox`
- `RadioGroup`
- `Switch`

Feedback and status:

- `StatusBadge`
- `InlineAlert`
- `EmptyState`

Dense data and workflow:

- `Table`
- `MetricBlock`
- `Pagination`
- `NavIndex`
- `CommandBar`
- `FilterRail`
- `DetailPane`

These live in `crates/new-alphabet-components`.

## State law

- serious components define required states,
- tables define `loading`, `empty`, and `error`,
- detail panes define `loading` and `unavailable`,
- filters define `zero_result`,
- actions and fields keep interactive and validation states explicit.

## Accessibility

The machine-readable checklist lives in `new_alphabet_components::COMPONENT_ACCESSIBILITY_CHECKS`.

Required rules include:

- semantic names,
- keyboard support where relevant,
- focus-visible behavior,
- color-independent status meaning.

## Example mapping

- `WorkflowActionExample` covers action hierarchy,
- `FormFieldExample` and `SettingsFieldExample` cover field bindings,
- `DashboardDataExample` and `ReviewDataExample` cover dense operational content,
- `AccessibilityCoverageExample` maps the checklist back to live markup.
