#!/bin/bash

PROJECT_NAME=$(basename "$(pwd)")
EXPORT_DIR="./txt_export"
MAX_CHARS=100000   # ≈ 25k tokens (veilig onder limiet)

mkdir -p "$EXPORT_DIR"

chunk=1
char_count=0
out_file="${EXPORT_DIR}/${PROJECT_NAME}_tokens_${chunk}.txt"
> "$out_file"

add_file() {
  local file="$1"
  local content
  content=$(cat "$file")
  local size=${#content}

  # Als dit bestand niet past, begin nieuw chunk
  if [ $((char_count + size)) -gt "$MAX_CHARS" ]; then
    chunk=$((chunk + 1))
    char_count=0
    out_file="${EXPORT_DIR}/${PROJECT_NAME}_tokens_${chunk}.txt"
    > "$out_file"
  fi

  echo "=== $file ===" >> "$out_file"
  echo "$content" >> "$out_file"
  echo >> "$out_file"

  char_count=$((char_count + size))
}

find . \
  \( -name "Cargo.toml" -o -name "package.json" -o -name "tsconfig.json" \
     -o -name "*.rs" -o -name "*.ts" -o -name "*.tsx" -o -name "*.js" \
     -o -name "*.jsx" -o -name "*.vue" -o -name "*.svelte" \
     -o -name "*.html" -o -name "*.css" -o -name "*.proto" \) \
  -not -path "*/target/*" \
  -not -path "*/node_modules/*" \
  -not -path "./export/*" \
  | sort | while read -r file; do
      add_file "$file"
done

echo "Token-safe export created in: $EXPORT_DIR"