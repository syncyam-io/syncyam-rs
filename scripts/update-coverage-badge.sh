#!/bin/bash

# Script to update coverage badge in README.md
# This script extracts coverage percentage from tarpaulin report and updates the badge

set -e

COVERAGE_REPORT="coverage/tarpaulin-report.html"
README_FILE="README.md"

if [ ! -f "$COVERAGE_REPORT" ]; then
    echo "Error: Coverage report not found at $COVERAGE_REPORT"
    echo "Please run 'make tarpaulin' first to generate the coverage report"
    exit 1
fi

# Extract coverage percentage
COVERAGE=$(grep -oP 'coverage: \K[0-9.]+' "$COVERAGE_REPORT" | head -1)

if [ -z "$COVERAGE" ]; then
    echo "Error: Could not extract coverage percentage from report"
    exit 1
fi

# Determine badge color based on coverage
if (( $(echo "$COVERAGE >= 90" | bc -l) )); then
    COLOR="brightgreen"
elif (( $(echo "$COVERAGE >= 80" | bc -l) )); then
    COLOR="green"
elif (( $(echo "$COVERAGE >= 70" | bc -l) )); then
    COLOR="yellowgreen"
elif (( $(echo "$COVERAGE >= 60" | bc -l) )); then
    COLOR="yellow"
else
    COLOR="red"
fi

# Create dynamic badge URL
BADGE_URL="https://img.shields.io/badge/coverage-${COVERAGE}%25-${COLOR}.svg"

echo "Coverage: ${COVERAGE}%"
echo "Badge URL: $BADGE_URL"

# Update README.md
if [ -f "$README_FILE" ]; then
    # Use sed to replace the coverage badge
    sed -i.bak "s|\[!\[Coverage\](https://img\.shields\.io/badge/coverage-[0-9.]*%25-[a-z]*\.svg)\]|\[!\[Coverage\]($BADGE_URL)\]|g" "$README_FILE"
    
    # Remove backup file
    rm -f "${README_FILE}.bak"
    
    echo "Updated coverage badge in $README_FILE"
else
    echo "Error: README.md not found"
    exit 1
fi
