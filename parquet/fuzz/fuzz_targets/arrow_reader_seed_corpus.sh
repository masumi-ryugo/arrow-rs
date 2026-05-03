#!/bin/bash -eu
# Build the seed corpus zip for the parquet `arrow_reader` fuzz target out of
# the upstream apache/arrow-testing snapshots.
#
# Usage: arrow_reader_seed_corpus.sh <arrow-testing-checkout> <output-zip>

ARROW_TESTING="${1:?arrow-testing checkout path required}"
OUT_ZIP="${2:?output zip path required}"

SRC_DIR="$ARROW_TESTING/data/parquet/fuzzing"
if [ ! -d "$SRC_DIR" ]; then
  echo "WARN: $SRC_DIR not present in arrow-testing; producing empty corpus" >&2
  : > /tmp/empty.txt
  zip -j "$OUT_ZIP" /tmp/empty.txt
  rm -f /tmp/empty.txt
  exit 0
fi

# Skip the README and any tracked README-like files; corpus entries should be
# raw inputs only.
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
find "$SRC_DIR" -type f ! -name 'README*' -print0 \
  | xargs -0 -I {} cp {} "$TMP/"
( cd "$TMP" && zip -r "$OUT_ZIP" . > /dev/null )
echo "Built parquet arrow_reader seed corpus: $(unzip -l "$OUT_ZIP" | tail -1)"
