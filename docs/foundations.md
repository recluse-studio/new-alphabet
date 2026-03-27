# Foundations

New Alphabet foundations are law, not styling garnish.

## Scope

The foundation layer fixes:

- breakpoints and page columns,
- region spans and rail widths,
- spacing tokens,
- type roles,
- density modes,
- color roles,
- border rules,
- radius rules,
- motion rules,
- and state tokens.

These contracts live in `crates/new-alphabet-foundation`.

## Grid

- `compact` uses 4 columns.
- `medium` uses 8 columns.
- `wide` uses 12 columns.
- `rail` and `detail` collapse by breakpoint instead of inventing free layout behavior.
- `main` remains the named primary field on every serious surface.

## Spacing

- spacing is finite and token-driven,
- density changes the effective point values,
- arbitrary spacing repair is invalid,
- stack and row rhythm should come from primitives, not wrappers.

## Type and density

- type roles are `display`, `heading`, `body`, `annotation`, and `data`,
- density modes are `calm`, `regular`, and `dense`,
- editorial and workflow surfaces share one type law.

## Surface language

- color roles are semantic and finite,
- first builds keep the canonical palette strict,
- border weight carries hierarchy,
- corner radius stays subtle and token-driven,
- motion is short and reduced-motion aware,
- global state tokens stay explicit.

## Verification

- foundation rules are exported through `new-alphabet-schema`,
- validation uses the same ids and families,
- runtime checks live in `crates/new-alphabet-foundation/src/lib.rs`.
