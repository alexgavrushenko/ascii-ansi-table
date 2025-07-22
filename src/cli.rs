use crate::table;
use crate::types::{Row, TableError, TableResult, TableUserConfig};
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
#[cfg(feature = "cli")]
use serde_json;
use std::fs;
use std::io::{self, Read, Write};

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "table")]
#[command(about = "A CLI for generating formatted tables")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
pub enum Commands {
    Generate {
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long, default_value = "honeywell")]
        border: String,

        #[arg(long)]
        alignment: Option<String>,

        #[arg(long)]
        single_line: bool,

        #[arg(short, long)]
        config: Option<String>,

        #[arg(long)]
        pretty: bool,
    },

    Validate {
        #[arg(short, long)]
        config: String,
    },

    Borders,

    StreamDemo {
        #[arg(short, long, default_value = "10")]
        rows: usize,

        #[arg(short, long, default_value = "1000")]
        delay: u64,

        #[arg(short, long, default_value = "honeywell")]
        border: String,

        #[arg(long)]
        colors: bool,

        #[arg(long)]
        widths: Option<String>,
    },

    Demo {
        #[arg(short, long, default_value = "10")]
        rows: usize,

        #[arg(short, long, default_value = "honeywell")]
        border: String,

        #[arg(long)]
        colors: bool,

        #[arg(long)]
        widths: Option<String>,
    },
}

#[cfg(feature = "cli")]
pub fn run_cli() -> TableResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            output,
            border,
            alignment,
            single_line,
            config,
            pretty,
        } => generate_table(
            input,
            output,
            border,
            alignment,
            single_line,
            config,
            pretty,
        ),
        Commands::Validate { config } => validate_config(config),
        Commands::Borders => list_borders(),
        Commands::StreamDemo {
            rows,
            delay,
            border,
            colors,
            widths,
        } => stream_demo(rows, delay, border, colors, widths),
        Commands::Demo {
            rows,
            border,
            colors,
            widths,
        } => table_demo(rows, border, colors, widths),
    }
}

#[cfg(feature = "cli")]
fn generate_table(
    input: Option<String>,
    output: Option<String>,
    border: String,
    alignment: Option<String>,
    single_line: bool,
    config_path: Option<String>,
    _pretty: bool,
) -> TableResult<()> {
    let input_data = read_input_data(input)?;
    let table_data: Vec<Row> = serde_json::from_str(&input_data)
        .map_err(|e| TableError::InvalidConfig(format!("Invalid JSON input: {e}")))?;

    let mut config = if let Some(config_path) = config_path {
        read_config_file(config_path)?
    } else {
        TableUserConfig {
            border: None,
            columns: None,
            column_default: None,
            single_line: None,
            spanning_cells: None,
            header: None,
        }
    };

    if border != "honeywell" {
        let border_config = crate::get_border_characters(&border)?;
        config.border = Some(crate::types::BorderUserConfig {
            top_body: Some(border_config.top_body),
            top_join: Some(border_config.top_join),
            top_left: Some(border_config.top_left),
            top_right: Some(border_config.top_right),
            bottom_body: Some(border_config.bottom_body),
            bottom_join: Some(border_config.bottom_join),
            bottom_left: Some(border_config.bottom_left),
            bottom_right: Some(border_config.bottom_right),
            body_left: Some(border_config.body_left),
            body_right: Some(border_config.body_right),
            body_join: Some(border_config.body_join),
            header_join: Some(border_config.header_join),
            join_body: Some(border_config.join_body),
            join_left: Some(border_config.join_left),
            join_right: Some(border_config.join_right),
            join_join: Some(border_config.join_join),
        });
    }

    if let Some(alignment) = alignment {
        let align: crate::types::Alignment = alignment.parse()?;
        if config.column_default.is_none() {
            config.column_default = Some(crate::types::ColumnUserConfig {
                alignment: Some(align),
                vertical_alignment: None,
                padding_left: None,
                padding_right: None,
                truncate: None,
                wrap_word: None,
                width: None,
            });
        } else {
            config.column_default.as_mut().unwrap().alignment = Some(align);
        }
    }

    config.single_line = Some(single_line);

    let table_output = table(&table_data, Some(&config))?;

    write_output(output, &table_output)?;

    Ok(())
}

#[cfg(feature = "cli")]
fn validate_config(config_path: String) -> TableResult<()> {
    let config = read_config_file(config_path)?;

    let full_config = config.merge_with_default(&crate::types::TableConfig::default());
    crate::core::validator::validate_config(&full_config)?;

    println!("Configuration is valid!");
    Ok(())
}

#[cfg(feature = "cli")]
fn list_borders() -> TableResult<()> {
    let borders = ["honeywell", "norc", "ramac", "void"];

    println!("Available border styles:");
    for border in &borders {
        println!("  {border}");

        let border_config = crate::get_border_characters(border)?;
        let example_data = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];

        let config = TableUserConfig {
            border: Some(crate::types::BorderUserConfig {
                top_body: Some(border_config.top_body),
                top_join: Some(border_config.top_join),
                top_left: Some(border_config.top_left),
                top_right: Some(border_config.top_right),
                bottom_body: Some(border_config.bottom_body),
                bottom_join: Some(border_config.bottom_join),
                bottom_left: Some(border_config.bottom_left),
                bottom_right: Some(border_config.bottom_right),
                body_left: Some(border_config.body_left),
                body_right: Some(border_config.body_right),
                body_join: Some(border_config.body_join),
                header_join: Some(border_config.header_join),
                join_body: Some(border_config.join_body),
                join_left: Some(border_config.join_left),
                join_right: Some(border_config.join_right),
                join_join: Some(border_config.join_join),
            }),
            columns: None,
            column_default: None,
            single_line: None,
            spanning_cells: None,
            header: None,
        };

        let example_table = table(&example_data, Some(&config))?;
        println!("{example_table}");
        println!();
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn read_input_data(input: Option<String>) -> TableResult<String> {
    match input {
        Some(path) => fs::read_to_string(path)
            .map_err(|e| TableError::InvalidConfig(format!("Failed to read input file: {e}"))),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).map_err(|e| {
                TableError::InvalidConfig(format!("Failed to read from stdin: {e}"))
            })?;
            Ok(buffer)
        }
    }
}

#[cfg(feature = "cli")]
fn read_config_file(path: String) -> TableResult<TableUserConfig> {
    let content = fs::read_to_string(path)
        .map_err(|e| TableError::InvalidConfig(format!("Failed to read config file: {e}")))?;

    serde_json::from_str(&content)
        .map_err(|e| TableError::InvalidConfig(format!("Invalid JSON in config file: {e}")))
}

#[cfg(feature = "cli")]
fn write_output(output: Option<String>, content: &str) -> TableResult<()> {
    match output {
        Some(path) => fs::write(path, content)
            .map_err(|e| TableError::InvalidConfig(format!("Failed to write output file: {e}"))),
        None => {
            io::stdout().write_all(content.as_bytes()).map_err(|e| {
                TableError::InvalidConfig(format!("Failed to write to stdout: {e}"))
            })?;
            Ok(())
        }
    }
}

// Streaming demo function that displays a table row by row with a delay.
// IMPORTANT: In streaming mode, we only overwrite the bottom border from the previous
// iteration (1 line up), NOT the entire previous record. This allows each row to be
// displayed progressively while replacing only the temporary bottom border with the
// proper join border for the next row.
#[cfg(feature = "cli")]
fn stream_demo(
    rows: usize,
    delay: u64,
    border: String,
    colors: bool,
    widths: Option<String>,
) -> TableResult<()> {
    use std::io::{self, Write};
    use std::thread;
    use std::time::Duration;

    use crate::types::{BorderUserConfig, ColumnUserConfig, StreamUserConfig};

    println!("🚀 ASCII ANSI Table Streaming Demo");
    println!("📊 Streaming {rows} rows with {delay}ms delay (1 row per second)");
    println!("🎨 Border style: {border}");
    if colors {
        println!("🌈 ANSI colors: enabled");
    }
    println!("────────────────────────────────────────");

    let border_config = crate::get_border_characters(&border)?;
    let mut stream_config = StreamUserConfig {
        border: Some(BorderUserConfig {
            top_body: Some(border_config.top_body),
            top_join: Some(border_config.top_join),
            top_left: Some(border_config.top_left),
            top_right: Some(border_config.top_right),
            bottom_body: Some(border_config.bottom_body),
            bottom_join: Some(border_config.bottom_join),
            bottom_left: Some(border_config.bottom_left),
            bottom_right: Some(border_config.bottom_right),
            body_left: Some(border_config.body_left),
            body_right: Some(border_config.body_right),
            body_join: Some(border_config.body_join),
            header_join: Some(border_config.header_join),
            join_body: Some(border_config.join_body),
            join_left: Some(border_config.join_left),
            join_right: Some(border_config.join_right),
            join_join: Some(border_config.join_join),
        }),
        columns: None,
        column_default: None,
        single_line: None,
    };

    let products = [
        "Gaming\nLaptop",
        "Wireless\nMouse",
        "Mechanical\nKeyboard",
        "4K\nMonitor",
        "Bluetooth\nSpeakers",
        "HD\nWebcam",
        "Noise-Cancelling\nHeadphones",
        "Android\nTablet",
        "Smartphone\nPro",
        "Fast\nCharger",
        "USB-C\nCable",
        "Power\nAdapter",
        "WiFi\nRouter",
        "Network\nSwitch",
        "Laser\nPrinter",
    ];
    let statuses = ["Active", "Sold", "Pending", "Shipped", "Delivered"];

    let mut all_sample_data = vec![];

    let header = if colors {
        vec![
            "\u{1b}[1;36mID\u{1b}[0m".to_string(),
            "\u{1b}[1;32mProduct\u{1b}[0m".to_string(),
            "\u{1b}[1;33mPrice\u{1b}[0m".to_string(),
            "\u{1b}[1;35mStatus\u{1b}[0m".to_string(),
        ]
    } else {
        vec![
            "ID".to_string(),
            "Product".to_string(),
            "Price".to_string(),
            "Status".to_string(),
        ]
    };
    all_sample_data.push(header.clone());

    for i in 0..rows {
        let product = products[i % products.len()];
        let status = statuses[i % statuses.len()];
        let price = format!("${}", (i + 1) * 25 + 99);

        let row = if colors {
            vec![
                format!("\u{1b}[37m{}\u{1b}[0m", i + 1),
                format!("\u{1b}[34m{}\u{1b}[0m", product),
                format!("\u{1b}[32m{}\u{1b}[0m", price),
                match status {
                    "Active" => format!("\u{1b}[32m✓ {status}\u{1b}[0m"),
                    "Sold" => format!("\u{1b}[31m✗ {status}\u{1b}[0m"),
                    "Pending" => format!("\u{1b}[33m⚠ {status}\u{1b}[0m"),
                    "Shipped" => format!("\u{1b}[36m🚚 {status}\u{1b}[0m"),
                    "Delivered" => format!("\u{1b}[35m📦 {status}\u{1b}[0m"),
                    _ => status.to_string(),
                },
            ]
        } else {
            vec![
                (i + 1).to_string(),
                product.to_string(),
                price,
                status.to_string(),
            ]
        };
        all_sample_data.push(row);
    }

    if let Some(widths_str) = widths {
        let widths: Result<Vec<usize>, _> = widths_str
            .split(',')
            .map(|w| w.trim().parse::<usize>())
            .collect();

        if let Ok(widths) = widths {
            let column_configs: Vec<ColumnUserConfig> = widths
                .iter()
                .map(|&width| ColumnUserConfig {
                    width: Some(width),
                    ..Default::default()
                })
                .collect();
            stream_config.columns = Some(column_configs);
        }
    } else {
        let temp_config = TableUserConfig {
            border: stream_config.border.clone(),
            columns: None,
            column_default: stream_config.column_default.clone(),
            single_line: stream_config.single_line,
            spanning_cells: None,
            header: None,
        };

        let _temp_table = crate::table(&all_sample_data, Some(&temp_config))?;

        let column_count = all_sample_data.first().map(|row| row.len()).unwrap_or(4);

        let default_widths = [6, 12, 8, 12];

        let column_configs: Vec<ColumnUserConfig> = default_widths
            .iter()
            .take(column_count)
            .map(|&width| ColumnUserConfig {
                width: Some(width),
                ..Default::default()
            })
            .collect();
        stream_config.columns = Some(column_configs);
    }

    use crate::features::streaming::create_string_stream;

    let column_widths: Vec<usize> = stream_config
        .columns
        .as_ref()
        .map(|cols| cols.iter().map(|col| col.width.unwrap_or(10)).collect())
        .unwrap_or_else(|| vec![6, 12, 8, 12]);

    let mut stream = create_string_stream(Some(stream_config));

    let header_output = stream.write_row(&all_sample_data[0])?;
    print!("{header_output}");
    io::stdout()
        .flush()
        .map_err(|e| TableError::InvalidConfig(format!("Failed to flush stdout: {e}")))?;

    let mut previous_output_lines = 0;

    for i in 0..rows {
        thread::sleep(Duration::from_millis(delay));

        if i > 0 {
            // Move up by the number of lines from previous iteration (row content + bottom border)
            for _ in 0..previous_output_lines {
                print!("\u{1b}[1A");
                print!("\u{1b}[0K");
            }
        }

        let row_output = stream.write_row(&all_sample_data[i + 1])?;
        print!("{row_output}");

        use crate::core::renderer::{BorderType, draw_border_line};

        let border_config = crate::get_border_characters(&border)?;
        let bottom_border = draw_border_line(&column_widths, &border_config, BorderType::Bottom);
        println!("{bottom_border}");

        // Count lines in this iteration's output for next iteration
        previous_output_lines = 1; // +1 for bottom border

        io::stdout()
            .flush()
            .map_err(|e| TableError::InvalidConfig(format!("Failed to flush stdout: {e}")))?;
    }

    println!("\n✅ Streaming demo complete! {rows} rows processed.");
    println!("💡 Try different options:");
    println!("   --rows 20 --delay 200 --colors --border ramac");
    println!("   --widths 4,12,8,12");

    Ok(())
}

#[cfg(feature = "cli")]
fn table_demo(
    rows: usize,
    border: String,
    colors: bool,
    widths: Option<String>,
) -> TableResult<()> {
    use crate::types::{ColumnUserConfig, TableUserConfig};

    println!("🚀 ASCII ANSI Table Demo");
    println!("📊 Generating table with {rows} rows");
    println!("🎨 Border style: {border}");
    if colors {
        println!("🌈 ANSI colors: enabled");
    }
    println!("────────────────────────────────────────");

    let statuses = ["Active", "Sold", "Pending", "Shipped", "Delivered"];

    let mut all_sample_data = vec![];

    let header = if colors {
        vec![
            "\u{1b}[1;36mID\u{1b}[0m".to_string(),
            "\u{1b}[1;32mProduct\u{1b}[0m".to_string(),
            "\u{1b}[1;33mPrice\u{1b}[0m".to_string(),
            "\u{1b}[1;35mStatus\u{1b}[0m".to_string(),
        ]
    } else {
        vec![
            "ID".to_string(),
            "Product".to_string(),
            "Price".to_string(),
            "Status".to_string(),
        ]
    };
    all_sample_data.push(header);

    let products = [
        "Gaming\nLaptop",
        "Wireless\nMouse",
        "Mechanical\nKeyboard",
        "4K\nMonitor",
        "Bluetooth\nSpeakers",
        "HD\nWebcam",
        "Noise-Cancelling\nHeadphones",
        "Android\nTablet",
        "Smartphone\nPro",
        "Fast\nCharger",
        "USB-C\nCable",
        "Power\nAdapter",
        "WiFi\nRouter",
        "Network\nSwitch",
        "Laser\nPrinter",
    ];

    for i in 0..rows {
        let product = products[i % products.len()];
        let status = statuses[i % statuses.len()];
        let price = format!("${}", (i + 1) * 25 + 99);

        let row = if colors {
            vec![
                format!("\u{1b}[37m{}\u{1b}[0m", i + 1),
                format!("\u{1b}[34m{}\u{1b}[0m", product),
                format!("\u{1b}[32m{}\u{1b}[0m", price),
                match status {
                    "Active" => format!("\u{1b}[32m✓ {status}\u{1b}[0m"),
                    "Sold" => format!("\u{1b}[31m✗ {status}\u{1b}[0m"),
                    "Pending" => format!("\u{1b}[33m⚠ {status}\u{1b}[0m"),
                    "Shipped" => format!("\u{1b}[36m🚚 {status}\u{1b}[0m"),
                    "Delivered" => format!("\u{1b}[35m📦 {status}\u{1b}[0m"),
                    _ => status.to_string(),
                },
            ]
        } else {
            vec![
                (i + 1).to_string(),
                product.to_string(),
                price,
                status.to_string(),
            ]
        };
        all_sample_data.push(row);
    }

    let border_config = crate::get_border_characters(&border)?;
    let mut config = TableUserConfig {
        border: Some(crate::types::BorderUserConfig {
            top_body: Some(border_config.top_body),
            top_join: Some(border_config.top_join),
            top_left: Some(border_config.top_left),
            top_right: Some(border_config.top_right),
            bottom_body: Some(border_config.bottom_body),
            bottom_join: Some(border_config.bottom_join),
            bottom_left: Some(border_config.bottom_left),
            bottom_right: Some(border_config.bottom_right),
            body_left: Some(border_config.body_left),
            body_right: Some(border_config.body_right),
            body_join: Some(border_config.body_join),
            header_join: Some(border_config.header_join),
            join_body: Some(border_config.join_body),
            join_left: Some(border_config.join_left),
            join_right: Some(border_config.join_right),
            join_join: Some(border_config.join_join),
        }),
        columns: None,
        column_default: None,
        single_line: None,
        spanning_cells: None,
        header: None,
    };

    if let Some(widths_str) = widths {
        let widths: Result<Vec<usize>, _> = widths_str
            .split(',')
            .map(|w| w.trim().parse::<usize>())
            .collect();

        if let Ok(widths) = widths {
            let column_configs: Vec<ColumnUserConfig> = widths
                .iter()
                .map(|&width| ColumnUserConfig {
                    width: Some(width),
                    ..Default::default()
                })
                .collect();
            config.columns = Some(column_configs);
        }
    } else {
        let column_count = all_sample_data.first().map(|row| row.len()).unwrap_or(4);
        let default_widths = [6, 12, 8, 12];

        let column_configs: Vec<ColumnUserConfig> = default_widths
            .iter()
            .take(column_count)
            .map(|&width| ColumnUserConfig {
                width: Some(width),
                ..Default::default()
            })
            .collect();
        config.columns = Some(column_configs);
    }

    let result = crate::table(&all_sample_data, Some(&config))?;
    println!("{result}");

    println!("✅ Table demo complete! {rows} rows generated.");
    println!("💡 Try different options:");
    println!("   --rows 20 --colors --border ramac");
    println!("   --widths 4,12,8,12");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    #[cfg(feature = "cli")]
    fn test_read_config_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config_json = r#"{"single_line": true}"#;
        temp_file.write_all(config_json.as_bytes()).unwrap();

        let config = read_config_file(temp_file.path().to_string_lossy().to_string()).unwrap();
        assert_eq!(config.single_line, Some(true));
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_generate_table_with_config() {
        let table_data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["John".to_string(), "30".to_string()],
        ];

        let config = TableUserConfig {
            border: None,
            columns: None,
            column_default: None,
            single_line: Some(false),
            spanning_cells: None,
            header: None,
        };

        let result = table(&table_data, Some(&config)).unwrap();
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("John"));
        assert!(result.contains("30"));
    }
}
