#!/usr/bin/env nu

# repo-list.nu
# List all indexed repositories

def "main" [] {
    let index_file = "data/repo-index.json"
    
    if not ($index_file | path exists) {
        $"Error: Index file not found at {$index_file}"
        $"Run 'cargo-metadata-extractor.nu' first to create the index."
        return
    }
    
    try {
        let index = open $index_file | from json
        
        $"📋 Indexed Repositories:"
        $""
        $"Total: {$($index | length)}"
        $""
        
        $index | each { |repo|
            $"  • {$repo.name} ({$repo.version})"
            $"    Path: {$repo.path | default 'Not specified'}"
            $"    Description: {$repo.description | default 'No description'}"
        }
    } catch { |e|
        $"Error reading index: {$e.msg}"
    }
}
