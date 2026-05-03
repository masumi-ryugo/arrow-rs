#!/bin/bash -eu
# Build the seed corpus zip for `csv_reader` from arrow-testing.
#
# Usage: csv_reader_seed_corpus.sh <arrow-testing-checkout> <output-zip>

ARROW_TESTING="${1:?arrow-testing checkout path required}"
OUT_ZIP="${2:?output zip path required}"

SRC_DIR="$ARROW_TESTING/data/csv/fuzzing"
if [ ! -d "$SRC_DIR" ]; then
  # Fall back to the regular csv data directory if arrow-testing doesn't yet
  # have a fuzzing/ subdir for csv.
  SRC_DIR="$ARROW_TESTING/data/csv"
fi

if [ ! -d "$SRC_DIR" ]; then
  echo "WARN: $SRC_DIR not present; producing empty corpus" >&2
  : > /tmp/empty.txt
  zip -j "$OUT_ZIP" /tmp/empty.txt
  rm -f /tmp/empty.txt
  exit 0
fi

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
find "$SRC_DIR" -type f \
    ! -iname 'README*' ! -iname '*.json' ! -iname '*.md' -print0 \
  | xargs -0 -I {} cp {} "$TMP/" 2>/dev/null || true

if [ -z "$(ls -A "$TMP")" ]; then
  : > "$TMP/empty.csv"
fi
( cd "$TMP" && zip -r "$OUT_ZIP" . > /dev/null )
echo "Built csv_reader seed corpus: $(unzip -l "$OUT_ZIP" | tail -1)"
