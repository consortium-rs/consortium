#!/bin/bash
# Quick test suite runner

echo "=========================================="
echo "Testing Parser Output"
echo "=========================================="
echo ""

echo "1. Running custom validation checks..."
python scripts/pac/test_output.py 2>&1 | grep -E "^(Register|Chip|Total|✓ All)" | tail -4

echo ""
echo "2. Running chiptool validation..."
python scripts/pac/validate_with_chiptool.py 2>&1 | grep -E "^(Passed|Failed):" | head -2

echo ""
echo "3. File statistics..."
echo "   YAML files generated: $(find crates/consortium-data/database/peri -name '*_v1.yaml' | wc -l)"
echo "   Chip JSON files:      $(find crates/consortium-data/database/chip -name '*.json' | wc -l)"
echo ""
echo "=========================================="
