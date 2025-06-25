use anyhow::{Context, Result};
use std::path::Path;

/// Read text-based files (source code and plain text)
pub fn read_text_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read text file: {}", path.display()))
}
