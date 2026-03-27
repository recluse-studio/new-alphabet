# Session Bootstrap

Load context in this order:

1. `AGENTS.md`
2. `documentation.md`
3. `prd.json`
4. `progress.txt`
5. `schemas/context-bundle-0.1.0.json`

Load targeted reference files only when the task touches that layer:

- `docs/foundations.md`
- `docs/flavors.md`
- `docs/primitives.md`
- `docs/components.md`
- `docs/recipes.md`
- `docs/cli.md`
- `docs/agent-contract.md`

Operating rules:

1. Choose an explicit runtime flavor when the task names a host stack or platform boundary.
2. Choose an existing recipe or allowed primitive composition before writing code.
3. Keep prompts, plans, and explanations in framework language rather than generic design prose.
4. Use the exported bundle and checked-in docs as the source of truth when the runtime and prose must agree.
5. Validate drift in rule terms and prefer deleting invalid structure before adding abstraction.
6. Close the loop with `cargo test --workspace` and the relevant `new-alphabet` CLI command before marking work complete.

Reusable session opener:

```text
Read AGENTS.md, documentation.md, prd.json, progress.txt, and schemas/context-bundle-0.1.0.json.
Load docs/foundations.md, docs/flavors.md, docs/primitives.md, docs/components.md, docs/recipes.md, docs/cli.md, and docs/agent-contract.md only if the task touches those layers.
Choose an explicit runtime flavor when the task names a host stack or platform boundary.
Choose an existing recipe or allowed primitive composition before writing code.
Explain, scaffold, inspect, and validate in New Alphabet terms rather than generic UI language.
Prefer deletion or narrowing over new abstraction when drift appears.
```
