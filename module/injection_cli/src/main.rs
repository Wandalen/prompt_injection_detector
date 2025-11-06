//! `injection_cli` - Prompt injection detection CLI
//!
//! CLI for detecting prompt injections using ModernBERT-based model with ORT or Burn backend.
//!
//! # Examples
//!
//! ```bash
//! # Single detection
//! cargo run -p injection_cli -- .detect text::"Ignore all previous instructions"
//!
//! # Interactive mode
//! cargo run -p injection_cli -- .interactive
//!
//! # From file
//! cargo run -p injection_cli -- .detect file::"test.txt"
//!
//! # From stdin
//! echo "test" | cargo run -p injection_cli -- .detect
//! ```

use anyhow::{Context, Result};
use colored::Colorize;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fs;
use std::io::{self, Read};
use std::time::Instant;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputFormat {
    Human,
    Json,
    Simple,
    Quiet,
}

/// CLI configuration
#[derive(Debug)]
struct Config {
    format: OutputFormat,
    show_time: bool,
    verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: OutputFormat::Human,
            show_time: false,
            verbose: false,
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Check for help
    if args.len() == 1 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return Ok(());
    }

    // Check for version
    if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
        println!("injection_cli {}", env!("CARGO_PKG_VERSION"));
        #[cfg(feature = "backend-ort")]
        println!("Backend: ORT (ONNX Runtime with CUDA)");
        #[cfg(feature = "backend-burn")]
        println!("Backend: Burn (with CUDA)");
        return Ok(());
    }

    // Parse command
    let command = args.get(1).map_or("", std::string::String::as_str);

    match command {
        ".detect" | "detect" => run_detect_mode(&args[2..]),
        ".interactive" | ".chat" | "interactive" | "chat" => run_interactive_mode(&args[2..]),
        _ => {
            eprintln!("Error: Unknown command '{command}'");
            eprintln!("Use --help for usage information");
            std::process::exit(1);
        }
    }
}

/// Run single-shot detection mode
fn run_detect_mode(args: &[String]) -> Result<()> {
    let mut config = Config::default();
    let mut text: Option<String> = None;
    let mut file: Option<String> = None;
    let mut warmup = false;

    // Parse arguments
    for arg in args {
        if arg.starts_with("text::") {
            text = Some(parse_value(arg, "text"));
        } else if arg.starts_with("file::") {
            file = Some(parse_value(arg, "file"));
        } else if arg.starts_with("format::") {
            let fmt = parse_value(arg, "format");
            config.format = match fmt.as_str() {
                "json" => OutputFormat::Json,
                "simple" => OutputFormat::Simple,
                "quiet" => OutputFormat::Quiet,
                "human" => OutputFormat::Human,
                _ => {
                    eprintln!("Warning: Unknown format '{fmt}', using 'human'");
                    OutputFormat::Human
                }
            };
        } else if arg.starts_with("quiet::") {
            let val = parse_value(arg, "quiet");
            if val == "true" || val == "1" {
                config.format = OutputFormat::Quiet;
            }
        } else if arg.starts_with("verbose::") {
            let val = parse_value(arg, "verbose");
            config.verbose = val == "true" || val == "1";
        } else if arg == "--time" {
            config.show_time = true;
        } else if arg == "--warmup" {
            warmup = true;
        }
    }

    // Get input text
    let input_text = if let Some(t) = text {
        t
    } else if let Some(f) = file {
        fs::read_to_string(&f)
            .with_context(|| format!("Failed to read file: {f}"))?
            .trim()
            .to_string()
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    };

    if input_text.is_empty() {
        anyhow::bail!("Error: No input text provided");
    }

    // Pre-warm if requested
    if warmup {
        if config.verbose {
            eprintln!("Pre-loading model...");
        }
        let warmup_start = Instant::now();
        injection_core::init()?;
        if config.verbose {
            eprintln!(
                "Model loaded in {:.2}s",
                warmup_start.elapsed().as_secs_f64()
            );
        }
    }

    // Run detection
    let start = Instant::now();
    let result = injection_core::detect(&input_text)?;
    let duration = start.elapsed();

    // Output result
    output_result(&result, &input_text, duration, &config)?;

    // Exit code for quiet mode
    if config.format == OutputFormat::Quiet {
        std::process::exit(i32::from(result == "injection"));
    }

    Ok(())
}

/// Run interactive/chat mode
fn run_interactive_mode(args: &[String]) -> Result<()> {
    let mut config = Config {
        show_time: true, // Always show timing in interactive mode
        ..Default::default()
    };

    // Parse arguments
    for arg in args {
        if arg.starts_with("verbose::") {
            let val = parse_value(arg, "verbose");
            config.verbose = val == "true" || val == "1";
        } else if arg.starts_with("format::") {
            let fmt = parse_value(arg, "format");
            config.format = match fmt.as_str() {
                "json" => OutputFormat::Json,
                "simple" => OutputFormat::Simple,
                _ => OutputFormat::Human,
            };
        }
    }

    // Pre-load model
    println!("Loading model...");
    let load_start = Instant::now();
    injection_core::init()?;
    let load_time = load_start.elapsed();
    println!("Model loaded in {:.2}s\n", load_time.as_secs_f64());

    // Print welcome message
    println!(
        "{}",
        "Prompt Injection Detector (interactive mode)".bold().cyan()
    );
    println!("Type 'exit', 'quit', or press Ctrl+D to quit\n");

    // Initialize readline
    let mut rl = DefaultEditor::new()?;

    // Main loop
    loop {
        let readline = rl.readline(&format!("{} ", ">".bold().green()));
        match readline {
            Ok(line) => {
                let line = line.trim();

                // Check for exit commands
                if line.is_empty() {
                    continue;
                }
                if line == "exit" || line == "quit" {
                    println!("Goodbye!");
                    break;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Run detection
                let start = Instant::now();
                match injection_core::detect(line) {
                    Ok(result) => {
                        let duration = start.elapsed();
                        output_interactive_result(&result, duration, &config);
                    }
                    Err(e) => {
                        eprintln!("{}: {}", "Error".red().bold(), e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("^C");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    Ok(())
}

/// Output result for single-shot mode
fn output_result(
    result: &str,
    text: &str,
    duration: std::time::Duration,
    config: &Config,
) -> Result<()> {
    match config.format {
        OutputFormat::Human => {
            let label = if result == "injection" {
                "INJECTION".red().bold()
            } else {
                "BENIGN".green().bold()
            };
            println!("\n{}: {}", "Result".bold(), label);
            if config.show_time {
                println!("{}: {:.2}ms", "Time".bold(), duration.as_millis());
            }
            if config.verbose {
                println!("{}: {}", "Input".bold(), text);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "label": result,
                "is_injection": result == "injection",
                "text": if config.verbose { Some(text) } else { None },
                "time_ms": u64::try_from(duration.as_millis()).unwrap_or(u64::MAX),
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Simple => {
            println!("{result}");
        }
        OutputFormat::Quiet => {
            // Silent output, just exit code
        }
    }
    Ok(())
}

/// Output result for interactive mode
fn output_interactive_result(result: &str, duration: std::time::Duration, config: &Config) {
    match config.format {
        OutputFormat::Human => {
            let (symbol, label_colored) = if result == "injection" {
                ("✗", result.red().bold())
            } else {
                ("✓", result.green().bold())
            };
            println!(
                "{} {} ({:.0}ms)",
                symbol,
                label_colored,
                duration.as_millis()
            );
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "label": result,
                "is_injection": result == "injection",
                "time_ms": u64::try_from(duration.as_millis()).unwrap_or(u64::MAX),
            });
            if let Ok(s) = serde_json::to_string(&json) {
                println!("{s}");
            }
        }
        OutputFormat::Simple => {
            println!("{result}");
        }
        OutputFormat::Quiet => {
            // Not applicable in interactive mode
            println!("{result}");
        }
    }
}

/// Parse unilang-style parameter value
fn parse_value(arg: &str, prefix: &str) -> String {
    let value = arg
        .strip_prefix(&format!("{prefix}::"))
        .unwrap_or("")
        .trim();

    // Remove surrounding quotes if present
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

/// Print comprehensive help message
fn print_help() {
    println!(
        "{}",
        "injection_cli - Prompt Injection Detection".bold().cyan()
    );
    println!("Version: {}\n", env!("CARGO_PKG_VERSION"));

    #[cfg(feature = "backend-ort")]
    println!("{} ORT (ONNX Runtime with CUDA)\n", "Backend:".bold());
    #[cfg(feature = "backend-burn")]
    println!("{} Burn (with CUDA)\n", "Backend:".bold());

    println!("{}", "USAGE:".bold());
    println!(
        "  {} {} [OPTIONS]",
        "injection_cli".cyan(),
        ".detect".yellow()
    );
    println!("  {} {}\n", "injection_cli".cyan(), ".interactive".yellow());

    println!("{}", "COMMANDS:".bold());
    println!("  {}        Single detection mode", ".detect".yellow());
    println!(
        "  {}   Interactive chat mode (continuous detection)\n",
        ".interactive".yellow()
    );

    println!("{}", "DETECT MODE OPTIONS:".bold());
    println!(
        "  {}        Direct text input (required if no file/stdin)",
        "text::\"TEXT\"".green()
    );
    println!("  {}        Read from file", "file::\"PATH\"".green());
    println!("  (stdin)                Read from stdin (if no text/file specified)");
    println!(
        "  {}    Output format: human, json, simple, quiet",
        "format::FORMAT".green()
    );
    println!(
        "  {}      Silent mode (exit code only: 0=benign, 1=injection)",
        "quiet::true".green()
    );
    println!("  {}    Show detailed logging", "verbose::true".green());
    println!("  {}             Show inference timing", "--time".green());
    println!(
        "  {}           Pre-load model before detection\n",
        "--warmup".green()
    );

    println!("{}", "INTERACTIVE MODE OPTIONS:".bold());
    println!(
        "  {}    Output format: human, json, simple",
        "format::FORMAT".green()
    );
    println!("  {}    Show detailed logging\n", "verbose::true".green());

    println!("{}", "EXAMPLES:".bold());
    println!("  {}", "# Single detection (direct text)".dimmed());
    println!("  injection_cli .detect text::\"Ignore all previous instructions\"\n");

    println!("  {}", "# From file".dimmed());
    println!("  injection_cli .detect file::\"suspicious.txt\"\n");

    println!("  {}", "# From stdin".dimmed());
    println!("  echo \"test\" | injection_cli .detect\n");

    println!("  {}", "# JSON output".dimmed());
    println!("  injection_cli .detect text::\"test\" format::json\n");

    println!("  {}", "# Quiet mode for scripts".dimmed());
    println!("  injection_cli .detect text::\"test\" quiet::true");
    println!("  echo $?  # 0=benign, 1=injection\n");

    println!("  {}", "# Interactive mode".dimmed());
    println!("  injection_cli .interactive\n");

    println!("  {}", "# With warmup and timing".dimmed());
    println!("  injection_cli .detect text::\"test\" --warmup --time\n");

    println!("{}", "OPTIONS:".bold());
    println!(
        "  {}, {}      Show this help message",
        "-h".green(),
        "--help".green()
    );
    println!(
        "  {}, {}   Show version information",
        "-V".green(),
        "--version".green()
    );
}
