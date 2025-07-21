pub struct CliBindings;

impl CliBindings {
    #[cfg(feature = "cli")]
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        crate::cli::run_cli().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[cfg(not(feature = "cli"))]
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        Err("CLI feature is not enabled. Please compile with --features cli".into())
    }
}
