# Paper Sage - AI-Powered Student Submission Grader

Paper Sage is a Rust-based command-line application that automatically grades student programming submissions using AI models. It supports multiple file formats, provides detailed feedback, and generates comprehensive reports.

## Features

- **Multi-format Support**: Handles `.rs`, `.py`, `.java`, `.txt`, `.pdf`, `.docx` files
- **AI-Powered Grading**: Uses OpenAI or Ollama models for intelligent evaluation
- **Flexible Configuration**: JSON-based grading criteria and task descriptions
- **Comprehensive Reports**: Generates both JSON and CSV output files
- **Error Handling**: Graceful fallback to mock responses when AI is unavailable
- **Docker Support (Ollama only)**: Run Ollama in Docker for local AI; Paper Sage runs natively

## Quick Start

### Prerequisites

- Rust 1.70+
- Docker & Docker Compose (for Ollama setup)

### Local Development

1. **Clone and build:**
   ```bash
   git clone <your-repo>
   cd paper-sage
   cargo build --release
   ```

2. **Create sample submissions:**
   ```bash
   mkdir -p submissions/student1 submissions/student2
   # Add your student files to these directories
   ```

3. **Configure grading criteria:**
   ```json
   {
     "task_description": "Implement a function that demonstrates good programming practices. The function should be well-documented, handle edge cases appropriately, and follow coding standards.",
     "evaluation_criteria": [
       "Code Quality: Code is well-structured, readable, and follows best practices",
       "Functionality: Code correctly implements the required functionality",
       "Documentation: Code is properly documented with comments and docstrings",
       "Testing: Code includes appropriate test cases",
       "Edge Cases: Code handles boundary conditions and error cases properly"
     ],
     "teacher_comment": "Please evaluate the code based on the criteria above. Focus on both correctness and code quality. Provide constructive feedback that will help the student improve.",
     "grading_strategy": {
       "correctness_weight": 0.5,
       "style_weight": 0.3,
       "edge_cases_weight": 0.2
     }
   }
   ```

4. **Run grading:**
   ```bash
   # With OpenAI (requires API key and access to gpt-3.5-turbo)
   export OPENAI_API_KEY="your-api-key"
   ./target/release/paper-sage --input submissions --config config.json --model-endpoint https://api.openai.com/v1/chat/completions

   # With Ollama (local AI)
   ./target/release/paper-sage --input submissions --config config.json --model-endpoint http://localhost:11434

   # Test with sample data
   ./target/release/paper-sage --input test/sample_submissions --config test/sample_config.json --model-endpoint http://localhost:11434
   ```

> **Note:**
> - The default OpenAI model is `gpt-3.5-turbo`. Make sure your API key has access and sufficient quota.
> - If you hit a rate limit, Paper Sage will automatically retry up to 3 times with a 20s delay.
> - If all retries fail or the API is unavailable, the app will use mock grading for that submission.

### Docker Setup (Ollama only)

1. **Start Ollama service:**
   ```bash
   docker-compose up -d ollama
   ```

2. **Pull AI models:**
   ```bash
   docker exec -it ollama ollama pull qwen2.5:0.5b
   ```

3. **Run Paper Sage locally:**
   ```bash
   ./target/release/paper-sage --input test/sample_submissions --config test/sample_config.json --model-endpoint http://localhost:11434
   ```

## Configuration

### Grading Strategy

The system uses a configurable weighted scoring formula defined in your grading configuration:
```
Total Score = correctness × correctness_weight + style × style_weight + edge_cases × edge_cases_weight
```

The weights are set in your JSON config file and must sum to 1.0. For example:
```json
"grading_strategy": {
  "correctness_weight": 0.5,
  "style_weight": 0.3,
  "edge_cases_weight": 0.2
}
```

### Supported File Formats

Paper Sage supports a wide range of file formats organized by category:

**Programming Languages:**
- `.rs`, `.py`, `.java`, `.cpp`, `.c`, `.cs`, `.js`, `.ts`, `.php`, `.rb`, `.go`, `.swift`, `.kt`

**Web Technologies:**
- `.html`, `.css`, `.jsx`, `.tsx`, `.vue`, `.svelte`

**Configuration & Documentation:**
- `.txt`, `.md`, `.json`, `.xml`, `.yaml`, `.yml`, `.toml`, `.ini`, `.cfg`, `.conf`

**Documents:**
- `.pdf`, `.docx`, `.doc`, `.rtf`

**Data & Scripts:**
- `.csv`, `.sql`, `.sh`, `.bat`, `.ps1`

### AI Model Integration

- **OpenAI**: Uses `gpt-3.5-turbo` by default (make sure your API key has access and sufficient quota)
- **Ollama**: Local models (llama2, qwen2.5, etc.)
- **Fallback**: Mock responses when AI is unavailable or all retries fail

#### Rate Limit Handling
- If the OpenAI API returns a rate limit error, Paper Sage will automatically wait 20 seconds and retry (up to 3 times).
- If all retries fail, the error will be reported and the submission will not be graded.

#### Quota Handling
- If you exceed your OpenAI quota, grading will fail until you add credits or upgrade your plan.

## Output

### JSON Results (`results.json`)
```json
[
  {
    "filename": "student1/main.py",
    "correctness": 85.0,
    "style": 90.0,
    "edge_cases": 75.0,
    "total": 84.0,
    "comment": "Excellent implementation with good documentation..."
  }
]
```

### CSV Results (`results.csv`)
```csv
Filename,Correctness,Style,EdgeCases,Total,Comment
"student1/main.py",85.00,90.00,75.00,84.00,"Excellent implementation..."
```

## Project Structure

```
paper-sage/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── config.rs            # Configuration parsing
│   ├── models.rs            # Data structures
│   ├── file_processor/      # File format handlers
│   ├── grader/              # AI grading engine
│   └── excel_generator.rs   # Report generation
├── test/
│   ├── sample_submissions/  # Sample student submissions
│   └── sample_config.json   # Sample grading configuration
├── docker-compose.yml       # Ollama setup only
```

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Running with different configurations
```bash
# Custom input folder
./target/release/paper-sage --input /path/to/submissions --config my_config.json

# Different AI endpoint
./target/release/paper-sage --model-endpoint https://api.openai.com/v1/chat/completions

# Test with sample data
./target/release/paper-sage --input test/sample_submissions --config test/sample_config.json
```

## Troubleshooting

### OpenAI API Issues
- **Model not found**: Make sure your API key has access to `gpt-3.5-turbo` (or the model you specify in the code).
- **Quota exceeded**: Add credits or upgrade your OpenAI plan at https://platform.openai.com/account/billing
- **Rate limit exceeded**: The app will retry automatically, but you may need to wait or upgrade your plan for higher throughput.
- **Fallback mode**: If the API is unavailable or all retries fail, the system uses mock responses for demonstration.

### Ollama Issues
- **Slow responses**: Models may be too large for available memory
- **Timeouts**: Increase timeout in `src/grader/ai_client.rs` or your config
- **Memory issues**: Increase Docker memory limits in `docker-compose.yml`

### File Processing Issues
- **Unsupported formats**: Check `src/file_processor/supported_formats.rs`
- **Permission errors**: Ensure read access to submission files

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

[Your License Here]

## Acknowledgments

- Built with Rust for performance and reliability
- Uses Ollama for local AI inference
- Supports OpenAI API for cloud-based AI
- Inspired by the need for automated programming assignment grading 