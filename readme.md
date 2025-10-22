# vllm_inferencer

Ultra-minimal LLM inference engine in Rust using [Candle](https://github.com/huggingface/candle) framework.

**Rules**: See [rulebook.md](rulebook.md) for project standards. Key: run `cargo fmt --all` before commits.

## Project Goal

**Prove Candle framework works for LLM inference in 3-5 days.**

Load GPT-2 to GPU, generate text, celebrate. That's it.

## Status

**Current Phase:** Phase 0 Complete → Phase 1 Ready to Start

**Phase 0: Foundation** ✅ COMPLETE
- Workspace structure with 3 crates
- Feature gating and crate distribution compliance
- MIT license and proper configuration
- Ready for implementation

**Phase 1: Core Inference** (3-5 days) 🔄 READY
- [ ] Day 1-2: Understand Candle API, get examples working
- [ ] Day 3-4: Load Phi-1.5, make forward pass work
- [ ] Day 5: Generate text, test, done!

See [roadmap.md](./roadmap.md) for complete timeline.

## Quick Start

**Prerequisites:** NVIDIA GPU (8GB+ VRAM), CUDA 11.0+, Rust 1.70+

```bash
# Build and run CLI
cargo run -p vllm_inferencer --features full -- prompt::"Once upon a time"

# Run HTTP server
cargo run -p vllm_server --features full

# Run tests
cargo nextest run --all-features
```

## Feature Flags

This workspace uses the enabled/full feature pattern:

- `enabled` - Minimal feature set (included in default)
- `full` - All features including GPU acceleration, model loading, HTTP server

**Usage:**

```bash
# Use default features (enabled)
cargo build

# Use full features (required for actual inference)
cargo build --features full
cargo run -p vllm_inferencer --features full
cargo run -p vllm_server --features full
```

## MVP Features (Ultra-Minimal)

**ONE THING:** CLI that generates text

- Load Phi-1.5 to GPU
- Take prompt from CLI argument
- Generate text (greedy sampling)
- Print to console

**That's it.** No fancy features. Just prove Candle works.

## Architecture

```
CLI/HTTP Interface
       ↓
 Generation Coordinator
       ↓
  Model Management
       ↓
 Candle Framework → GPU
```

See [spec.md](./spec.md) for complete specification.

## Project Structure

```
vllm_inferencer/
├── src/
│   ├── main.rs          # Entry point (CLI + HTTP)
│   ├── lib.rs           # Public API
│   ├── model.rs         # Model loading
│   ├── generate.rs      # Generation logic
│   └── error.rs         # Error types
├── tests/
│   ├── readme.md        # Manual testing plan
│   └── basic_tests.rs   # Integration tests
├── secret/              # Secret management (gitignored)
├── spec.md              # Complete specification
├── decisions.md         # Design decisions log
├── Cargo.toml
├── Makefile
└── readme.md            # This file
```

## Documentation

- **[roadmap.md](./roadmap.md)** - Development roadmap and timeline
- **[spec.md](./spec.md)** - Complete technical specification
- **[decisions.md](./decisions.md)** - Auto-decision log
- **[tests/readme.md](./tests/readme.md)** - Testing strategy and manual procedures

## Development

```bash
cargo nextest run --all-features  # Run tests
cargo doc --open                  # Generate docs
```

## Dependencies

### Core
- **candle-core**, **candle-nn**, **candle-transformers** (0.8) - ML framework
- **tokenizers** (0.15) - HuggingFace tokenizers
- **error_tools** (0.35) - Error handling (workspace standard)

### HTTP Server (Milestone 2+)
- **axum** (0.7) - HTTP framework
- **tokio** (1.x) - Async runtime

### CLI
- **clap** (4.x) - Command-line parsing

## Model Support

**MVP:** GPT-2 Small (124M parameters) only

**Future:** Gemma-3B, Llama-7B, MamayLM-Gemma-3-4B-IT

## Out of Scope (MVP)

**Everything else:**
- ❌ HTTP API (add later if CLI works)
- ❌ KV Cache (add later if needed)
- ❌ Advanced sampling
- ❌ Batching, streaming, monitoring
- ❌ Larger models
- ❌ Everything not listed above

**Philosophy:** Validate the riskiest assumption (Candle works) in minimum time. Add features ONLY if MVP succeeds.

## Success Criteria

**Does it work?**

```bash
$ cargo run -- --prompt "Once upon a time"
> Once upon a time, there was a wizard...

✅ SUCCESS - Candle works, continue building
❌ FAILURE - Consider PyTorch instead
```

That's the only metric that matters for MVP.

## Testing

All tests located in `tests/` directory:
- `basic_tests.rs` - Core functionality
- Manual testing procedures in `tests/readme.md`

Run tests: `w3 .test l::3` or `make ctest3`

## License

MIT OR Apache-2.0

## Contributing

1. Follow project rulebooks in `/home/user1/pro/genai/code/rules/`
2. Use `error_tools` for error handling
3. All code must pass `w3 .test l::3`
4. Update `spec.md` before changing behavior
5. Document manual testing in `tests/readme.md`

## References

- [Candle Framework](https://github.com/huggingface/candle)
- [GPT-2 Model](https://huggingface.co/gpt2)
- [Specification](./spec.md)
- [Design Decisions](./decisions.md)

---

**Status:** Ready for Milestone 1 implementation 🚀
