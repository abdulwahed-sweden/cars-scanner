use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use csv::ReaderBuilder;

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
}

// Display error information
fn display_error(error: &ErrorCode) {
    println!("================================");
    println!("Error Code: {}", error.code);
    println!("Description: {}", error.description);
    println!("Severity: {}", error.severity);
    println!("System: {}", error.system);
    
    println!("\nPossible Causes:");
    for cause in error.possible_causes.split('|') {
        println!("  - {}", cause.trim());
    }
    
    println!("\nRecommended Actions:");
    for action in error.recommended_actions.split('|') {
        println!("  - {}", action.trim());
    }
    println!("================================\n");
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
        println!("Error: Could not find error codes database at {}", csv_file);
        println!("Please make sure the file exists in the correct location.");
        return Ok(());
    }
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Execute the appropriate command
    match &cli.command {
        Commands::Lookup { code } => {
            match db.lookup_error(code) {
                Some(error) => {
                    display_error(error);
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
    }
    
    Ok(())
}