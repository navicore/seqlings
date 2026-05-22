#!/usr/bin/env bash
# scripts/audit-curriculum.sh
#
# Mechanical audit of the seqlings corpus. Reports findings to stdout
# and exits nonzero if any. See docs/design/SOLUTIONS_AND_HINTS_AUDIT.md
# for the rationale and finding classes.
#
# Checks (all by default):
#   solutions  — every solutions/*.seq passes `seqc test`
#   stubs      — every exercises/*.seq lints clean (errors only)
#
# Hints are deliberately NOT scanned here. Pedagogy ("does this hint
# scaffold or spoon-feed?") is subjective and gets a separate
# human-audit pass, tracked outside this script.
#
# Each finding line:  FINDING <class> <path> -- <details>
# Classes: SolutionFails, SolutionTimeout, StubLintError

set -uo pipefail

# ============================================================
# Args
# ============================================================
CHAPTER=""
CHECK="all"
TIMEOUT=30
VERBOSE=0

usage() {
  cat <<'EOF'
Usage: scripts/audit-curriculum.sh [options]

Options:
  --chapter NN         Restrict to chapter directory beginning "NN-"
                       (e.g. --chapter 38).
  --check WHICH        One of: all (default), solutions, stubs.
  --timeout SECS       Per-solution test timeout. Default: 30.
  -v, --verbose        Print every file as it's checked.
  -h, --help           This text.

Reports findings to stdout. Exits 0 if clean, 1 if any findings,
2 on bad arguments or missing tooling.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --chapter)    CHAPTER="$2"; shift 2 ;;
    --check)      CHECK="$2"; shift 2 ;;
    --timeout)    TIMEOUT="$2"; shift 2 ;;
    -v|--verbose) VERBOSE=1; shift ;;
    -h|--help)    usage; exit 0 ;;
    *) echo "Unknown arg: $1" >&2; usage >&2; exit 2 ;;
  esac
done

case "$CHECK" in
  all|solutions|stubs) ;;
  *) echo "Unknown --check: $CHECK" >&2; usage >&2; exit 2 ;;
esac

command -v seqc >/dev/null || {
  echo "Error: seqc not on PATH" >&2; exit 2
}

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

FINDINGS_FILE="$TMP/findings"
: > "$FINDINGS_FILE"

# ============================================================
# Helpers
# ============================================================
chapter_filter() {
  # Returns 0 if path-fragment $1 passes the chapter filter, else 1.
  local dir="$1"
  if [[ -n "$CHAPTER" && "$dir" != "$CHAPTER"-* ]]; then
    return 1
  fi
}

finding() {
  local class="$1" path="$2" details="${3:-}"
  if [[ -n "$details" ]]; then
    echo "FINDING $class $path -- $details" | tee -a "$FINDINGS_FILE"
  else
    echo "FINDING $class $path" | tee -a "$FINDINGS_FILE"
  fi
}

log() { [[ "$VERBOSE" -eq 1 ]] && echo "  $*" >&2; }

# ============================================================
# Check: solutions
# ============================================================
check_solutions() {
  echo "==> Checking solutions/" >&2
  local count=0
  for chap_dir in "$ROOT/solutions"/*/; do
    local chap; chap="$(basename "$chap_dir")"
    chapter_filter "$chap" || continue
    for sol in "$chap_dir"*.seq; do
      [[ -f "$sol" ]] || continue
      count=$((count + 1))
      log "solution: $sol"
      local rel="solutions/$chap/$(basename "$sol")"
      # seqc test requires the file to be named test-*.seq.
      local testfile="$TMP/test-$(basename "$sol")"
      cp "$sol" "$testfile"
      local out exit_code
      out="$(timeout "$TIMEOUT" seqc test "$testfile" 2>&1)"; exit_code=$?
      if [[ "$exit_code" -eq 124 ]]; then
        finding SolutionTimeout "$rel" "exceeded ${TIMEOUT}s"
      elif echo "$out" | grep -q 'No tests found'; then
        # Compile-mode exercises (chapter 00, a few others) have no
        # test-* words; `seqc test` exits 2 and says so. Not a defect.
        :
      elif echo "$out" | grep -qE '[1-9][0-9]* failed|FAILED|panicked'; then
        local snippet
        snippet="$(echo "$out" | grep -E 'FAILED|panicked|[1-9][0-9]* failed' | head -1 | tr -s ' ')"
        finding SolutionFails "$rel" "$snippet"
      elif [[ "$exit_code" -ne 0 ]]; then
        local snippet
        snippet="$(echo "$out" | grep -iE 'error' | head -1 | tr -s ' ')"
        finding SolutionFails "$rel" "exit $exit_code${snippet:+: $snippet}"
      fi
    done
  done
  echo "    checked $count solution file(s)" >&2
}

# ============================================================
# Check: stubs
# ============================================================
check_stubs() {
  echo "==> Checking exercises/ stubs (lint errors only)" >&2
  local count=0
  for chap_dir in "$ROOT/exercises"/*/; do
    local chap; chap="$(basename "$chap_dir")"
    chapter_filter "$chap" || continue
    for stub in "$chap_dir"*.seq; do
      [[ -f "$stub" ]] || continue
      count=$((count + 1))
      log "stub: $stub"
      local rel="exercises/$chap/$(basename "$stub")"
      local out
      out="$(seqc lint --errors-only "$stub" 2>&1 || true)"
      if echo "$out" | grep -qiE 'error|parse error'; then
        local snippet
        snippet="$(echo "$out" | grep -iE 'error' | head -1 | tr -s ' ')"
        finding StubLintError "$rel" "$snippet"
      fi
    done
  done
  echo "    checked $count stub file(s)" >&2
}

# ============================================================
# Main
# ============================================================
echo "seqlings curriculum audit"
echo "  chapter:  ${CHAPTER:-all}"
echo "  check:    $CHECK"
echo "  timeout:  ${TIMEOUT}s"
echo

case "$CHECK" in
  all)       check_solutions; echo; check_stubs ;;
  solutions) check_solutions ;;
  stubs)     check_stubs ;;
esac

echo
total="$(wc -l < "$FINDINGS_FILE" | tr -d ' ')"
if [[ "$total" -eq 0 ]]; then
  echo "OK — no findings."
  exit 0
fi

echo "FAIL — $total finding(s)."
echo
echo "Counts by class:"
awk '/^FINDING/{print $2}' "$FINDINGS_FILE" | sort | uniq -c | sed 's/^/  /'
exit 1
