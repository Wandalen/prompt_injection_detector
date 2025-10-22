# roadmap

vLLM inference engine development roadmap

**Status:** Phase 0 Complete - Ready for Implementation
**Last Updated:** 2025-10-22

---

## Current Status

### ✅ Phase 0: Foundation (COMPLETE)

**Completed:**
- Workspace structure created with 3 crates
- Cargo.toml with workspace dependencies
- enabled/full feature pattern implemented
- All dependencies optional with feature gates
- Code feature-gated with #[cfg(feature = "full")]
- MIT license added
- Comprehensive .gitignore configuration
- Secrets management structure
- Rulebook and specification documents
- Git repository initialized and pushed

**Deliverables:**
- `/home/user1/pro/lib/vllm_inferencer/` workspace
- 3 member crates: vllm_core, vllm_inferencer, vllm_server
- Crate distribution rulebook compliant
- Ready for crates.io publication (after implementation)

---

## Upcoming Phases

### 🔄 Phase 1: Core Inference Implementation (IN PROGRESS)

**Goal:** Prove Candle framework works with Phi-1.5 model

**Duration:** 3-5 days

**Status:** Ready to start

**Tasks:**

#### Day 1-2: Candle Framework Learning
- [ ] Study Candle examples in their repository
- [ ] Run existing Phi model examples
- [ ] Understand tensor operations and device management
- [ ] Test CUDA device initialization
- [ ] Verify GPU memory allocation with nvidia-smi

**Deliverables:**
- Working knowledge of Candle API
- Test scripts demonstrating basic tensor operations
- GPU verification completed

#### Day 3-4: Model Loading Implementation
- [ ] Implement ModelLoader::new() in `module/vllm_core/src/model.rs`
- [ ] Add HuggingFace Hub model download
- [ ] Load Phi-1.5 model weights from safetensors
- [ ] Initialize model on GPU device
- [ ] Implement forward pass
- [ ] Add tokenizer initialization
- [ ] Test tokenization encode/decode

**Deliverables:**
- Functional ModelLoader with Phi-1.5
- Model loads to GPU successfully
- Tokenizer works correctly

#### Day 5: Text Generation
- [ ] Implement Generator::generate() in `module/vllm_core/src/generate.rs`
- [ ] Add greedy sampling logic (argmax)
- [ ] Implement generation loop
- [ ] Add EOS token detection
- [ ] Wire up CLI in `module/vllm_inferencer/src/main.rs`
- [ ] Test end-to-end generation

**Deliverables:**
- Working text generation from CLI
- Coherent output for test prompts

**Success Criteria:**
```bash
$ cargo run -p vllm_inferencer --features full -- prompt::"Once upon a time"
Prompt: Once upon a time
Max tokens: 50

=== Generated Text ===
Once upon a time, there was a wizard who lived in a castle...
```

**Blockers/Risks:**
- Candle API might be immature (CRITICAL - project blocker)
- Phi model might not work as expected
- GPU memory issues with 1.3B parameter model
- CUDA compatibility problems

**Mitigation:**
- Keep implementation minimal
- Document all issues found
- Have PyTorch backup plan ready

---

### 📋 Phase 2: HTTP API Server (PLANNED)

**Goal:** Expose inference via REST API

**Duration:** 2-3 days

**Prerequisites:** Phase 1 complete

**Tasks:**

#### Day 1: Server Implementation
- [ ] Implement health endpoint in `module/vllm_server/src/main.rs`
- [ ] Add request/response types
- [ ] Integrate vllm_core with async wrapper
- [ ] Add proper error handling
- [ ] Test with curl

#### Day 2: Production Readiness
- [ ] Add request validation
- [ ] Implement proper logging
- [ ] Add CORS configuration
- [ ] Test concurrent requests
- [ ] Document API endpoints

#### Day 3: Integration Testing
- [ ] Write HTTP integration tests
- [ ] Test error scenarios
- [ ] Load testing with multiple clients
- [ ] Document deployment

**Deliverables:**
- Working HTTP server on port 3000
- /health and /generate endpoints
- API documentation
- Integration tests passing

**Success Criteria:**
```bash
$ cargo run -p vllm_server --features full
Server started on 0.0.0.0:3000

$ curl -X POST http://localhost:3000/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello", "max_tokens": 20}'
{"generated": "Hello world, this is a test..."}
```

---

### 🚀 Phase 3: Optimization (FUTURE)

**Goal:** Improve performance and features

**Duration:** 3-5 days

**Prerequisites:** Phase 2 complete

**Potential Tasks:**
- [ ] Implement KV Cache for 5-10x speedup
- [ ] Add temperature/top-p sampling
- [ ] Support streaming responses
- [ ] Batch request processing
- [ ] Memory optimization
- [ ] Benchmark and profile

**Status:** Deferred until MVP proves viable

---

## Timeline

```
Phase 0: Foundation              [████████████████] COMPLETE
Phase 1: Core Inference (5 days) [░░░░░░░░░░░░░░░░] 0%
  └─ Day 1-2: Learning          [░░░░░░░░░░░░░░░░]
  └─ Day 3-4: Model Loading     [░░░░░░░░░░░░░░░░]
  └─ Day 5: Generation          [░░░░░░░░░░░░░░░░]
Phase 2: HTTP API (3 days)       [░░░░░░░░░░░░░░░░] NOT STARTED
Phase 3: Optimization (future)   [░░░░░░░░░░░░░░░░] NOT STARTED

Total Estimated: 8-10 days for MVP (Phase 1 + 2)
```

---

## Dependencies

### External Dependencies
- **Candle Framework** (0.8) - Core ML framework
- **HuggingFace Hub** - Model hosting
- **CUDA** (11.0+) - GPU acceleration
- **Rust** (1.70+) - Language toolchain

### Hardware Dependencies
- **NVIDIA GPU** - RTX 3060 or better
- **8GB+ VRAM** - For Phi-1.5 (1.3B params)
- **16GB+ RAM** - System memory
- **20GB+ Storage** - Model downloads

### Critical Path
1. CUDA must be available → GPU detection
2. GPU detection → Model loading
3. Model loading → Text generation
4. Text generation → HTTP API

---

## Risk Assessment

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Candle framework immature | CRITICAL | Medium | PyTorch backup plan |
| GPU memory insufficient | HIGH | Low | Test early, use smaller model |
| CUDA compatibility issues | HIGH | Medium | Document requirements clearly |
| Phi model doesn't work | MEDIUM | Low | Switch to GPT-2 if available |
| Performance too slow | MEDIUM | Medium | Add KV Cache in Phase 3 |
| API design issues | LOW | Low | Keep API minimal initially |

---

## Success Metrics

### Phase 1 Success Criteria
- ✅ Model loads to GPU (verify with nvidia-smi)
- ✅ Text generation produces coherent output
- ✅ CLI accepts prompt and generates response
- ✅ End-to-end latency < 10 seconds for 50 tokens
- ✅ GPU utilization > 60% during generation

### Phase 2 Success Criteria
- ✅ HTTP server starts successfully
- ✅ Health endpoint returns 200 OK
- ✅ Generate endpoint produces valid JSON
- ✅ Handles concurrent requests correctly
- ✅ Error responses are properly formatted

### Overall MVP Success
- ✅ Proves Candle framework is viable
- ✅ Generates human-readable text
- ✅ Can be deployed and used by others
- ✅ Code passes all tests
- ✅ Documentation is complete

---

## Decision Points

### After Phase 1
**Decision:** Continue with Candle or pivot to PyTorch?

**Continue if:**
- Text generation works
- Performance is acceptable
- API is stable enough
- No major blockers found

**Pivot if:**
- Candle has critical bugs
- Performance is unusable
- API is too limited
- Documentation is insufficient

### After Phase 2
**Decision:** Add Phase 3 optimizations or ship MVP?

**Optimize if:**
- Performance needs improvement
- Users request advanced features
- Competitive pressure exists

**Ship if:**
- MVP meets all requirements
- Performance is good enough
- No critical feature gaps

---

## Next Steps

### Immediate Actions (Start Phase 1)

1. **Setup Development Environment:**
   ```bash
   cd /home/user1/pro/lib/vllm_inferencer
   cargo check --features full
   nvidia-smi  # Verify GPU
   ```

2. **Study Candle Examples:**
   ```bash
   git clone https://github.com/huggingface/candle
   cd candle/candle-examples
   cargo run --example phi --release
   ```

3. **Begin Model Loading Implementation:**
   - Edit `module/vllm_core/src/model.rs`
   - Implement ModelLoader::new()
   - Test GPU device initialization

4. **Track Progress:**
   - Update this roadmap daily
   - Check off completed tasks
   - Document blockers immediately

---

## References

- **Specification:** [spec.md](./spec.md)
- **Rulebook:** [rulebook.md](./rulebook.md)
- **Decisions:** [decisions.md](./decisions.md)
- **Candle Docs:** https://github.com/huggingface/candle
- **Phi Model:** https://huggingface.co/microsoft/phi-1_5

---

## Change Log

- **2025-10-22:** Roadmap created, Phase 0 marked complete
- **2025-10-21:** Project initialized, foundation laid

---

**Note:** This roadmap follows the ultra-minimal MVP philosophy. Each phase must be completed and validated before proceeding to the next. If Phase 1 fails, the project pivots to PyTorch rather than continuing with Candle.
