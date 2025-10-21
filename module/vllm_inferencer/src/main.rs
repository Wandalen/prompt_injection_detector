//! vllm_inferencer - Ultra-minimal LLM inference engine
//!
//! Main entry point with unilang CLI interface.
//!
//! # Example
//!
//! ```bash
//! cargo run -- .infer prompt::"Once upon a time"
//! ```

/// Main entry point
///
/// MVP: Simplified CLI parsing for proof-of-concept
/// TODO: Integrate full unilang system after MVP validation
#[cfg(feature = "full")]
#[inline]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // MVP: Simple arg parsing (will use full unilang after MVP proves Candle works)
    let args: Vec<String> = std::env::args().collect();

    // Check for help
    if args.len() == 1 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_usage();
        return Ok(());
    }

    // Parse unilang-style args: .infer prompt::"text" max_tokens::50
    let mut prompt: Option<String> = None;
    let mut max_tokens: usize = 50;

    for arg in args.iter().skip(1) {
        if arg.starts_with("prompt::") {
            prompt = Some(arg.strip_prefix("prompt::").unwrap_or("").to_string());
        } else if arg.starts_with("max_tokens::") {
            if let Ok(n) = arg
                .strip_prefix("max_tokens::")
                .unwrap_or("50")
                .parse::<usize>()
            {
                max_tokens = n;
            }
        }
    }

    let prompt = prompt.ok_or("Error: prompt is required. Use: prompt::\"your text here\"")?;

    println!("Prompt: {}", prompt);
    println!("Max tokens: {}", max_tokens);
    println!();

    // Load model and initialize generator
    let model = vllm_core::ModelLoader::new()?;
    let mut generator = vllm_core::Generator::new(model);

    // Generate text
    let generated = generator.generate(&prompt, max_tokens)?;

    // Print result
    println!("\n=== Generated Text ===");
    println!("{}", generated);

    Ok(())
}

/// Print usage information
#[cfg(feature = "full")]
#[inline]
fn print_usage() {
    eprintln!("vllm_inferencer - Ultra-minimal LLM inference engine");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  cargo run -p vllm_inferencer -- .infer prompt::\"your text\" [max_tokens::50]");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  cargo run -p vllm_inferencer -- .infer prompt::\"Once upon a time\"");
    eprintln!("  cargo run -p vllm_inferencer -- .infer prompt::\"Hello world\" max_tokens::20");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  prompt::TEXT        Input prompt text (required)");
    eprintln!("  max_tokens::NUM     Maximum tokens to generate (default: 50)");
    eprintln!("  --help, -h          Show this help message");
}

/// Stub main when full feature is not enabled
#[cfg(not(feature = "full"))]
fn main() {
    eprintln!("Error: vllm_inferencer requires 'full' feature to be enabled");
    eprintln!("Build with: cargo build --features full");
    std::process::exit(1);
}
