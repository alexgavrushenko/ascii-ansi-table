#[cfg(feature = "cli")]
use ascii_ansi_table::cli::run_cli;

#[cfg(feature = "cli")]
fn main() {
    if let Err(e) = run_cli() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI feature is not enabled. Please compile with --features cli");
    std::process::exit(1);
}
