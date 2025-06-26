use std::collections::HashSet;

/// Supported file formats for processing
pub struct SupportedFormats;

impl SupportedFormats {
    /// Check if a file extension is supported
    pub fn is_supported(extension: &str) -> bool {
        let supported_extensions: HashSet<&str> = [
            // Programming languages
            "rs", "py", "java", "cpp", "c", "cs", "js", "ts", "php", "rb", "go", "swift", "kt",
            // Web technologies
            "html", "css", "jsx", "tsx", "vue", "svelte",
            // Configuration and documentation
            "txt", "md", "json", "xml", "yaml", "yml", "toml", "ini", "cfg", "conf",
            // Documentation
            "pdf", "docx", "doc", "rtf", // Data files
            "csv", "sql", "sh", "bat", "ps1",
        ]
        .iter()
        .copied()
        .collect();

        supported_extensions.contains(extension)
    }

    /// Get all supported extensions as a vector
    pub fn get_all_supported() -> Vec<&'static str> {
        vec![
            "rs", "py", "java", "cpp", "c", "cs", "js", "ts", "php", "rb", "go", "swift", "kt",
            "html", "css", "jsx", "tsx", "vue", "svelte", "txt", "md", "json", "xml", "yaml",
            "yml", "toml", "ini", "cfg", "conf", "pdf", "docx", "doc", "rtf", "csv", "sql", "sh",
            "bat", "ps1",
        ]
    }

    /// Get supported extensions by category
    pub fn extensions_by_category() -> std::collections::HashMap<&'static str, Vec<&'static str>> {
        let mut categories = std::collections::HashMap::new();
        categories.insert(
            "source_code",
            vec![
                "rs", "py", "java", "cpp", "c", "cs", "js", "ts", "php", "rb", "go", "swift", "kt",
            ],
        );
        categories.insert("text", vec!["txt", "md"]);
        categories.insert("documents", vec!["pdf", "docx", "doc", "rtf"]);
        categories.insert("data_files", vec!["csv", "sql", "sh", "bat", "ps1"]);
        categories.insert(
            "web_technologies",
            vec!["html", "css", "jsx", "tsx", "vue", "svelte"],
        );
        categories.insert(
            "configuration_and_documentation",
            vec!["json", "xml", "yaml", "yml", "toml", "ini", "cfg", "conf"],
        );
        categories
    }
}
