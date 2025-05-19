use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use csv::ReaderBuilder;
use colored::*;

// Define the error code structure
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ErrorCode {
    code: String,
    description: String,
    severity: String,
    system: String,
    possible_causes: String,
    recommended_actions: String,
}

impl ErrorCode {
    fn to_text(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Error Code: {}\n", self.code));
        output.push_str(&format!("Description: {}\n", self.description));
        output.push_str(&format!("Severity: {}\n", self.severity));
        output.push_str(&format!("System: {}\n", self.system));
        
        output.push_str("\nPossible Causes:\n");
        for cause in self.possible_causes.split('|') {
            output.push_str(&format!("  - {}\n", cause.trim()));
        }
        
        output.push_str("\nRecommended Actions:\n");
        for action in self.recommended_actions.split('|') {
            output.push_str(&format!("  - {}\n", action.trim()));
        }
        
        output
    }
    
    fn to_html(&self) -> String {
        let mut output = String::new();
        output.push_str("<div class='error-code'>\n");
        output.push_str(&format!("<h2>Error Code: {}</h2>\n", self.code));
        output.push_str(&format!("<p><strong>Description:</strong> {}</p>\n", self.description));
        output.push_str(&format!("<p><strong>Severity:</strong> {}</p>\n", self.severity));
        output.push_str(&format!("<p><strong>System:</strong> {}</p>\n", self.system));
        
        output.push_str("<h3>Possible Causes:</h3>\n<ul>\n");
        for cause in self.possible_causes.split('|') {
            output.push_str(&format!("<li>{}</li>\n", cause.trim()));
        }
        output.push_str("</ul>\n");
        
        output.push_str("<h3>Recommended Actions:</h3>\n<ul>\n");
        for action in self.recommended_actions.split('|') {
            output.push_str(&format!("<li>{}</li>\n", action.trim()));
        }
        output.push_str("</ul>\n");
        output.push_str("</div>\n");
        
        output
    }
}

// Define the diagnostics database
struct DiagnosticsDatabase {
    errors: HashMap<String, ErrorCode>,
}

impl DiagnosticsDatabase {
    // Create a new database
    fn new() -> Self {
        DiagnosticsDatabase {
            errors: HashMap::new(),
        }
    }

    // Load data from a CSV file
    fn load_from_csv(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = fs::File::open(file_path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        
        for result in reader.deserialize() {
            let record: ErrorCode = result?;
            self.errors.insert(record.code.clone(), record);
        }
        
        println!("Loaded {} error codes from database", self.errors.len());
        Ok(())
    }

    // Look up an error code
    fn lookup_error(&self, code: &str) -> Option<&ErrorCode> {
        self.errors.get(code)
    }
    
    // List errors by system
    fn list_errors_by_system(&self, system: &str) -> Vec<&ErrorCode> {
        self.errors.values()
            .filter(|error| error.system.to_lowercase() == system.to_lowercase())
            .collect()
    }
    
    // List errors by severity
    fn list_errors_by_severity(&self, severity: &str) -> Vec<&ErrorCode> {
        self.errors.values()
            .filter(|error| error.severity.to_lowercase() == severity.to_lowercase())
            .collect()
    }
    
    // Search by keyword
    fn search_by_keyword(&self, keyword: &str) -> Vec<&ErrorCode> {
        let keyword_lower = keyword.to_lowercase();
        self.errors.values()
            .filter(|error| {
                error.description.to_lowercase().contains(&keyword_lower) || 
                error.possible_causes.to_lowercase().contains(&keyword_lower) ||
                error.recommended_actions.to_lowercase().contains(&keyword_lower)
            })
            .collect()
    }
}

// Define the command line interface
#[derive(Parser)]
#[command(author = "Abdul Wahed", version = "1.0", about = "Car Diagnostic Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Command to look up an error code
    Lookup {
        #[arg(short, long)]
        code: String,
        
        #[arg(short, long)]
        export: Option<String>,
    },
    
    // Command to list errors by system
    ListBySystem {
        #[arg(short, long)]
        system: String,
    },
    
    // Command to list errors by severity
    ListBySeverity {
        #[arg(short, long)]
        severity: String,
    },
    
    // Command to search by keyword
    Search {
        #[arg(short, long)]
        keyword: String,
    },
    
    // Command to start interactive mode
    Interactive,
}

// Display error information with color
fn display_error(error: &ErrorCode) {
    println!("{}", "================================".bright_blue());
    println!("{} {}", "Error Code:".bright_yellow(), error.code.bright_white());
    println!("{} {}", "Description:".bright_yellow(), error.description);
    
    // Color code the severity
    let severity_colored = match error.severity.as_str() {
        "Low" => error.severity.bright_green(),
        "Medium" => error.severity.bright_yellow(),
        "High" => error.severity.bright_red(),
        "Critical" => error.severity.on_red().bright_white(),
        _ => error.severity.normal(),
    };
    
    println!("{} {}", "Severity:".bright_yellow(), severity_colored);
    println!("{} {}", "System:".bright_yellow(), error.system.bright_cyan());
    
    println!("\n{}", "Possible Causes:".bright_magenta());
    for cause in error.possible_causes.split('|') {
        println!("  - {}", cause.trim());
    }
    
    println!("\n{}", "Recommended Actions:".bright_green());
    for action in error.recommended_actions.split('|') {
        println!("  - {}", action.trim());
    }
    println!("{}", "================================\n".bright_blue());
}

// Function to export error to file
fn export_to_file(error: &ErrorCode, file_path: &str) -> Result<(), Box<dyn Error>> {
    let content = if file_path.ends_with(".html") {
        // Create an HTML document
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>Car Error Code Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str(".error-code { border: 1px solid #ddd; padding: 15px; margin-bottom: 20px; }\n");
        html.push_str("h2 { color: #d9534f; }\n");
        html.push_str("h3 { color: #5bc0de; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("<h1>Car Error Code Report</h1>\n");
        html.push_str(&error.to_html());
        html.push_str("</body>\n</html>");
        html
    } else {
        // Default to text format
        error.to_text()
    };
    
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    
    println!("Report exported to {}", file_path);
    Ok(())
}

// Run interactive mode
fn run_interactive_mode(db: &DiagnosticsDatabase) -> Result<(), Box<dyn Error>> {
    println!("{}", "=== Car Diagnostic Tool Interactive Mode ===".bright_blue());
    println!("Type '{}' for available commands or '{}' to quit", "help".bright_green(), "exit".bright_red());
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    
    loop {
        print!("{} ", ">".bright_cyan());
        io::stdout().flush()?;
        
        input.clear();
        handle.read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();
        
        match command.as_str() {
            "exit" | "quit" => break,
            
            "help" => {
                println!("{}", "Available commands:".bright_blue());
                println!("  {} {} - Look up details for an error code", "lookup".bright_green(), "<code>".bright_yellow());
                println!("  {} {} - List all errors for a specific system", "system".bright_green(), "<system_name>".bright_yellow());
                println!("  {} {} - List all errors with a specific severity", "severity".bright_green(), "<level>".bright_yellow());
                println!("  {} {} - Search for errors containing a keyword", "search".bright_green(), "<keyword>".bright_yellow());
                println!("  {} - Display this help message", "help".bright_green());
                println!("  {} - Exit the interactive mode", "exit".bright_red());
            },
            
            "lookup" => {
                if parts.len() < 2 {
                    println!("Usage: {} {}", "lookup".bright_green(), "<code>".bright_yellow());
                    continue;
                }
                
                let code = parts[1];
                match db.lookup_error(code) {
                    Some(error) => display_error(error),
                    None => println!("Error code '{}' not found in database", code.bright_red()),
                }
            },
            
            "system" => {
                if parts.len() < 2 {
                    println!("Usage: {} {}", "system".bright_green(), "<system_name>".bright_yellow());
                    continue;
                }
                
                let system = parts[1];
                let errors = db.list_errors_by_system(system);
                if errors.is_empty() {
                    println!("No errors found for system: {}", system.bright_red());
                } else {
                    println!("Found {} errors for system: {}", errors.len().to_string().bright_green(), system.bright_cyan());
                    for error in errors {
                        display_error(error);
                    }
                }
            },
            
            "severity" => {
                if parts.len() < 2 {
                    println!("Usage: {} {}", "severity".bright_green(), "<level>".bright_yellow());
                    continue;
                }
                
                let severity = parts[1];
                let errors = db.list_errors_by_severity(severity);
                if errors.is_empty() {
                    println!("No errors found with severity: {}", severity.bright_red());
                } else {
                    println!("Found {} errors with severity: {}", errors.len().to_string().bright_green(), severity.bright_cyan());
                    for error in errors {
                        display_error(error);
                    }
                }
            },
            
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: {} {}", "search".bright_green(), "<keyword>".bright_yellow());
                    continue;
                }
                
                let keyword = parts[1];
                let errors = db.search_by_keyword(keyword);
                if errors.is_empty() {
                    println!("No errors found containing keyword: {}", keyword.bright_red());
                } else {
                    println!("Found {} errors containing keyword: {}", errors.len().to_string().bright_green(), keyword.bright_cyan());
                    for error in errors {
                        display_error(error);
                    }
                }
            },
            
            _ => println!("{} Type '{}' for available commands.", "Unknown command.".bright_red(), "help".bright_green()),
        }
    }
    
    println!("Exiting interactive mode");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the database
    let mut db = DiagnosticsDatabase::new();
    
    // Define the CSV file path
    let csv_file = "src/data/error_codes.csv";
    
    // Check if the file exists and load it
    if Path::new(csv_file).exists() {
        db.load_from_csv(csv_file)?;
    } else {
        println!("{}: Could not find error codes database at {}", "Error".bright_red(), csv_file);
        println!("Please make sure the file exists in the correct location.");
        return Ok(());
    }
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Execute the appropriate command
    match &cli.command {
        Commands::Lookup { code, export } => {
            match db.lookup_error(code) {
                Some(error) => {
                    display_error(error);
                    
                    if let Some(file_path) = export {
                        if let Err(e) = export_to_file(error, file_path) {
                            eprintln!("{}: Failed to export report: {}", "Error".bright_red(), e);
                        }
                    }
                },
                None => println!("Error code '{}' not found in database", code),
            }
        },
        Commands::ListBySystem { system } => {
            let errors = db.list_errors_by_system(system);
            if errors.is_empty() {
                println!("No errors found for system: {}", system);
            } else {
                println!("Found {} errors for system: {}", errors.len(), system);
                for error in errors {
                    display_error(error);
                }
            }
        },
        Commands::ListBySeverity { severity } => {
            let errors = db.list_errors_by_severity(severity);
            if errors.is_empty() {
                println!("No errors found with severity: {}", severity);
            } else {
                println!("Found {} errors with severity: {}", errors.len(), severity);
                for error in errors {
                    display_error(error);
                }
            }
        },
        Commands::Search { keyword } => {
            let errors = db.search_by_keyword(keyword);
            if errors.is_empty() {
                println!("No errors found containing keyword: {}", keyword);
            } else {
                println!("Found {} errors containing keyword: {}", errors.len(), keyword);
                for error in errors {
                    display_error(error);
                }
            }
        },
        Commands::Interactive => {
            run_interactive_mode(&db)?;
        },
    }
    
    Ok(())
}