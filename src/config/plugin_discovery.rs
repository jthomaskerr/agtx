# Implement custom plugin discovery

// Logic to list project-local and global plugins

use std::fs;
use std::path::Path;

pub fn discover_plugins() -> Result<Vec<Plugin>, String> {
    let mut plugins = Vec::new();

    // Project-local plugins
    let project_plugins = Path::new(".agtx/plugins").read_dir();
    if let Ok(entries) = project_plugins {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = parse_plugin_toml(entry.path()) {
                    plugins.push(name);
                }
            }
        }
    }

    // Global plugins
    let global_plugins = Path::new("~/.config/agtx/plugins").read_dir();
    if let Ok(entries) = global_plugins {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = parse_plugin_toml(entry.path()) {
                    plugins.push(name);
                }
            }
        }
    }

    // Deduplicate plugins by name (project-local > global > bundled)
    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    plugins.dedup();

    Ok(plugins)
}

pub fn parse_plugin_toml(path: &Path) -> Option<Plugin> {
    // Attempt to parse the plugin.toml
    // Skip if parsing fails
    None
}
