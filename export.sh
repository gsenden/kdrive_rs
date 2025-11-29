#!/bin/bash

PROJECT_NAME=$(basename "$(pwd)")
EXPORT_DIR="./txt_export"
OUTPUT="${EXPORT_DIR}/${PROJECT_NAME}.txt"

mkdir -p "$EXPORT_DIR"

> "$OUTPUT"

find . \( -name "Cargo.toml" -o -name "package.json" -o -name "tsconfig.json" \) \
    -not -path "*/target/*" \
    -not -path "*/node_modules/*" \
    -not -path "./export/*" | sort | while read -r file; do
    echo "=== $file ===" >> "$OUTPUT"
    cat "$file" >> "$OUTPUT"
    echo -e "\n" >> "$OUTPUT"
done

find . \( -name "*.rs" -o -name "*.ts" -o -name "*.tsx" -o -name "*.js" -o -name "*.jsx" -o -name "*.vue" -o -name "*.svelte" -o -name "*.html" -o -name "*.css" \) \
    -not -path "*/target/*" \
    -not -path "*/node_modules/*" \
    -not -path "./export/*" | sort | while read -r file; do
    echo "=== $file ===" >> "$OUTPUT"
    cat "$file" >> "$OUTPUT"
    echo -e "\n" >> "$OUTPUT"
done

echo "Text export created: $OUTPUT"