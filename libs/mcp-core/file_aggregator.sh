#!/usr/bin/env bash

set -euo pipefail

# Usage: ./file_aggregator.sh "<dir1,dir2,...>" <output_file> [exclude_paths...]
# Example:
#   ./file_aggregator.sh "src,src-tauri" combined.txt "src-tauri/target,node_modules"

if [ $# -lt 2 ]; then
  echo "Usage: $0 \"<directory1,directory2,...>\" <output_file> [exclude_paths...]"
  exit 1
fi

# Split the comma-separated directories
IFS=',' read -ra dirs <<< "$1"
outfile="$2"
# Initialize excludes as an empty array if no exclusions provided
excludes=()
# Split the comma-separated exclusions if provided
if [ $# -gt 2 ] && [ -n "$3" ]; then
  IFS=',' read -ra excludes <<< "$3"
fi
shift 2
[ $# -gt 0 ] && shift

rm -f "$outfile"
touch "$outfile"

script_name="$(basename "$0")"
outfile_name="$(basename "$outfile")"

echo "==> Directories to process: ${dirs[*]}"
echo "==> Output file: $outfile"
echo "==> Exclusion patterns: ${excludes[*]:-none}"

for dir in "${dirs[@]}"; do
  # Remove trailing slash
  dir="${dir%/}"
  echo "----------------------------------------"
  echo "Processing directory: $dir"

  # Build the 'find' command as an array to avoid shell-parsing issues
  find_cmd=( find "$dir" "(" )

  # Always exclude this script and the output file
  # (both top-level and nested)
  find_cmd+=( 
    "-path" "$script_name" "-o" "-path" "$script_name/*"
    "-o" "-path" "*/$script_name" "-o" "-path" "*/$script_name/*"
    "-o" "-path" "$outfile_name" "-o" "-path" "$outfile_name/*"
    "-o" "-path" "*/$outfile_name" "-o" "-path" "*/$outfile_name/*"
  )

  # Add the user-specified exclusions, similarly for top-level + nested
  if [ ${#excludes[@]} -gt 0 ]; then
    for exclude in "${excludes[@]}"; do
      [ -z "$exclude" ] && continue
      echo "Will exclude: $exclude"
      find_cmd+=( 
        "-o" "-path" "$exclude" 
        "-o" "-path" "$exclude/*" 
        "-o" "-path" "*/$exclude" 
        "-o" "-path" "*/$exclude/*"
      )
    done
  fi

  # Close parentheses, prune matches, otherwise print files
  find_cmd+=( ")" "-prune" "-o" "-type" "f" "-print" )

  echo "Running find command: ${find_cmd[*]}"
  "${find_cmd[@]}" | while read -r file; do
    echo "Including file: $file"
    relpath="${file#$dir/}"
    echo "# $relpath" >> "$outfile"
    filesize=$(wc -c < "$file")
    echo "Size: ${filesize} bytes"
    cat "$file" >> "$outfile"
    echo >> "$outfile"
  done
done
