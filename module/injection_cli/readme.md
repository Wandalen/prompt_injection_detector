# vllm_inferencer

CLI interface for vLLM inference engine.

## usage

```bash
# Generate text with default settings
cargo run -p vllm_inferencer -- prompt::"Once upon a time"

# Specify max tokens
cargo run -p vllm_inferencer -- prompt::"Hello world" max_tokens::100

# Using release build
cargo run -p vllm_inferencer --release -- prompt::"The future of AI"
```

## arguments

- `prompt::"text"` - Input prompt (required)
- `max_tokens::N` - Maximum tokens to generate (default: 50)
- `--help` or `-h` - Show help message

## requirements

- NVIDIA GPU with CUDA 11.0+ (CPU fallback available)
- ~2GB GPU memory for Phi-1.5 model
- Internet connection for first run (downloads model)

## secrets

Optional HuggingFace token can be placed in `secret/hf_token.txt` at workspace root for faster downloads and access to gated models.
