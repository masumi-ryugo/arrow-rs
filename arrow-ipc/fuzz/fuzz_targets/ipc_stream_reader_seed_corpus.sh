#!/bin/bash -eu
# Build the seed corpus zip for `ipc_stream_reader` from arrow-testing.
#
# Usage: ipc_stream_reader_seed_corpus.sh <arrow-testing-checkout> <output-zip>

ARROW_TESTING="${1:?arrow-testing checkout path required}"
OUT_ZIP="${2:?output zip path required}"

SRC_DIR="$ARROW_TESTING/data/arrow-ipc-stream"
if [ ! -d "$SRC_DIR" ]; then
  echo "WARN: $SRC_DIR not present in arrow-testing; producing empty corpus" >&2
  : > /tmp/empty.txt
  zip -j "$OUT_ZIP" /tmp/empty.txt
  rm -f /tmp/empty.txt
  exit 0
fi

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
# Pull in both `*.stream` golden inputs and prior clusterfuzz testcases (which
# have no extension), but skip README files and any text-mode metadata.
find "$SRC_DIR" -type f \
    ! -iname 'README*' ! -iname '*.json' ! -iname '*.md' -print0 \
  | xargs -0 -I {} cp {} "$TMP/" 2>/dev/null || true

if [ -z "$(ls -A "$TMP")" ]; then
  echo "WARN: no seed inputs found under $SRC_DIR" >&2
  : > "$TMP/empty.bin"
fi
( cd "$TMP" && zip -r "$OUT_ZIP" . > /dev/null )
echo "Built ipc_stream_reader seed corpus: $(unzip -l "$OUT_ZIP" | tail -1)"
