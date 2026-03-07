#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

cargo run -q -p new-alphabet-docs
cargo run -q -p new-alphabet-demo-blog
cargo run -q -p new-alphabet-demo-saas
