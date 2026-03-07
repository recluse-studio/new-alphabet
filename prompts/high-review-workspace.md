# High Review Workspace Prompt

User prompt:

```text
Build a dense review workspace with left navigation rail, center results table, right detail pane, persistent action strip, loading and dirty states, and a calm editorial tone.
```

Recipe path:

- primary surface: `ReviewQueue`

Primitive anchor:

- `AppShell`
- `PageGrid`
- `Rail`
- `Region`
- `Panel`
- `Stack`
- `Band`

Component anchor:

- `CommandBar`
- `Table`
- `FilterRail`
- `DetailPane`
- `InlineAlert`
- `NavIndex`

Validation focus:

- `composition`
- `state_coverage`
- `accessibility`
- `naming`

Expected outcome:

Keep the action strip in `action_band`, keep results in `main`, keep detail inspection explicit, and validate loading, unavailable, and dirty-state behavior before export.
