/// Supported file formats for processing
pub struct SupportedFormats;

impl SupportedFormats {
    /// Check if a file extension is supported
    pub fn is_supported(extension: &str) -> bool {
        matches!(extension, "rs" | "py" | "java" | "txt" | "pdf" | "docx")
    }

    /// Get all supported extensions
    pub fn all_extensions() -> Vec<&'static str> {
        vec!["rs", "py", "java", "txt", "pdf", "docx"]
    }

    /// Get supported extensions by category
    pub fn extensions_by_category() -> std::collections::HashMap<&'static str, Vec<&'static str>> {
        let mut categories = std::collections::HashMap::new();
        categories.insert("source_code", vec!["rs", "py", "java"]);
        categories.insert("text", vec!["txt"]);
        categories.insert("documents", vec!["pdf", "docx"]);
        categories
    }
}
