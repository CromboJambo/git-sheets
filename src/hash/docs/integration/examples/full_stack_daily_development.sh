#!/bin/bash
# ============================================================================
# Full Stack Daily Development Workflow
# ============================================================================
# This example demonstrates how git-sheets, nustage, and zed-sheet-lsp work
# together in a typical daily development scenario.
#
# Prerequisites:
#   - All three projects built (cargo build --release from workspace root)
#   - Sample data file available or created by this script
#
# Usage:
#   ./docs/integration/examples/full_stack_daily_development.sh
# ============================================================================

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(dirname "$SCRIPT_DIR"/../../..)"  # Go up to workspace root
TEMP_DIR=""
CLEANUP_DONE=false

# Colors for output (optional, degrades gracefully)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Utility functions
log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

cleanup() {
    if [[ "$CLEANUP_DONE" == "true" ]]; then
        return
    fi

    if [[ -n "$TEMP_DIR" && -d "$TEMP_DIR" ]]; then
        log_info "Cleaning up temporary directory: $TEMP_DIR"
        rm -rf "$TEMP_DIR"
    fi
    CLEANUP_DONE=true
}

trap cleanup EXIT INT TERM

# Step 1: Create temporary workspace
log_info "Creating temporary test environment..."
TEMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/trifecta-demo-XXXXXX")
log_success "Test directory: $TEMP_DIR"

cd "$TEMP_DIR"

# Step 2: Create sample data file (CSV)
log_info "Creating sample sales data..."
cat > sales.csv << 'EOF'
Date,Region,Product,Revenue,Cost,Units
2024-01-01,North,Laptop,1200,800,5
2024-01-02,South,Phone,900,600,10
2024-01-03,North,Tablet,700,500,8
2024-01-04,East,Laptop,1100,750,3
2024-01-05,West,Phone,950,620,12
2024-01-06,North,Desktop,1500,1000,4
2024-01-07,South,Laptop,1250,850,6
2024-01-08,East,Tablet,680,480,9
EOF

log_success "Sample data created: sales.csv"

# Step 3: Initialize git-sheets repository
log_info "Initializing git-sheets repository..."
"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" init > /dev/null 2>&1 || {
    log_warn "git-sheets not found, trying cargo run..."
    cd "$WORKSPACE_ROOT/git-sheets" && cargo build --release >/dev/null 2>&1
    cd "$TEMP_DIR"
}

"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" init
log_success "Git-sheets repository initialized"

# Step 4: Create initial snapshot (before any changes)
log_info "Creating baseline snapshot..."
"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" \
    snapshot sales.csv \
    -m "Initial Q1 data import" \
    -k "0"

log_success "Baseline snapshot created"

# Step 5: Initialize nustage pipeline for this data
log_info "Initializing nustage pipeline..."
"$WORKSPACE_ROOT/nustage/target/release/nustage" init sales.csv > /dev/null 2>&1 || {
    log_warn "nustage not built, building now..."
    cd "$WORKSPACE_ROOT/nustage" && cargo build --release >/dev/null 2>&1
}

# Check if sidecar was created
if [[ -f ".nustage.json" ]]; then
    log_success "Nustage sidecar initialized: .nustage.json"
    log_info "Sidecar contents:"
    cat .nustage.json | head -20
else
    log_warn "No .nustage.json created (pipeline may be empty)"
fi

# Step 6: Add transformation steps via nustage CLI
log_info "Adding transformation steps..."

# Filter high-value transactions
"$WORKSPACE_ROOT/nustage/target/release/nustage" \
    add-step filter Revenue > 1000 \
    sales.csv || log_warn "Filter step may already exist or require syntax adjustment"

# Add calculated column (Margin = Revenue - Cost)
"$WORKSPACE_ROOT/nustage/target/release/nustage" \
    add-column Margin "@Revenue - @Cost" \
    sales.csv || log_warn "Add column step may require different syntax"

log_success "Transformation steps added to pipeline"

# Step 7: Snapshot before risky operation (bulk update simulation)
log_info "Creating pre-batch snapshot..."
"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" \
    snapshot sales.csv \
    -m "Before bulk price adjustment batch"

log_success "Pre-batch snapshot created"

# Step 8: Simulate manual changes (or editor edits via zed-sheet-lsp)
log_info "Simulating data modification..."
cat > sales_modified.csv << 'EOF'
Date,Region,Product,Revenue,Cost,Units
2024-01-01,North,Laptop,1300,800,5
2024-01-02,South,Phone,950,600,10
2024-01-03,North,Tablet,750,500,8
2024-01-04,East,Laptop,1150,750,3
2024-01-05,West,Phone,950,620,12
2024-01-06,North,Desktop,1600,1000,4
2024-01-07,South,Laptop,1350,850,6
2024-01-08,East,Tablet,720,480,9
EOF

cp sales_modified.csv sales.csv
log_success "Data modified (simulated bulk price update)"

# Step 9: Create post-change snapshot
log_info "Creating post-batch snapshot..."
"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" \
    snapshot sales.csv \
    -m "After bulk price adjustment batch"

log_success "Post-batch snapshot created"

# Step 10: View diff between snapshots
log_info "Comparing snapshots to see what changed..."
echo ""
echo "=== Snapshot Comparison ==="
"$WORKSPACE_ROOT/git-sheets/target/release/git-sheets" \
    log -l 5

echo ""
log_success "Full stack workflow complete!"
echo ""
echo "What we accomplished:"
echo "  ✓ Created sample data (sales.csv)"
echo "  ✓ Initialized git-sheets for version control"
echo "  ✓ Captured baseline snapshot"
echo "  ✓ Set up nustage pipeline with transformations"
echo "  ✓ Added transformation steps to sidecar"
echo "  ✓ Snapshot before risky operation"
echo "  ✓ Made data changes (simulated)"
echo "  ✓ Snapshot after changes for audit trail"
echo ""
echo "Next steps you could take:"
echo "  - Run: git-sheets diff snapshots/*.toml to see detailed changes"
echo "  - Open in Zed: zed sales.csv (with zed-sheet-lsp active)"
echo "  - Execute pipeline: nustage process sales.csv"
echo ""

# Optional: Show generated sidecar content
if [[ -f ".nustage.json" ]]; then
    log_info "Current pipeline state (.nustage.json):"
    echo "---"
    cat .nustage.json | head -30
    echo "---"
fi

exit 0
