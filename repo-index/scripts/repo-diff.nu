#!/usr/bin/env nu

# repo-diff.nu
# Compare dependencies between two repositories

def "main" [repo1: string, repo2: string] {
    let index_file = "data/repo-index.json"
    
    if not ($index_file | path exists) {
        $"Error: Index file not found at {$index_file}"
        $"Run 'cargo-metadata-extractor.nu' first to create the index."
        return
    }
    
    try {
        let index = open $index_file | from json
        
        match ($index | where name == $repo1) {
            $r1 {
                match ($index | where name == $repo2) {
                    $r2 {
                        # Convert dependencies to tables for comparison
                        let deps1 = $r1.dependencies | default {} | transpose name version
                        let deps2 = $r2.dependencies | default {} | transpose name version
                        
                        # Find differences
                        let all_deps = $deps1 | merge $deps2
                        let added = $deps2 | where $name not in $deps1
                        let removed = $deps1 | where $name not in $deps2
                        let changed = $deps1 | where $name in $deps2 and $version != $deps2.version
                        
                        $"📊 Dependency Comparison: {$repo1} vs {$repo2}"
                        $""
                        $"📈 Summary:"
                        $"   Total unique dependencies: {$($all_deps | length)}"
                        $"   Added: {$($added | length)}"
                        $"   Removed: {$($removed | length)}"
                        $"   Changed versions: {$($changed | length)}"
                        $""
                        
                        if ($added | length) > 0 {
                            $"📥 Added dependencies:"
                            $added | each { |dep|
                                $"    + {$dep.name} ({$dep.version})"
                            }
                        }
                        
                        if ($removed | length) > 0 {
                            $"📤 Removed dependencies:"
                            $removed | each { |dep|
                                $"    - {$dep.name} ({$dep.version})"
                            }
                        }
                        
                        if ($changed | length) > 0 {
                            $"🔄 Changed versions:"
                            $changed | each { |dep|
                                $"    * {$dep.name}: {$dep.version} → {$deps2.version}"
                            }
                        }
                    }
                    _ {
                        $"❌ Repository not found: {$repo2}"
                    }
                }
            }
            _ {
                $"❌ Repository not found: {$repo1}"
            }
        }
    } catch { |e|
        $"Error reading index: {$e.msg}"
    }
}
