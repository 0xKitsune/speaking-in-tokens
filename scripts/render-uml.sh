#!/usr/bin/env bash
set -euo pipefail
# Local-only rendering: supply a downloaded PlantUML jar and optional Java binary.
: "${PLANTUML_JAR:?Set PLANTUML_JAR to a local plantuml.jar}"
JAVA_BIN="${JAVA_BIN:-java}"
repo_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
"$JAVA_BIN" -Djava.awt.headless=true -jar "$PLANTUML_JAR" -tsvg "$repo_dir"/examples/raft/design/*.puml
