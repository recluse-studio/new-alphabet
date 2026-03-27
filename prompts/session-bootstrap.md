# Session Bootstrap

Load context in this order:

1. `AGENTS.md`
2. `documentation.md`
3. `prd.json`
4. `progress.txt`
5. `schemas/context-bundle-0.1.0.json`

Load targeted reference files only when the task touches that layer:

- `docs/foundations.md`
- `docs/workbench-policy.md`
- `docs/flavors.md`
- `docs/primitives.md`
- `docs/components.md`
- `docs/recipes.md`
- `docs/cli.md`
- `docs/agent-contract.md`

Operating rules:

1. Apply `docs/workbench-policy.md` when the task is a dense work surface and the stack is unspecified or the host flavor does not already impose stricter shell law.
2. Choose an explicit runtime flavor when the task names a host stack or platform boundary.
3. Choose an existing recipe or allowed primitive composition before writing code.
4. Keep prompts, plans, and explanations in framework language rather than generic design prose.
5. Use the exported bundle and checked-in docs as the source of truth when the runtime and prose must agree.
6. Validate drift in rule terms and prefer deleting invalid structure before adding abstraction.
7. Close the loop with `cargo test --workspace` and the relevant `new-alphabet` CLI command before marking work complete.

Reusable session opener:

```text
Read AGENTS.md, documentation.md, prd.json, progress.txt, and schemas/context-bundle-0.1.0.json.
Load docs/foundations.md, docs/workbench-policy.md, docs/flavors.md, docs/primitives.md, docs/components.md, docs/recipes.md, docs/cli.md, and docs/agent-contract.md only if the task touches those layers.
Apply docs/workbench-policy.md when the task is a dense work surface and the stack is unspecified or the host flavor does not already impose stricter shell law.
Choose an explicit runtime flavor when the task names a host stack or platform boundary.
Choose an existing recipe or allowed primitive composition before writing code.
Explain, scaffold, inspect, and validate in New Alphabet terms rather than generic UI language.
Prefer deletion or narrowing over new abstraction when drift appears.
```
