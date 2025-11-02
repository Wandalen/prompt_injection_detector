#!/usr/bin/env python3
"""
Export ProtectAI DeBERTa prompt injection model to ONNX format.

This script downloads the pre-trained model from HuggingFace and converts it to ONNX
format for use with the ORT (ONNX Runtime) backend.

Model: protectai/deberta-v3-base-prompt-injection-v2
Output: artifacts/model.onnx (572MB)
"""

import os
import sys
from pathlib import Path

try:
    from optimum.onnxruntime import ORTModelForSequenceClassification
    from transformers import AutoTokenizer
except ImportError:
    print("Error: Required packages not installed")
    print("Run: pip install huggingface_hub torch transformers optimum onnx onnxruntime")
    sys.exit(1)


def main():
    # Model configuration
    MODEL_ID = "protectai/deberta-v3-base-prompt-injection-v2"
    OUTPUT_DIR = Path("artifacts")
    TOKENIZER_DIR = OUTPUT_DIR / "tokenizer"

    print(f"Exporting model: {MODEL_ID}")
    print(f"Output directory: {OUTPUT_DIR}")
    print()

    # Create output directories
    OUTPUT_DIR.mkdir(exist_ok=True)
    TOKENIZER_DIR.mkdir(exist_ok=True)

    # Check if model already exists
    model_path = OUTPUT_DIR / "model.onnx"
    if model_path.exists():
        print(f"⚠️  Model already exists at {model_path}")
        response = input("Overwrite? [y/N]: ").strip().lower()
        if response != 'y':
            print("Export cancelled")
            return
        print()

    # Download and export to ONNX
    print("Step 1/3: Downloading model from HuggingFace...")
    print("(This may take several minutes for first download)")

    try:
        model = ORTModelForSequenceClassification.from_pretrained(
            MODEL_ID,
            export=True,  # Export to ONNX format
            provider="CPUExecutionProvider",  # Use CPU for export
        )
    except Exception as e:
        print(f"❌ Failed to download/export model: {e}")
        sys.exit(1)

    print("✓ Model downloaded and exported to ONNX")
    print()

    # Save model
    print("Step 2/3: Saving ONNX model...")
    try:
        model.save_pretrained(OUTPUT_DIR)
        print(f"✓ Model saved to {OUTPUT_DIR}/model.onnx")
    except Exception as e:
        print(f"❌ Failed to save model: {e}")
        sys.exit(1)

    print()

    # Download and save tokenizer
    print("Step 3/3: Downloading tokenizer...")
    try:
        tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)
        tokenizer.save_pretrained(TOKENIZER_DIR)
        print(f"✓ Tokenizer saved to {TOKENIZER_DIR}/tokenizer.json")
    except Exception as e:
        print(f"❌ Failed to download tokenizer: {e}")
        sys.exit(1)

    print()
    print("=" * 60)
    print("✅ Export complete!")
    print()

    # Verify files
    model_size = model_path.stat().st_size if model_path.exists() else 0
    print("Artifacts created:")
    print(f"  - {OUTPUT_DIR}/model.onnx ({model_size / 1024 / 1024:.1f} MB)")
    print(f"  - {TOKENIZER_DIR}/tokenizer.json")

    config_path = OUTPUT_DIR / "config.json"
    if config_path.exists():
        print(f"  - {OUTPUT_DIR}/config.json")

    print()
    print("Next steps:")
    print("  1. Verify artifacts: ls -lh artifacts/")
    print("  2. Run tests: cargo test --features backend-ort")
    print("  3. Try CLI: cargo run -p injection_cli --features full -- .detect text::\"test\"")


if __name__ == "__main__":
    main()
