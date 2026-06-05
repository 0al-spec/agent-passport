use std::path::PathBuf;
use std::process::ExitCode;

use agent_passport::{validate_file, CheckOptions, IntegrityMode};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "agent-passport")]
#[command(about = "Validate Agent Passport YAML documents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Validate one or more passport files.
    Validate(ValidateArgs),
}

#[derive(Debug, Parser)]
struct ValidateArgs {
    /// Passport YAML file(s) to validate.
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Verify agentIntegrity.codeHashes against local files.
    #[arg(long)]
    check_integrity: bool,

    /// Base directory for relative agentIntegrity sourceFile paths.
    #[arg(long, value_name = "DIR", default_value = ".")]
    integrity_root: PathBuf,

    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => run_validate(args),
    }
}

fn run_validate(args: ValidateArgs) -> ExitCode {
    let options = CheckOptions {
        integrity: if args.check_integrity {
            IntegrityMode::VerifyFiles {
                root: args.integrity_root,
            }
        } else {
            IntegrityMode::StructureOnly
        },
    };

    let mut reports = Vec::new();
    for path in &args.paths {
        reports.push(validate_file(path, &options));
    }

    if args.json {
        match serde_json::to_string_pretty(&reports) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("failed to serialize validation report: {error}");
                return ExitCode::from(2);
            }
        }
    } else {
        print_human_reports(&reports);
    }

    if reports.iter().all(|report| report.valid) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn print_human_reports(reports: &[agent_passport::ValidationReport]) {
    for report in reports {
        if report.valid {
            println!("OK {}", report.path);
        } else {
            println!("INVALID {}", report.path);
        }

        for check in &report.checks {
            println!("  {} {}: {}", check.severity, check.path, check.message);
        }
    }
}
