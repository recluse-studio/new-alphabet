# Roadmap

This file defines the public V0 posture.

## V0 scope

### Must ship

- constitution and naming law,
- foundation runtime,
- primitives,
- core semantic components,
- editorial and workflow recipe proofs,
- schema bundle and validation baseline,
- CLI init, add, explain, validate, inspect, export context, and structured plan,
- docs site and canonical reference docs,
- demo blog and demo SaaS,
- OSS launch materials and remote-ready repository assembly.

### Should ship

- deeper plan matching,
- sharper anti-pattern coverage,
- broader example mapping in docs and exported context.

### Later

- additional workflow recipes,
- controlled extension points,
- richer recipe browsing,
- protocol-native agent adapters,
- constrained recipe synthesis.

## Milestones

- `M0` Constitution Freeze: philosophy, naming, grid, type, density, color-role rules, and anti-patterns are fixed.
- `M1` Runtime Foundations: foundation contracts and structural primitives are real.
- `M2` Semantic Layer: the minimum serious component set is complete.
- `M3` Recipe Proofs: editorial and workflow recipes prove one family.
- `M4` CLI and Context Export: CLI commands work and the agent-readable contract can be generated.
- `M5` Public Offering: docs, demos, release materials, and repository structure are public-ready.

## Quality requirements

### Performance

- no unnecessary runtime styling churn,
- reasonable SSR performance for demo surfaces,
- modest hydration cost for editorial surfaces.

### Doctrinal quality

- no V0 surface should read as generic component-library output,
- editorial and workflow surfaces should remain recognizably one family,
- adaptability must happen through approved structures rather than ad hoc styling.

### Reliability

- contracts and component APIs stay strongly typed,
- examples and scaffold output stay deterministic where feasible,
- schema and implementation parity is maintained.

### Security

- the CLI must not execute arbitrary instructions by default,
- scaffold and export commands must be explicit about file writes,
- basic framework use must remain network-free.

### Maintainability

- layer boundaries remain clear,
- recipes do not bypass foundations and primitives,
- documentation and exported context stay aligned with runtime behavior.

## Testing strategy

### Unit

- foundation token and rule tests,
- schema serialization and validation tests,
- component prop and state tests.

### Integration

- recipe composition tests,
- SSR and hydration tests where relevant,
- CLI command tests.

### Agentic

- context-bundle completeness tests,
- prompt-to-plan fixture tests,
- repair-suggestion tests.

### Snapshot or visual

- recipe reference output tests,
- generated docs and demo output checks.

### Conformance

- schema parity checks,
- accessibility checks,
- spacing and anti-pattern checks.

## Acceptance gates

### Design gate

- primitives align to the declared grid and spacing rules,
- hierarchy remains coherent across editorial and workflow examples,
- recipes read as one family.

### Aesthetic gate

- surfaces remain recognizably New Alphabet,
- the system does not collapse into generic contemporary SaaS styling,
- Crouwelian logic stays visible in structure and hierarchy.

### API gate

- component APIs are narrow and documented,
- variants are finite and explicit,
- unsupported combinations are blocked or rejected.

### State gate

- required states are implemented and documented,
- no serious surface ships with undefined loading, empty, or error behavior where relevant.

### Accessibility gate

- keyboard use is verified,
- focus-visible behavior exists,
- semantics and accessible naming are validated.

### Platform gate

- SSR works,
- hydration works where used,
- responsive behavior is defined for required recipes.

### Agent gate

- schemas exist for foundations, primitives, core components, and recipes,
- CLI can export a context bundle,
- at least three prompt-to-plan examples are demonstrated.

### Docs gate

- README explains philosophy and quick start clearly,
- docs cover foundations, primitives, components, recipes, CLI, and agent workflows,
- examples are runnable and mapped to docs.

### OSS gate

- license files are present,
- contribution guidance exists,
- issue and PR templates exist,
- release-notes discipline is established.

## Success metrics

- two reference apps exist,
- at least three prompt-to-plan examples exist,
- at least five must-ship CLI surfaces exist,
- four V0 recipe proofs exist,
- README, docs site, examples, prompts, and exported context agree on naming and layer model.

## Risks

- over-abstraction: keep V0 narrow and implementation-led.
- over-expansion: treat grammar quality as more important than surface count.
- pastiche: test for Crouwelian logic, not visual cosplay.
- adoption friction: keep strict paths legible through docs and scaffolds.
- agent fantasy without executable rules: keep schemas, validation, and export real before broadening claims.

## Open questions

- how strict should token scales remain before demos expose edge cases,
- how much recipe adaptation is acceptable before a new recipe is required,
- which CLI flows need machine-readable output beyond V0,
- how much extension can exist later without dissolving the doctrine.
