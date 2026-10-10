#!/usr/bin/env bash
# Negative tests for .gitleaks.toml: fake secrets MUST be flagged, public
# testnet ids MUST NOT be. Usage: scripts/test-gitleaks-config.sh [gitleaks-binary]
set -u
GL="${1:-gitleaks}"
CFG="$(cd "$(dirname "$0")/.." && pwd)/.gitleaks.toml"
ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT
fail=0

B32="ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
SEED="S${B32}ABCDEFGHIJKLMNOPQRSTUVW"   # fake Stellar secret seed, 56 chars
PUB="G${B32}ABCDEFGHIJKLMNOPQRSTUVW"    # fake public account id, 56 chars
# Token prefixes are split so this file itself does not trip the scanner.
GH_PREFIX="gh""p_"
AWS_PREFIX="AK""IA"
[ ${#SEED} -eq 56 ] && [ ${#PUB} -eq 56 ] || { echo "bad fixture length"; exit 2; }

# case_ <label> <leak|clean> <file path> <content>
# A fresh repository per case: gitleaks scans every ref, so a shared repo would
# let one case contaminate the next.
case_() {
  local T
  T="$(mktemp -d -p "$ROOT")"
  (
    cd "$T" && git init -q && git config user.email t@t && git config user.name t
    mkdir -p "$(dirname "$3")"
    printf '%s\n' "$4" > "$3"
    git add -A && git commit -q -m t
  )
  local got
  if (cd "$T" && "$GL" git --config "$CFG" --no-banner --redact -l error >/dev/null 2>&1); then
    got=clean
  else
    got=leak
  fi
  if [ "$got" = "$2" ]; then echo "PASS $1 ($got)"; else echo "FAIL $1: expected $2, got $got"; fail=1; fi
}

case_ "stellar seed in src is flagged"          leak  src/a.ts            "const k = '$SEED';"
case_ "stellar seed in .env.example is flagged" leak  .env.example        "STELLAR_SECRET=$SEED"
case_ "github token is flagged"                 leak  src/b.ts            "const t = '${GH_PREFIX}aBcDeFgHiJkLmNoPqRsTuVwXyZ0123456789';"
case_ "aws key is flagged"                      leak  src/c.ts            "aws_access_key_id = ${AWS_PREFIX}IOSFODNN7ABCDEFG"
case_ "public G id in code is allowed"          clean src/d.ts            "const issuer = '$PUB';"
case_ "public G id in contracts/README allowed" clean contracts/README.md "issuer $PUB"
# The allowlist must never exempt a secret seed, even inside an exempt path.
case_ "seed hidden in contracts/README is flagged" leak contracts/README.md "secret $SEED"
exit $fail
