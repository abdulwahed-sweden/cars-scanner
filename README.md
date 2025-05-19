# Car Diagnostic Scanner

A command-line tool built in Rust for diagnosing car error codes. This tool helps mechanics and car owners understand error codes, their severity, possible causes, and recommended actions.

## Features

- Look up specific error codes
- List errors by system (Engine, Transmission, ABS, etc.)
- List errors by severity (Low, Medium, High, Critical)
- Detailed information about each error code
- Fast and efficient parsing of error codes

## Installation

Make sure you have Rust and Cargo installed, then:

```bash
git clone https://github.com/abdulwahed-sweden/cars-scanner.git
cd cars-scanner
cargo build --release