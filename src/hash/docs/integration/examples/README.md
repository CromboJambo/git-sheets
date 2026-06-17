# Integration Examples

This directory contains practical example workflows demonstrating how `git-sheets`, `nustage`, and `zed-sheet-lsp` work together (or independently) in real scenarios.

## Available Examples

### Full Stack Workflow (All Three Projects)

**File:** `full_stack_daily_development.sh`

Demonstrates a complete workflow using all three projects:
1. Open file in Zed with LSP diagnostics from `.nustage.json`
2. Make changes via editor code actions
3. Snapshot before risky operation using git-sheets
4. Run transformation pipeline via nustage CLI
5. Capture audit trail

**Use when:** You want the full power of all three projects integrated.

### Version Control Only (git-sheets Standalone)

**File:** `minimal_version_control.sh`

Shows how to use git-sheets without any other project:
1. Initialize repository
2. Snapshot data before running macro
3. Work in Excel or other spreadsheet tool
4. Snapshot after changes
5. Diff snapshots to see what changed

**Use when:** You just need version control for spreadsheets, no pipeline engine needed.

### Pipeline-First Workflow (nustage Standalone)

**File:** `pipeline_first_workflow.sh`

Demonstrates nustage without history tracking:
1. Initialize data file with pipeline
2. Add transformation steps
3. Execute pipeline
4. Export results

**Use when:** You want to focus on transformations, not version control.

### Audit Trail for Compliance

**File:** `audit_trail_compliance.sh`

Shows how to create regulatory-compliant audit trails:
1. Snapshot inherited data with metadata
2. Each change as separate snapshot with explicit message
3. Generate full audit trail report

**Use when:** You need to document changes for compliance or legal requirements.

### Editor Integration Pattern (Future)

**File:** `editor_integration_pattern.sh`

Demonstrates the pattern for editor-sidecar integration:
1. User triggers rename in Zed
2. LSP delegates to nustage for safe rename
3. Sidecar updated atomically
4. Optional snapshot captures before/after

**Use when:** Building editor features that need pipeline awareness.

## Running Examples

### Prerequisites

Each example has its own `requirements.txt` or inline comments listing dependencies. Generally:

```bash
# For full stack examples
cargo build --release  # Builds all projects

# For git-sheets only
cd git-sheets && cargo build --release

# For nustage only  
cd nustage && cargo build --release
```

### Executing Examples

Make each script executable first:

```bash
chmod +x docs/integration/examples/*.sh
```

Then run from workspace root:

```bash
./docs/integration/examples/full_stack_daily_development.sh
```

Each example is self-contained and creates its own temporary test data.

## Example Structure

Each example file follows this structure:

1. **Header:** Purpose and prerequisites
2. **Setup:** Create test environment with sample data
3. **Steps:** Numbered commands to execute
4. **Expected Output:** What you should see after each step
5. **Cleanup:** Optional teardown instructions

## Modifying Examples

When adding new examples:

1. Keep them self-contained (no shared state between runs)
2. Use temporary directories (`mktemp -d`) for test data
3. Document any assumptions about environment
4. Include error handling where appropriate
5. Add comments explaining each step's purpose

## Integration Verification

These examples also serve as integration tests. If an example fails, it may indicate:
- Breaking change in a project's API
- Missing dependency between projects
- Documentation out of sync with code

Run all examples periodically to verify integration integrity:

```bash
for script in docs/integration/examples/*.sh; do
    echo "Running $script..."
    bash "$script" || exit 1
done
echo "All integration examples passed!"
```

## Related Documentation

- [`../README.md`](../README.md) - Integration overview
- [`../AUDIT.md`](../AUDIT.md) - Type overlap analysis
- [`../dependency-config.md`](../dependency-config.md) - Dependency management

---

**Note:** These examples are living documentation. Update them as the projects evolve to ensure they remain accurate and useful.