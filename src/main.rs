use clap::Parser;
use env_guard::cli::{Cli, Commands};
use env_guard::git::get_staged_changes;
use env_guard::scanner::FoundSecret;
use env_guard::scanner::{Scanner, ScannerConfig};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::Scan { entropy_threshold } => run_scan(entropy_threshold),
        Commands::InstallHook => run_install_hook(),
    }
}

fn run_scan(entropy_threshold: f64) -> ExitCode {
    // 1. Fetch staged git changes
    let staged_files = match get_staged_changes() {
        Ok(files) => files,
        Err(err) => {
            eprintln!("Error reading git staged changes: {}", err);
            return ExitCode::FAILURE;
        }
    };

    // 2. Configure and run scanner
    let mut config = ScannerConfig::default();
    config.entropy_threshold = entropy_threshold;

    let scanner = match Scanner::new(config) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error initializing scanner: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let findings = scanner.scan(&staged_files);

    // 3. Evaluate results and set ExitCode
    if findings.is_empty() {
        // Minimal/silent output on success
        println!("EnvGuard: No secrets detected in staged changes.");
        return ExitCode::SUCCESS;
    } else {
        print_findings(&findings);
        return ExitCode::FAILURE;
    }
}
fn print_findings(findings: &[FoundSecret]) {
    eprintln!("\x1b[1;31m[!] EnvGuard detected potential secret(s) in staged changes:\x1b[0m\n");

    for finding in findings {
        eprintln!(
            "  \x1b[1mFile:\x1b[0m {} (Line {})",
            finding.file_path, finding.line_number
        );
        eprintln!("  \x1b[1mDetector:\x1b[0m {}", finding.detector);
        eprintln!("  \x1b[1mLine:\x1b[0m {}", finding.line_content.trim());
        eprintln!("  \x1b[33mTo ignore this line, append '// envguard:ignore' to it.\x1b[0m\n");
    }

    eprintln!("\x1b[1;31mCommit aborted. Please remove secrets before committing.\x1b[0m");
}

fn run_install_hook() -> ExitCode {
    // TODO: We will implement Git hook installation next!
    todo!()
}
