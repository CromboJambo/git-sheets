#!/usr/bin/env nu

# repo-lookup.nu
# Look up repository information from the cargo metadata index

def "main" [repo_name: string] {
    let index_file = "data/repo-index.json"
    
    if not ($index_file | path exists) {
        $"Error: Index file not found at {$index_file}"
        $"Run 'cargo-metadata-extractor.nu' first to create the index."
        return
    }
    
    try {
        let index = open $index_file | from json
        
        match ($index | where name == $repo_name) {
            $repo {
                $"📋 Repository: {$repo.name} ({$repo.version})"
                $"📝 Description: {$repo.description | default 'No description'}"
                $"🔧 Path: {$repo.path | default 'Not specified'}"
                $"📦 Dependencies: {$($repo.dependencies | length)}"
                $"🎯 Targets: {$($repo.targets | length)}"
                
                if ($repo.dependencies | length) > 0 {
                    $"  Direct dependencies:"
                    $repo.dependencies | default {} | each { |dep|
                        $"    - {$dep.name}: {$dep.version}"
                    }
                }
            }
            _ {
                $"❌ Repository not found: {$repo_name}"
            }
        }
    } catch { |e|
        $"Error reading index: {$e.msg}"
    }
}
