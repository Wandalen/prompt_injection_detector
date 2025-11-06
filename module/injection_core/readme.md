# vllm_core

core LLM inference library using Candle framework.

## features

- **model loading**: Phi-1.5 from HuggingFace Hub
- **gpu acceleration**: CUDA support with CPU fallback
- **text generation**: greedy sampling (deterministic)
- **secrets management**: HuggingFace token via workspace_tools

## usage

```rust
use vllm_core::{ModelLoader, Generator};

// Load model
let model = ModelLoader::new()?;
let mut generator = Generator::new( model );

// Generate text
let output = generator.generate( "Once upon a time", 50 )?;
println!( "{}", output );
```

## secrets

Place HuggingFace token in one of:
- `secret/hf_token.txt`
- `secret/huggingface_token.txt`
- Environment variable `HF_TOKEN`

Token is optional - anonymous access will be used if not found.

## testing

See `tests/readme.md` for manual testing plan and automated test strategy.
