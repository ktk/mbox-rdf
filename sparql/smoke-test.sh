#!/bin/bash
# Smoke test: runs every .rq file in the sparql/ directory against a SPARQL endpoint
# and checks that each returns at least one result.
#
# Usage:
#   ./sparql/smoke-test.sh
#   SPARQL_ENDPOINT=http://other-host:7029 ./sparql/smoke-test.sh

set -euo pipefail

SPARQL_ENDPOINT="${SPARQL_ENDPOINT:-http://localhost:7029}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PASSED=0
FAILED=0
ERRORS=""

echo "SPARQL smoke test against ${SPARQL_ENDPOINT}"
echo "============================================="

for rq_file in "${SCRIPT_DIR}"/*.rq; do
  name="$(basename "$rq_file")"
  query="$(cat "$rq_file")"

  # URL-encode and POST to SPARQL endpoint
  response=$(curl -s -w "\n%{http_code}" \
    --data-urlencode "query=${query}" \
    -H "Accept: application/sparql-results+json" \
    "${SPARQL_ENDPOINT}" 2>&1) || true

  http_code=$(echo "$response" | tail -1)
  body=$(echo "$response" | sed '$d')

  if [ "$http_code" != "200" ]; then
    echo "  ✗ ${name} — HTTP ${http_code}"
    FAILED=$((FAILED + 1))
    ERRORS="${ERRORS}\n  ${name}: HTTP ${http_code}"
    continue
  fi

  # Count results: look for "bindings" array length
  count=$(echo "$body" | grep -o '"bindings"' | head -1)
  # More robust: count actual result rows
  num_results=$(echo "$body" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    print(len(data.get('results', {}).get('bindings', [])))
except:
    print(0)
" 2>/dev/null || echo "0")

  if [ "$num_results" -gt 0 ] 2>/dev/null; then
    echo "  ✓ ${name} — ${num_results} result(s)"
    PASSED=$((PASSED + 1))
  else
    echo "  ✗ ${name} — 0 results"
    FAILED=$((FAILED + 1))
    ERRORS="${ERRORS}\n  ${name}: 0 results"
  fi
done

echo ""
echo "Results: ${PASSED} passed, ${FAILED} failed"

if [ "$FAILED" -gt 0 ]; then
  echo -e "\nFailed queries:${ERRORS}"
  exit 1
fi
