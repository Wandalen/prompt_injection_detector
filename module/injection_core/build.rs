//! Build script for `injection_core`
//!
//! When the `backend-burn` feature is enabled, this script converts the ONNX model
//! to Burn format (.mpk) if not already present.

#[cfg(feature = "backend-burn")]
use std::path::Path;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
#[cfg(feature = "backend-burn")]
use burn_import::onnx::ModelGen;

/// HuggingFace model repository
const HF_REPO: &str = "tihilya/modernbert-base-prompt-injection-detection";
const HF_BASE_URL: &str = "https://huggingface.co";

fn main() {
    // Download model files if missing (for both ORT and Burn backends)
    download_model_files_if_missing();

    // Burn-specific: ONNX → MPK conversion
    #[cfg(feature = "backend-burn")]
    {
        let force = std::env::var("BURN_IMPORT_ONNX").as_deref() == Ok("1");
        let generated_rs = Path::new("src/burn_impl/model/model.rs");
        let generated_weights = Path::new("src/burn_impl/model/model.mpk");

        if !force && generated_rs.exists() && generated_weights.exists() {
            return;
        }

        // Look for artifacts in parent directories
        let onnx_path = if Path::new("../../artifacts/model.onnx").exists() {
            eprintln!("Build script: Found ONNX at ../../artifacts/model.onnx");
            Path::new("../../artifacts/model.onnx")
        } else if Path::new("../artifacts/model.onnx").exists() {
            eprintln!("Build script: Found ONNX at ../artifacts/model.onnx");
            Path::new("../artifacts/model.onnx")
        } else if Path::new("artifacts/model.onnx").exists() {
            eprintln!("Build script: Found ONNX at artifacts/model.onnx");
            Path::new("artifacts/model.onnx")
        } else {
            eprintln!("Build script: model.onnx not found; skipping Burn import.");
            return;
        };

        eprintln!("Build script: Starting ONNX → Burn model conversion...");

        if let Err(err) = std::fs::create_dir_all("src/burn_impl/model") {
            eprintln!("Build script: unable to ensure src/burn_impl/model directory exists: {err}");
        }

        let mut generator = ModelGen::new();
        generator
            .half_precision(true)
            .input(onnx_path.to_str().expect("valid UTF-8 path"))
            .out_dir("src/burn_impl/model")
            .run_from_cli();

        // Patch generated model.rs to add extern crate alloc
        patch_model_rs();

        // Copy model.mpk to artifacts directory for runtime use
        copy_model_to_artifacts();
    }
}

/// Download model files from HuggingFace if they don't exist locally
fn download_model_files_if_missing() {
    // Determine artifacts directory (try multiple locations)
    let artifacts_dir = if Path::new("../../artifacts").exists() {
        Path::new("../../artifacts")
    } else if Path::new("../artifacts").exists() {
        Path::new("../artifacts")
    } else if Path::new("artifacts").exists() {
        Path::new("artifacts")
    } else {
        // Create artifacts directory in workspace root
        let artifacts_path = Path::new("../../artifacts");
        if let Err(e) = fs::create_dir_all(artifacts_path) {
            eprintln!("Build script: Failed to create artifacts directory: {}", e);
            return;
        }
        eprintln!("Build script: Created artifacts directory at {:?}", artifacts_path);
        artifacts_path
    };

    // Check and download model.onnx
    let model_path = artifacts_dir.join("model.onnx");
    if !model_path.exists() {
        eprintln!("Build script: model.onnx not found, downloading from HuggingFace...");
        download_file(
            &format!("{}/{}/resolve/main/model.onnx", HF_BASE_URL, HF_REPO),
            &model_path,
            "model.onnx",
        );
    } else {
        eprintln!("Build script: model.onnx already exists at {:?}", model_path);
    }

    // Check and download tokenizer.json
    let tokenizer_path = artifacts_dir.join("tokenizer.json");
    if !tokenizer_path.exists() {
        eprintln!("Build script: tokenizer.json not found, downloading from HuggingFace...");
        download_file(
            &format!("{}/{}/resolve/main/tokenizer.json", HF_BASE_URL, HF_REPO),
            &tokenizer_path,
            "tokenizer.json",
        );
    } else {
        eprintln!("Build script: tokenizer.json already exists at {:?}", tokenizer_path);
    }

    // Check and download config.json (optional)
    let config_path = artifacts_dir.join("config.json");
    if !config_path.exists() {
        eprintln!("Build script: config.json not found, downloading from HuggingFace...");
        download_file(
            &format!("{}/{}/resolve/main/config.json", HF_BASE_URL, HF_REPO),
            &config_path,
            "config.json",
        );
    }
}

/// Download a file from URL to destination path with progress indicator
fn download_file(url: &str, dest: &Path, name: &str) {
    eprintln!("Build script: Downloading {} from {}", name, url);
    eprintln!("Build script: Destination: {:?}", dest);

    let response = match ureq::get(url).call() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Build script: Failed to download {}: {}", name, e);
            eprintln!("Build script: Please manually download {} to {:?}", name, dest);
            return;
        }
    };

    // Get content length for progress
    let total_size = response
        .header("Content-Length")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let mut reader = response.into_reader();
    let mut file = match fs::File::create(dest) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Build script: Failed to create file {:?}: {}", dest, e);
            return;
        }
    };

    let mut buffer = vec![0; 1024 * 1024]; // 1MB buffer
    let mut downloaded: u64 = 0;
    let mut last_progress = 0;

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(e) => {
                eprintln!("Build script: Error reading response: {}", e);
                return;
            }
        };

        if let Err(e) = file.write_all(&buffer[..bytes_read]) {
            eprintln!("Build script: Error writing to file: {}", e);
            return;
        }

        downloaded += bytes_read as u64;

        // Show progress every 10MB or at completion
        if total_size > 0 {
            let progress = (downloaded * 100 / total_size) as u32;
            if progress != last_progress && (progress % 10 == 0 || progress == 100) {
                eprintln!(
                    "Build script: Downloaded {} / {} MB ({}%)",
                    downloaded / 1024 / 1024,
                    total_size / 1024 / 1024,
                    progress
                );
                last_progress = progress;
            }
        } else if downloaded % (10 * 1024 * 1024) == 0 {
            // Show progress every 10MB if size unknown
            eprintln!(
                "Build script: Downloaded {} MB...",
                downloaded / 1024 / 1024
            );
        }
    }

    eprintln!(
        "Build script: ✓ Successfully downloaded {} ({} bytes)",
        name, downloaded
    );
}

#[cfg(feature = "backend-burn")]
fn patch_model_rs() {
    let model_rs_path = "src/burn_impl/model/model.rs";

    // Read the generated file
    let content = match fs::read_to_string(model_rs_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Build script: unable to read {}: {}", model_rs_path, e);
            return;
        }
    };

    // Check if already patched
    if content.contains("extern crate alloc;") {
        return;
    }

    // Add extern crate alloc after the first line (comment)
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return;
    }

    let mut patched = String::new();
    patched.push_str(lines[0]); // First line (comment)
    patched.push('\n');
    patched.push_str("extern crate alloc;\n"); // Add extern crate
    patched.push('\n');

    // Add rest of the lines
    for line in &lines[1..] {
        patched.push_str(line);
        patched.push('\n');
    }

    // Write back
    if let Err(e) = fs::write(model_rs_path, patched) {
        eprintln!("Build script: unable to write {}: {}", model_rs_path, e);
    } else {
        eprintln!("Build script: patched {} with 'extern crate alloc;'", model_rs_path);
    }
}

#[cfg(feature = "backend-burn")]
fn copy_model_to_artifacts() {
    let source_mpk = Path::new("src/burn_impl/model/model.mpk");

    if !source_mpk.exists() {
        eprintln!("Build script: model.mpk not found, skipping copy");
        return;
    }

    // Find artifacts directory
    let artifacts_dir = if Path::new("../../artifacts").exists() {
        Path::new("../../artifacts")
    } else if Path::new("../artifacts").exists() {
        Path::new("../artifacts")
    } else if Path::new("artifacts").exists() {
        Path::new("artifacts")
    } else {
        eprintln!("Build script: artifacts directory not found, skipping model.mpk copy");
        return;
    };

    let dest_mpk = artifacts_dir.join("model.mpk");

    match fs::copy(source_mpk, &dest_mpk) {
        Ok(bytes) => {
            eprintln!("Build script: Copied model.mpk to {:?} ({} bytes)", dest_mpk, bytes);
        }
        Err(e) => {
            eprintln!("Build script: Failed to copy model.mpk: {}", e);
        }
    }
}
