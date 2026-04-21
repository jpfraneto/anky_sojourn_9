#!/usr/bin/env bash
set -euo pipefail

SOURCE="program/target/idl/sojourn_9.json"
DESTINATION="idl/sojourn_9.json"

if [[ ! -f "${SOURCE}" ]]; then
  echo "IDL not found at ${SOURCE}. Run 'anchor build' in program/ first."
  exit 1
fi

cp "${SOURCE}" "${DESTINATION}"
echo "Copied ${SOURCE} -> ${DESTINATION}"

