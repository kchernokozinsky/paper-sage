# Paper Sage - AI-Powered Student Submission Grader

A Rust command-line application that automatically grades student programming submissions using AI models. Supports multiple file formats and can work with both OpenAI and local LLMs like Ollama.

## Features

- **Multi-format Support**: Reads `.rs`, `.py`, `.java`, `.txt`, `.pdf`, and `.docx` files
- **AI-Powered Grading**: Uses OpenAI GPT-4 or local models via Ollama
- **Comprehensive Evaluation**: Grades correctness, style, and edge case handling
- **CSV Reports**: Generates formatted CSV reports with all grading results
- **Resume Capability**: Can resume interrupted grading sessions
- **Flexible Configuration**: JSON-based task description and grading criteria
- **Robust Error Handling**: Gracefully handles file read errors, API issues, and malformed responses

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd paper-sage
```

2. Build the application:
```bash
cargo build --release
```

3. Set up your API key (for OpenAI):
```bash
export OPENAI_API_KEY="your-openai-api-key-here"
```

## Usage

### Basic Usage

```bash
# Grade submissions using OpenAI
cargo run -- --input sample_submissions --config sample_config.json

# Use a local Ollama model
cargo run -- --input sample_submissions --config sample_config.json --model-endpoint http://localhost:11434

# Resume from a previous run
cargo run -- --input sample_submissions --config sample_config.json --resume results.json
```

### Command Line Arguments

- `--input` / `-i`: Path to folder containing student submissions (required)
- `--config` / `-c`: Path to JSON configuration file (required)
- `--model-endpoint` / `-m`: Base URL of AI model endpoint (optional, defaults to OpenAI)
- `--resume` / `-r`: Path to JSON file to resume from (optional)

### Configuration File Format

Create a JSON file with the following structure:

```json
{
  "task_description": "Implement a function that calculates the factorial of a given number.",
  "evaluation_criteria": [
    "Correctness: The function should correctly calculate factorial for valid inputs",
    "Style: Code should be well-formatted and readable",
    "Edge Cases: Should handle negative numbers and zero appropriately"
  ],
  "teacher_comment": "Focus on both correctness and code quality.",
  "grading_strategy": {
    "correctness_weight": 0.5,
    "style_weight": 0.3,
    "edge_cases_weight": 0.2
  }
}
```

## Supported File Formats

- **Source Code**: `.rs`, `.py`, `.java`, `.txt`
- **Documents**: `.pdf`, `.docx`
- **Plain Text**: `.txt`

## AI Model Integration

### OpenAI
- Default endpoint: `https://api.openai.com/v1/chat/completions`
- Requires `OPENAI_API_KEY` environment variable
- Uses GPT-4 model

### Ollama (Local)
- Endpoint format: `http://localhost:11434`
- Uses the `llama2` model by default
- No API key required

### Custom Models
You can use any model that supports either:
- OpenAI-compatible API (`/v1/chat/completions`)
- Ollama-compatible API (`/api/generate`)

## Output Files

The application generates two output files:

1. **`results.csv`**: CSV report with columns:
   - Filename
   - Correctness (0-100)
   - Style (0-100)
   - EdgeCases (0-100)
   - Total (weighted average)
   - Comment (detailed feedback)

2. **`results.json`**: JSON file for resuming interrupted sessions

## Example Output

The CSV report includes:
- All grading results in a structured format
- Properly escaped comments
- Scores with 2 decimal places
- Error messages for failed submissions

## Error Handling

The application handles various error scenarios:
- Missing or unreadable files
- Unsupported file formats
- API connection issues
- Malformed JSON responses
- Invalid configuration files

Failed submissions are included in the results with zero scores and error messages in the comment field.

## Testing

The application includes sample data for testing:

```bash
# Test with sample submissions
cargo run -- --input sample_submissions --config sample_config.json
```

This will process the sample files and demonstrate the application's functionality.

## Development

### Building from Source

```bash
cargo build
cargo test
```

### Dependencies

- **walkdir**: Recursive directory traversal
- **pdf-extract**: PDF text extraction
- **docx-rs**: Word document parsing
- **reqwest**: HTTP client for API calls
- **serde**: JSON serialization/deserialization
- **clap**: Command-line argument parsing
- **tracing**: Logging and progress tracking

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here] 