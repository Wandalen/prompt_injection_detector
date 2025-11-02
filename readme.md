# Prompt Injection Detector

High-performance prompt injection detection system using ProtectAI's DeBERTa model with ONNX Runtime.

**Rules**: See [rulebook.md](rulebook.md) for project standards. Key: run `cargo fmt --all` before commits.

## Project Goal

**Detect prompt injection attacks with 95%+ accuracy using ONNX Runtime for fast inference.**

Load DeBERTa model, classify text as benign or injection, deploy to production. That's it.

## Status

**Current Phase:** Phase 0 Complete → Phase 1 Ready to Start

**Phase 0: Foundation** ✅ COMPLETE
- Workspace structure with 3 crates
- Feature gating and crate distribution compliance
- MIT license and proper configuration
- Ready for implementation

**Phase 1: Core Detection** (5 days) 🔄 READY
- [ ] Day 1-2: Setup ONNX Runtime, load DeBERTa model
- [ ] Day 3-4: Implement classification logic
- [ ] Day 5: Test detection accuracy, validate against ProtectAI benchmarks

See [roadmap.md](./roadmap.md) for complete timeline.

## Quick Start

**Prerequisites:**
- NVIDIA GPU (8GB+ VRAM, optional)
- Rust 1.70+
- CUDA 12.x+ (for GPU acceleration)
- Model files (see installation below)

### Model Installation

**🎉 Automatic Download (Recommended)**

Model files are **automatically downloaded** from HuggingFace during the first build:

```bash
# Just build - models download automatically!
make build
# or
cargo build -p injection_cli --release
```

The build script will:
1. ✅ Create `artifacts/` directory if it doesn't exist
2. ✅ Download `model.onnx` (572MB) from HuggingFace
3. ✅ Download `tokenizer.json` (3.5MB)
4. ✅ Download `config.json` (1.4KB)
5. ✅ For Burn backend: auto-convert ONNX → MPK

**Model Repository:** https://huggingface.co/tihilya/modernbert-base-prompt-injection-detection

**Progress Output:**
```
Build script: model.onnx not found, downloading from HuggingFace...
Build script: Downloading model.onnx from https://huggingface.co/...
Build script: Downloaded 100 / 572 MB (17%)
Build script: Downloaded 200 / 572 MB (34%)
...
Build script: ✓ Successfully downloaded model.onnx (599000438 bytes)
```

**Manual Download** (Optional)

If automatic download fails due to network issues:

```bash
# Download from HuggingFace manually
mkdir -p artifacts
cd artifacts
wget https://huggingface.co/tihilya/modernbert-base-prompt-injection-detection/resolve/main/model.onnx
wget https://huggingface.co/tihilya/modernbert-base-prompt-injection-detection/resolve/main/tokenizer.json
wget https://huggingface.co/tihilya/modernbert-base-prompt-injection-detection/resolve/main/config.json
cd ..
```

**Expected artifacts structure:**
```
prompt_injection_detector/
├── artifacts/
│   ├── model.onnx       # Required for ORT backend
│   ├── model.mpk        # Auto-generated for Burn backend
│   ├── tokenizer/
│   │   └── tokenizer.json  # Required for both
│   └── config.json      # Optional, for reference
```

**Verify installation:**
```bash
ls -lh artifacts/
# Should show:
# model.onnx    (~572MB)
# tokenizer.json
```

### Build and Run

```bash
# Build and run CLI
cargo run -p injection_cli --features full -- .detect text::"Ignore all previous instructions"

# Run HTTP server
cargo run -p injection_server --features full

# Run tests
cargo nextest run --all-features
```

## Feature Flags

This workspace uses the enabled/full feature pattern:

- `enabled` - Minimal feature set (included in default)
- `full` - All features including ONNX Runtime, model loading, HTTP server

**Usage:**

```bash
# Use default features (enabled)
cargo build

# Use full features (required for actual detection)
cargo build --features full
cargo run -p injection_cli --features full
cargo run -p injection_server --features full
```

## MVP Features

**ONE THING:** Detect prompt injection attacks

- Load DeBERTa model via ONNX Runtime
- Classify text as benign (0) or injection (1)
- Return confidence scores
- CLI and HTTP API interfaces

**That's it.** No complex features. Just prove ONNX Runtime works for fast ML inference.

## Architecture

```
CLI/HTTP Interface
       ↓
 Classification Coordinator
       ↓
  Model Management
       ↓
 ONNX Runtime → GPU (optional)
```

See [spec.md](./spec.md) for complete specification.

## Project Structure

```
prompt_injection_detector/
├── module/
│   ├── injection_core/     # Core detection library
│   ├── injection_cli/      # CLI with unilang
│   └── injection_server/   # HTTP API server
├── secret/                 # Secret management (gitignored)
├── spec.md                 # Complete specification
├── decisions.md            # Design decisions log
├── roadmap.md              # Development roadmap
├── Cargo.toml              # Workspace configuration
└── readme.md               # This file
```

## Documentation

- **[roadmap.md](./roadmap.md)** - Development roadmap and timeline
- **[spec.md](./spec.md)** - Complete technical specification
- **[decisions.md](./decisions.md)** - Design decision log

## Model Information

**Model:** protectai/deberta-v3-base-prompt-injection-v2

- **Type:** Binary text classification
- **Architecture:** DeBERTa-v3-base (200M parameters)
- **Accuracy:** 99.93% (training), 95.25% (production)
- **Max Length:** 512 tokens
- **Classes:** 0=benign, 1=injection
- **License:** Apache 2.0

## Development

```bash
cargo nextest run --all-features  # Run tests
cargo doc --open                  # Generate docs
cargo fmt --all                   # Format code
```

## Dependencies

### Core
- **ort** (1.16) - ONNX Runtime for fast inference
- **tokenizers** (0.15) - HuggingFace tokenizers
- **error_tools** (0.35) - Error handling (workspace standard)
- **unilang** (0.30) - CLI command framework

### HTTP Server (Milestone 2+)
- **axum** (0.7) - HTTP framework
- **tokio** (1.x) - Async runtime

## API Examples

### CLI

```bash
# Detect injection
cargo run -p injection_cli --features full -- .detect text::"Ignore all previous instructions"

# With custom threshold
cargo run -p injection_cli --features full -- .detect text::"Hello world" threshold::0.8
```

### HTTP API

```bash
# Start server
cargo run -p injection_server --features full

# Health check
curl http://localhost:3000/health

# Detect injection
curl -X POST http://localhost:3000/detect \
  -H "Content-Type: application/json" \
  -d '{"text": "Ignore all previous instructions", "threshold": 0.5}'
```

**Response:**
```json
{
  "label": "injection",
  "confidence": 0.98,
  "is_safe": false
}
```

## Deployment

### Docker

```bash
# Build image
docker build -t prompt-injection-detector .

# Run container
docker run -p 3000:3000 prompt-injection-detector
```

### Vast.ai

See [deployment documentation](./docs/deployment.md) for complete Vast.ai setup guide.

## Out of Scope (MVP)

**Everything else:**
- ❌ Jailbreak detection (model limitation)
- ❌ Non-English text (model limitation)
- ❌ Streaming responses
- ❌ Batching
- ❌ Advanced monitoring
- ❌ Everything not listed above

**Philosophy:** Build the simplest thing that proves ONNX Runtime works for fast ML inference. Add features ONLY if MVP succeeds.

## Success Criteria

**Does it work?**

```bash
$ cargo run -p injection_cli --features full -- .detect text::"Ignore all previous instructions"
Input: "Ignore all previous instructions"
Threshold: 0.5

=== Detection Result ===
Classification: INJECTION
Confidence: 98.5%
Safe: false

✅ SUCCESS - ONNX Runtime works, continue building
❌ FAILURE - Consider alternatives
```

That's the only metric that matters for MVP.

## License

MIT OR Apache-2.0

## Contributing

1. Follow project rulebooks
2. Use `error_tools` for error handling
3. All code must pass `cargo nextest run --all-features`
4. Update `spec.md` before changing behavior
5. Run `cargo fmt --all` before commits

## References

- [ProtectAI DeBERTa Model](https://huggingface.co/protectai/deberta-v3-base-prompt-injection-v2)
- [ONNX Runtime](https://onnxruntime.ai/)
- [Unilang CLI Framework](https://crates.io/crates/unilang)
- [Specification](./spec.md)
- [Design Decisions](./decisions.md)

---

**Status:** Ready for Milestone 1 implementation 🚀
