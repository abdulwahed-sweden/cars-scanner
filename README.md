Here's the complete, reformatted `README.md` in English as a single code block for easy copying:

```markdown
# Car Diagnostic Scanner

A comprehensive Rust-based tool for diagnosing vehicle error codes, designed for both mechanics and car owners. Provides detailed code explanations, severity assessment, and repair recommendations.

## Features

### Core Functionality
- **Multi-Platform Support**: Works on Windows, macOS, and Linux
- **Dual Interface**:
  - Graphical User Interface (GUI)
  - Command Line Interface (CLI)
- **Comprehensive Database**: 5000+ standardized error codes

### Diagnostic Capabilities
- **Error Code Analysis**:
  - Detailed code descriptions
  - Color-coded severity indicators (Low, Medium, High, Critical)
  - Affected vehicle systems
- **Repair Guidance**:
  - Probable causes with likelihood percentages
  - Step-by-step diagnostic procedures
  - Recommended repair actions with difficulty ratings

### Advanced Features
- **AI-Powered Insights**:
  - Simplified explanations for beginners
  - Technical details for professionals
  - Multilingual support (English/Swedish)
- **Smart Search**:
  - Code lookup
  - System-based filtering
  - Severity level filtering
  - Keyword search

### Output Options
- **Report Generation**:
  - HTML with full styling
  - Plain text
  - JSON (for API integration)
- **Export Formats**:
  - Printable PDF
  - Shareable links

## Installation

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation Steps
```bash
git clone https://github.com/abdulwahed-sweden/cars-scanner.git
cd cars-scanner
cargo build --release
```

The compiled binary will be available at:  
`target/release/cars-scanner`

## Usage

### Graphical Interface
```bash
cargo run --release -- gui
```

**Workflow**:
1. Connection method selection (manual/auto)
2. Vehicle scanning
3. Error code review
4. Detailed diagnostics
5. Report generation

### Command Line Interface

#### Basic Operations
```bash
# Look up specific error code
cargo run --release -- lookup P0300

# Export report
cargo run --release -- lookup P0300 --format html --output report.html

# List system-specific codes
cargo run --release -- list --system Engine

# Filter by severity
cargo run --release -- list --severity High
```

#### Advanced Search
```bash
# Keyword search
cargo run --release -- search "misfire"

# Interactive mode
cargo run --release -- interactive
```

## Database Coverage

The tool includes comprehensive error code databases:

| Category | Coverage | Examples |
|----------|----------|----------|
| Powertrain (P) | 3000+ codes | P0300, P0171 |
| Chassis (C) | 800+ codes | C1201, C0034 |
| Body (B) | 600+ codes | B1234, B1000 |
| Network (U) | 400+ codes | U0121, U0140 |

## AI Integration

Powered by DeepSeek API, the AI features provide:

- **Multi-level explanations**:
  - Beginner-friendly summaries
  - Mechanic-level technical details
- **Multilingual support**:
  - English (default)
  - Swedish (`--lang sv`)
- **Smart suggestions**:
  - Common misdiagnoses to avoid
  - Alternative solutions

## Roadmap

### Upcoming Features
- Direct OBD-II scanner integration (USB/Bluetooth)
- Real-time vehicle monitoring
- Expanded multilingual support (Arabic, Spanish)
- Mobile companion app

### Planned Improvements
- Enhanced AI diagnostics
- Community-contributed repair solutions
- Vehicle-specific repair manuals

## License

MIT License

## Contributing

We welcome contributions! Please see our Contribution Guidelines.

## Support

For issues and feature requests, please open an issue on our GitHub repository.
```

You can copy this entire block and paste it directly into your `README.md` file. The formatting is preserved with:
- Proper section headers
- Clean code blocks
- Organized lists and tables
- Consistent markdown syntax
- All English text as requested

The document maintains all the key information from your original README while being better structured and more professional.