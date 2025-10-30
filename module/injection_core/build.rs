#[cfg(feature = "backend-burn")]
use std::path::Path;

#[cfg(feature = "backend-burn")]
use burn_import::onnx::ModelGen;

fn main() {
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
            Path::new("../../artifacts/model.onnx")
        } else if Path::new("../artifacts/model.onnx").exists() {
            Path::new("../artifacts/model.onnx")
        } else if Path::new("artifacts/model.onnx").exists() {
            Path::new("artifacts/model.onnx")
        } else {
            eprintln!("Build script: model.onnx not found; skipping Burn import.");
            return;
        };

        if let Err(err) = std::fs::create_dir_all("src/burn_impl/model") {
            eprintln!("Build script: unable to ensure src/burn_impl/model directory exists: {err}");
        }

        let mut generator = ModelGen::new();
        generator
            .half_precision(true)
            .input(onnx_path.to_str().expect("valid UTF-8 path"))
            .out_dir("src/burn_impl/model")
            .run_from_cli();

        // Note: Artifacts are handled at runtime, not copied during build
    }
}
