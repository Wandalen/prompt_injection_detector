# vllm_inferencer Test Documentation

## Test Strategy

Following project rulebooks:
- All tests located in `tests/` directory (not in `src/`)
- Test-driven development (TDD): write tests before implementation
- Manual testing documented for subjective quality checks
- Automated tests for objective functionality

## Manual Testing Plan

### Milestone 1: CLI Text Generation

**Prerequisites:**
- NVIDIA GPU with CUDA 11.0+ installed
- Verify with: `nvidia-smi`
- Rust 1.70+ installed

**Test Scenarios:**

#### M1-T1: Model Loading Verification
1. Run: `cargo run --release -- --prompt "Test" --max-tokens 10`
2. Observe console output for model loading progress
3. Monitor GPU memory: `watch -n 1 nvidia-smi`
4. **Expected:**
   - Model loads in <10 seconds
   - GPU memory increases by ~500MB-2GB
   - No errors printed to stderr

#### M1-T2: Text Coherence (Manual Quality Check)
1. Run with 5 diverse prompts:
   - "Once upon a time"
   - "The future of artificial intelligence"
   - "In a world where"
   - "Scientists discovered"
   - "def hello_world():"
2. **Expected:** Generated text is grammatically coherent (human judgment)

#### M1-T3: Determinism Validation
1. Run same prompt twice: `cargo run -- --prompt "Hello world" --max-tokens 20`
2. Compare outputs character-by-character
3. **Expected:** Identical outputs (greedy sampling is deterministic)

#### M1-T4: Memory Stability
1. Run 100 sequential generations (use script):
   ```bash
   for i in {1..100}; do
     cargo run --release -- --prompt "Test $i" --max-tokens 10
     sleep 1
   done
   ```
2. Monitor `nvidia-smi` throughout
3. **Expected:** GPU memory remains stable (±100MB variance max)

---

### Milestone 2: HTTP API

#### M2-T1: Server Startup
1. Run: `cargo run --release --features http_server -- --server`
2. Check server responds: `curl http://localhost:3000/health`
3. **Expected:** `{"status":"ok"}` response in <50ms

#### M2-T2: Generation Endpoint
1. Send request:
   ```bash
   curl -X POST http://localhost:3000/generate \
     -H "Content-Type: application/json" \
     -d '{"prompt": "Hello", "max_tokens": 20}'
   ```
2. **Expected:** JSON response with `text`, `tokens_generated`, `time_ms` fields

#### M2-T3: Error Handling
1. Test invalid requests:
   - Missing prompt: `curl -X POST http://localhost:3000/generate -d '{}'`
   - Invalid JSON: `curl -X POST http://localhost:3000/generate -d 'not json'`
   - Invalid max_tokens: `curl -X POST http://localhost:3000/generate -d '{"prompt":"Hi","max_tokens":-1}'`
2. **Expected:** HTTP 400 with clear error messages

#### M2-T4: Concurrent Requests (Sequential Processing)
1. Send 20 concurrent requests:
   ```bash
   for i in {1..20}; do
     curl -X POST http://localhost:3000/generate \
       -H "Content-Type: application/json" \
       -d "{\"prompt\":\"Request $i\",\"max_tokens\":10}" &
   done
   wait
   ```
2. **Expected:** All requests complete successfully (sequential processing, no crashes)

---

### Milestone 3: KV Cache Optimization

#### M3-T1: Performance Comparison
1. **Baseline (No KV Cache):** Measure generation time for 50 tokens
2. **With KV Cache:** Measure generation time for 50 tokens
3. Calculate speedup ratio: `baseline_time / cache_time`
4. **Expected:** Speedup between 5x and 10x

#### M3-T2: Correctness Validation
1. Generate text for 10 test prompts with and without KV Cache
2. Compare outputs character-by-character
3. **Expected:** All outputs match exactly (±0.001 numerical precision acceptable)

#### M3-T3: Memory Profiling
1. Run 100 generations with KV Cache enabled
2. Monitor GPU memory via `nvidia-smi`
3. **Expected:**
   - Memory usage scales linearly with sequence length
   - No memory leaks (final memory ≤ initial + 100MB)

---

## Automated Tests

### Test Files

- **`basic_tests.rs`** - Core functionality (M1)
  - Model loading to GPU
  - Tokenization correctness vs HuggingFace
  - Basic generation

- **`http_api_tests.rs`** (M2+) - HTTP endpoints
  - Health check endpoint
  - Generate endpoint with valid requests
  - Error handling for invalid requests

- **`cache_tests.rs`** (M3) - KV Cache
  - Correctness (outputs match baseline)
  - Performance (speedup measurement)
  - Memory behavior

### Running Tests

```bash
# Run all tests (Level 3)
w3 .test l::3

# Or using Makefile
make ctest3

# Run specific test file
cargo test --test basic_tests
```

### Test Requirements

- Tests **must** pass before merging to main branch
- Tests **must** run on GPU-enabled CI environment
- All tests **must** be warning-free (`RUSTFLAGS="-D warnings"`)

---

## Reference Validation

### HuggingFace Comparison (Milestone 1)

To validate correctness against reference implementation:

```python
# Python script using HuggingFace transformers
from transformers import GPT2LMHeadModel, GPT2Tokenizer

tokenizer = GPT2Tokenizer.from_pretrained("gpt2")
model = GPT2LMHeadModel.from_pretrained("gpt2")

prompt = "Once upon a time"
inputs = tokenizer(prompt, return_tensors="pt")

# Greedy generation
outputs = model.generate(
    **inputs,
    max_new_tokens=50,
    do_sample=False,  # Greedy
    pad_token_id=tokenizer.eos_token_id
)

generated_text = tokenizer.decode(outputs[0])
print(generated_text)
```

Compare Rust implementation output with Python output for 10 test prompts.
All outputs **must** match exactly for Greedy Sampling.

---

## Known Limitations

- GPU-only testing (no CPU fallback mode)
- Requires NVIDIA GPU with CUDA 11.0+
- Cannot run tests in CI without GPU runners
- Manual testing required for text quality evaluation

---

## Test Execution Checklist

### Milestone 1 Completion
- [ ] M1-T1: Model loading verified
- [ ] M1-T2: Text coherence checked for 5 prompts
- [ ] M1-T3: Determinism validated
- [ ] M1-T4: Memory stability over 100 runs
- [ ] All automated tests pass: `w3 .test l::3`
- [ ] Correctness validated against HuggingFace (10 prompts)

### Milestone 2 Completion
- [ ] M2-T1: Server startup verified
- [ ] M2-T2: Generation endpoint tested
- [ ] M2-T3: Error handling validated
- [ ] M2-T4: 20 concurrent requests handled
- [ ] All HTTP tests pass
- [ ] Manual API testing with curl/Postman

### Milestone 3 Completion
- [ ] M3-T1: Performance speedup measured (5x-10x)
- [ ] M3-T2: Correctness validated (10 prompts match)
- [ ] M3-T3: Memory profiling shows no leaks
- [ ] All cache tests pass
- [ ] Performance data documented

---

**All manual tests must be executed and results documented before declaring milestone complete.**
