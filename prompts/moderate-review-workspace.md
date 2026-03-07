# Moderate Review Workspace Prompt

User prompt:

```text
Build me a B2B admin workspace for reviewing submissions with a left rail, dense table, and right-side detail pane.
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

Validation focus:

- `composition`
- `state_coverage`
- `accessibility`

Expected outcome:

Keep navigation and filters in the optional rail, keep the results field primary, and keep detail inspection adjacent rather than turning the surface into generic dashboard chrome.
