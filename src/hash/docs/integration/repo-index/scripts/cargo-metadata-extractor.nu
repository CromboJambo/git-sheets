#!/usr/bin/env nu

# cargo-metadata-extractor.nu
# Extracts and indexes cargo metadata from all repositories recursively

def "main" [] {
    let output_dir = "repo-index/data"
    
    # Create output directory if it doesn't exist
    mkdir $output_dir
    
    # Find all Cargo.toml files recursively
    let cargo_toml_files = glob "**/Cargo.toml"
    
    if ($cargo_toml_files | length) == 0 {
        $"No Cargo.toml files found in current directory or subdirectories."
        return
    }
    
    # Process each Cargo.toml file
    let results = $cargo_toml_files | each { |toml_path|
        # Extract repo name from path
        let parts = $toml_path | path split
        let repo_name = $parts | reverse | first | path split | first
        let metadata_file = $"{$output_dir}/{repo_name}-metadata.json"
        
        $"Extracting metadata from: {$repo_name}..."
        
        # Run cargo metadata and save to file
        try {
            cargo metadata --format-version 1 | save -f $metadata_file
            { success: true, repo: $repo_name }
        } catch { |e|
            { success: false, repo: $repo_name, error: $e.msg }
        }
    }
    
    # Create a consolidated index
    let index_file = $"{$output_dir}/repo-index.json"
    
    # Build index from successful extractions
    let successful = $results | where success == true
    
    $successful | each { |result|
        open $"{$output_dir}/{result.repo}-metadata.json"
    } | default {} | to nuon | save -f $index_file
    
    let failed = $results | where success == false
    
    let successful_count = $successful | length
    $"✅ Successfully indexed {$successful_count} repositories."
    
    if ($failed | length) > 0 {
        $"❌ Failed to index {$($failed | length)} repositories:"
        $failed | each { |result|
            $"   - {$result.repo}: {$result.error}"
        }
    }
    
    $"📊 Index saved to: {$index_file}"
    
    # Show summary
    $"📋 Summary:"
    $"   - Total repositories: {$($cargo_toml_files | length)}"
    $"   - Successful: {$successful_count}"
    $"   - Failed: {$($failed | length)}"
    $"   - Index file: {$index_file}"
}
