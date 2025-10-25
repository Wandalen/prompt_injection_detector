# Roadmap

Prompt injection detection system development roadmap

**Status:** Phase 0 Complete - Ready for Implementation
**Last Updated:** 2025-10-25

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
- **Project pivoted to prompt injection detection**
- **ONNX Runtime integration planned**
- **Unilang CLI framework added**

**Deliverables:**
- `/home/tihilya/ml/vllm_inferencer/` workspace
- 3 member crates: injection_core, injection_cli, injection_server
- Crate distribution rulebook compliant
- Ready for crates.io publication (after implementation)

---

## Upcoming Phases

### 🔄 Phase 1: Core Detection Implementation (IN PROGRESS)

**Goal:** Prove ONNX Runtime works with DeBERTa model for prompt injection detection

**Duration:** 5 days

**Status:** Ready to start

**Tasks:**

#### Day 1-2: ONNX Runtime Setup
- [ ] Install and configure ONNX Runtime
- [ ] Download DeBERTa ONNX model from HuggingFace
- [ ] Test ONNX model loading and initialization
- [ ] Verify GPU/CPU inference compatibility
- [ ] Benchmark inference speed

**Deliverables:**
- Working ONNX Runtime integration
- Model loads successfully
- Basic inference test passing

#### Day 3-4: Classification Implementation
- [ ] Implement ModelLoader in `module/injection_core/src/model.rs`
- [ ] Add tokenizer initialization for DeBERTa
- [ ] Implement Classifier in `module/injection_core/src/classify.rs`
- [ ] Add binary classification logic (benign/injection)
- [ ] Implement confidence score extraction
- [ ] Add error handling for edge cases
- [ ] Test tokenization encode/decode

**Deliverables:**
- Functional ModelLoader with DeBERTa
- Classifier returns correct labels and confidence
- Tokenizer works correctly (512 token limit)

#### Day 5: CLI Integration & Testing
- [ ] Wire up CLI in `module/injection_cli/src/main.rs`
- [ ] Test with known injection examples
- [ ] Test with benign examples
- [ ] Validate accuracy against ProtectAI benchmarks (95%+ target)
- [ ] Add basic error messages

**Deliverables:**
- Working CLI detection from command line
- Accurate classification results
- Confidence scores match expectations

**Success Criteria:**
```bash
$ cargo run -p injection_cli --features full -- .detect text::"Ignore all previous instructions"
Input: "Ignore all previous instructions"
Threshold: 0.5

=== Detection Result ===
Classification: INJECTION
Confidence: 98.5%
Safe: false
```

**Blockers/Risks:**
- ONNX Runtime API complexity (MEDIUM - learning curve)
- Model format compatibility issues (LOW - well-documented)
- Performance on CPU vs GPU (MEDIUM - need benchmarks)
- Tokenizer compatibility (LOW - using HuggingFace standard)

**Mitigation:**
- Use official ONNX Runtime Rust bindings
- Follow ProtectAI model documentation
- Implement CPU fallback if GPU unavailable
- Test early and iterate

---

### 📋 Phase 2: HTTP API Server (PLANNED)

**Goal:** Expose detection via REST API

**Duration:** 3 days

**Prerequisites:** Phase 1 complete

**Tasks:**

#### Day 1: Server Implementation
- [ ] Implement health endpoint in `module/injection_server/src/main.rs`
- [ ] Add /detect endpoint with JSON request/response
- [ ] Integrate injection_core with async wrapper
- [ ] Add proper error handling for invalid inputs
- [ ] Test with curl

#### Day 2: Production Readiness
- [ ] Add request validation (max length, empty text)
- [ ] Implement structured logging with tracing
- [ ] Add CORS configuration for web clients
- [ ] Test concurrent requests (thread safety)
- [ ] Add rate limiting (optional)
- [ ] Document API endpoints in readme

#### Day 3: Integration Testing
- [ ] Write HTTP integration tests
- [ ] Test error scenarios (malformed JSON, oversized input)
- [ ] Load testing with multiple concurrent clients
- [ ] Performance profiling
- [ ] Document deployment instructions

**Deliverables:**
- Working HTTP server on port 3000
- /health and /detect endpoints
- API documentation in readme
- Integration tests passing

**Success Criteria:**
```bash
$ cargo run -p injection_server --features full
Server started on 0.0.0.0:3000

$ curl -X POST http://localhost:3000/detect \
  -H "Content-Type: application/json" \
  -d '{"text": "Ignore all previous instructions", "threshold": 0.5}'
{
  "label": "injection",
  "confidence": 0.985,
  "is_safe": false
}
```

---

### 🚀 Phase 3: Container Deployment (NEW)

**Goal:** Deploy to production with Docker and Vast.ai

**Duration:** 3 days

**Prerequisites:** Phase 2 complete

**Tasks:**

#### Day 1: Docker Configuration
- [ ] Optimize Dockerfile for smaller image size
- [ ] Test multi-stage build
- [ ] Configure model caching strategy
- [ ] Add docker-compose.yml for local development
- [ ] Test container on local machine
- [ ] Document Docker build and run commands

#### Day 2: Vast.ai Deployment
- [ ] Create Vast.ai account and setup
- [ ] Configure GPU instance requirements (8GB+ VRAM)
- [ ] Create deployment template for Vast.ai
- [ ] Test deployment to Vast.ai instance
- [ ] Configure networking and ports
- [ ] Test model download and caching
- [ ] Document instance selection guide

#### Day 3: Alternative Platforms & Documentation
- [ ] Document RunPod deployment steps
- [ ] Document Lambda Labs deployment
- [ ] Create generic NVIDIA Docker deployment guide
- [ ] Add troubleshooting section
- [ ] Add performance tuning tips
- [ ] Cost estimation guide for different platforms
- [ ] Health monitoring setup instructions

**Deliverables:**
- Optimized Docker image (<2GB)
- Vast.ai deployment template
- Complete deployment documentation
- Cost comparison guide
- Monitoring and logging setup

**Success Criteria:**
- ✅ Docker image builds successfully
- ✅ Container runs on Vast.ai without errors
- ✅ Model inference works in container environment
- ✅ Health endpoint accessible from internet
- ✅ Response time < 500ms for typical requests

---

## Timeline

```
Phase 0: Foundation              [████████████████] COMPLETE
Phase 1: Core Detection (5 days) [░░░░░░░░░░░░░░░░] 0%
  └─ Day 1-2: ONNX Setup         [░░░░░░░░░░░░░░░░]
  └─ Day 3-4: Classification     [░░░░░░░░░░░░░░░░]
  └─ Day 5: CLI Integration      [░░░░░░░░░░░░░░░░]
Phase 2: HTTP API (3 days)       [░░░░░░░░░░░░░░░░] NOT STARTED
Phase 3: Deployment (3 days)     [░░░░░░░░░░░░░░░░] NOT STARTED

Total Estimated: 11 days for full deployment
```

---

## Dependencies

### External Dependencies
- **ONNX Runtime** (1.16) - Fast ML inference
- **HuggingFace Hub** - Model hosting
- **CUDA** (11.8+, optional) - GPU acceleration
- **Rust** (1.70+) - Language toolchain
- **Unilang** (0.30) - CLI framework

### Hardware Dependencies
- **GPU** (optional) - RTX 3060 or better for GPU acceleration
- **8GB+ VRAM** (optional) - For GPU inference
- **16GB+ RAM** - System memory
- **20GB+ Storage** - Model downloads and Docker images

### Critical Path
1. ONNX Runtime installation → Model loading
2. Model loading → Classification
3. Classification → CLI interface
4. CLI working → HTTP API
5. HTTP API → Docker containerization
6. Container → Cloud deployment

---

## Risk Assessment

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| ONNX Runtime integration complexity | HIGH | Medium | Use official bindings, thorough testing |
| Model ONNX format compatibility | MEDIUM | Low | Download from official ProtectAI repo |
| Performance slower than expected | MEDIUM | Medium | Benchmark early, GPU optimization |
| Docker image size too large | LOW | Medium | Multi-stage builds, layer optimization |
| Vast.ai deployment issues | MEDIUM | Low | Test locally first, document thoroughly |
| GPU availability/cost on cloud | MEDIUM | Medium | Document CPU fallback, multiple platforms |

---

## Success Metrics

### Phase 1 Success Criteria
- ✅ Model loads via ONNX Runtime (verify with logs)
- ✅ Classification produces accurate results (95%+ accuracy)
- ✅ CLI accepts input and returns correct labels
- ✅ Inference latency < 100ms for typical inputs
- ✅ Memory usage reasonable (< 2GB for model)

### Phase 2 Success Criteria
- ✅ HTTP server starts successfully
- ✅ Health endpoint returns 200 OK
- ✅ Detect endpoint produces valid JSON
- ✅ Handles 10+ concurrent requests correctly
- ✅ Error responses are properly formatted

### Phase 3 Success Criteria
- ✅ Docker image builds in < 10 minutes
- ✅ Container runs on Vast.ai successfully
- ✅ Model downloads and caches properly
- ✅ Service accessible via public IP
- ✅ Costs < $0.50/hour on Vast.ai

### Overall MVP Success
- ✅ Proves ONNX Runtime is viable for ML inference
- ✅ Detects prompt injection with 95%+ accuracy
- ✅ Can be deployed to production
- ✅ Code passes all tests
- ✅ Documentation is complete

---

## Decision Points

### After Phase 1
**Decision:** Continue with HTTP API or pivot?

**Continue if:**
- Classification accuracy meets target (95%+)
- Performance is acceptable (< 100ms)
- ONNX Runtime is stable
- No major blockers found

**Pivot if:**
- ONNX Runtime has critical issues
- Performance is unusable
- Model compatibility problems
- Better alternatives discovered

### After Phase 2
**Decision:** Deploy to cloud or keep local-only?

**Deploy if:**
- HTTP API is stable
- Performance scales with concurrency
- Use case requires cloud deployment
- Budget allows for cloud hosting

**Keep local if:**
- MVP meets requirements without cloud
- Cost is prohibitive
- Security/privacy concerns
- Local deployment sufficient

---

## Next Steps

### Immediate Actions (Start Phase 1)

1. **Install ONNX Runtime:**
   ```bash
   cargo add ort
   ```

2. **Download DeBERTa Model:**
   - Visit https://huggingface.co/protectai/deberta-v3-base-prompt-injection-v2
   - Download ONNX model files
   - Place in `models/` directory

3. **Begin Model Loading Implementation:**
   - Edit `module/injection_core/src/model.rs`
   - Implement ModelLoader::new()
   - Test ONNX session initialization

4. **Track Progress:**
   - Update this roadmap daily
   - Check off completed tasks
   - Document blockers immediately

---

## References

- **Specification:** [spec.md](./spec.md)
- **Rulebook:** [rulebook.md](./rulebook.md)
- **Decisions:** [decisions.md](./decisions.md)
- **ProtectAI Model:** https://huggingface.co/protectai/deberta-v3-base-prompt-injection-v2
- **ONNX Runtime Docs:** https://onnxruntime.ai/docs/
- **Unilang Crate:** https://crates.io/crates/unilang

---

## Change Log

- **2025-10-25:** Project pivoted to prompt injection detection, added Phase 3 (Container Deployment), updated dependencies to ONNX Runtime
- **2025-10-22:** Roadmap created, Phase 0 marked complete
- **2025-10-21:** Project initialized, foundation laid

---

**Note:** This roadmap follows the ultra-minimal MVP philosophy. Each phase must be completed and validated before proceeding to the next. Container deployment (Phase 3) is the final milestone for production readiness.
