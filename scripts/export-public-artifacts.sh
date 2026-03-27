#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BUNDLE="schemas/context-bundle-0.1.0.json"

cargo run -q -p new-alphabet-cli -- export context --output "$BUNDLE"

jq '.foundations' "$BUNDLE" > schemas/foundations.json
jq '.flavors' "$BUNDLE" > schemas/flavors.json
jq '.primitives' "$BUNDLE" > schemas/primitives.json
jq '.components' "$BUNDLE" > schemas/components.json
jq '.recipes' "$BUNDLE" > schemas/recipes.json
jq '.composition_rules' "$BUNDLE" > schemas/composition-rules.json
jq '.state_contracts' "$BUNDLE" > schemas/state-contracts.json
jq '.anti_patterns' "$BUNDLE" > schemas/anti-patterns.json
jq '.validation_rules' "$BUNDLE" > schemas/validation-rules.json
jq '.examples' "$BUNDLE" > schemas/examples.json
jq '.prompt_intents' "$BUNDLE" > schemas/prompt-intents.json
jq '.schemas[] | select(.id == "new-alphabet.contract") | .document' "$BUNDLE" > schemas/contract-bundle.schema.json
jq '.schemas[] | select(.id == "new-alphabet.foundations") | .document' "$BUNDLE" > schemas/foundations.schema.json
jq '.schemas[] | select(.id == "new-alphabet.flavors") | .document' "$BUNDLE" > schemas/flavors.schema.json
jq '.schemas[] | select(.id == "new-alphabet.primitives") | .document' "$BUNDLE" > schemas/primitives.schema.json
jq '.schemas[] | select(.id == "new-alphabet.components") | .document' "$BUNDLE" > schemas/components.schema.json
jq '.schemas[] | select(.id == "new-alphabet.recipes") | .document' "$BUNDLE" > schemas/recipes.schema.json
jq '.schemas[] | select(.id == "new-alphabet.project") | .document' "$BUNDLE" > schemas/project.schema.json
