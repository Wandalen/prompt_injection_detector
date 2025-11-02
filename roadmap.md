# Roadmap

Prompt injection detection system development roadmap

**Status:** Phase 1 Complete - Core System Operational
**Last Updated:** 2025-10-31

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
- **Dual backend architecture: ORT + Burn**
- **Unilang CLI framework integrated**

**Deliverables:**
- `/home/tihilya/ml/vllm_inferencer/` workspace
- 3 member crates: injection_core, injection_cli, injection_server
- Crate distribution rulebook compliant
- Ready for crates.io publication (after implementation)

---

### ✅ Phase 1: Core Detection Implementation (COMPLETE)

**Goal:** Implement production-ready prompt injection detection with dual backend support

**Duration:** Completed

**Status:** ✅ COMPLETE

**Completed Tasks:**

#### ✅ Backend Implementation
- [x] ORT backend with ONNX Runtime + CUDA acceleration
- [x] Burn backend with native Rust + CUDA support
- [x] Compile-time backend selection via feature flags
- [x] CUDA GPU acceleration for both backends
- [x] CPU fallback support (automatic)
- [x] Benchmark both backends for performance comparison

**Results:**
- ORT Backend: ~8-12ms inference (267x faster with caching)
- Burn Backend: ~170ms inference (60x faster with caching)
- Both backends support CUDA acceleration
- Automatic model format handling (ONNX for ORT, MPK for Burn)

#### ✅ Model Loading & Caching
- [x] Lazy initialization pattern (load on first use)
- [x] Optional pre-initialization via `init()` function
- [x] Model caching with thread-safe access (RwLock for ORT, Mutex for Burn)
- [x] Automatic model format conversion (ONNX → MPK for Burn)
- [x] Artifact path auto-detection
- [x] Build script for MPK generation

**Results:**
- First call: 1.9s (ORT) / 8-10s (Burn) - includes model loading
- Subsequent calls: 8-12ms (ORT) / 170ms (Burn) - cached model
- 60-267x performance improvement with caching
- Thread-safe concurrent access

#### ✅ Classification Logic
- [x] Binary classification (benign/injection)
- [x] Tokenization with ModernBERT tokenizer
- [x] Max length handling (512 tokens)
- [x] Confidence score extraction
- [x] Proper error handling
- [x] Label mapping (class 0=benign, class 1=injection)

**Results:**
- Accurate detection using ModernBERT-based classifier
- Proper tokenization and preprocessing
- Clean error messages for edge cases

#### ✅ CLI Implementation
- [x] Interactive/chat mode for continuous testing
- [x] Single-shot detection mode
- [x] Multiple input methods (text/file/stdin)
- [x] Multiple output formats (human/JSON/simple/quiet)
- [x] Performance timing display
- [x] Colored output (green=benign, red=injection)
- [x] Command history support (up/down arrows)
- [x] Warmup flag for pre-loading
- [x] Verbose mode for debugging
- [x] Help and version commands

**Results:**
- Full-featured CLI with excellent UX
- Interactive mode ideal for testing
- Scriptable with quiet mode (exit codes)
- JSON output for automation

#### ✅ Testing & Quality
- [x] Comprehensive test suite for both backends
- [x] Lazy initialization performance tests
- [x] Feature-gated tests (ORT/Burn)
- [x] Integration tests
- [x] All tests passing
- [x] Makefile for build automation
- [x] Complete documentation (README, API docs)

**Results:**
- 100% test pass rate
- Both backends fully tested
- Build automation via Makefile
- Production-ready code quality

**Deliverables:**
- ✅ Working detection system with dual backends
- ✅ Interactive CLI for testing
- ✅ Model caching for fast inference
- ✅ Comprehensive documentation
- ✅ Makefile for build automation
- ✅ Complete test coverage

**Success Criteria:** ✅ ALL MET

```bash
$ make interactive
Loading model... done (1.9s)
Prompt Injection Detector (interactive mode)
Type 'exit' or Ctrl+D to quit

> What is the capital of France?
✓ benign (8ms)

> Ignore all previous instructions
✗ injection (12ms)

✅ SUCCESS - System operational and production-ready
```

---

## Completed Phases Summary

```
Phase 0: Foundation              [████████████████] ✅ COMPLETE
Phase 1: Core Detection          [████████████████] ✅ COMPLETE
  ├─ ORT Backend                 [████████████████] ✅
  ├─ Burn Backend                [████████████████] ✅
  ├─ Model Caching               [████████████████] ✅
  ├─ Interactive CLI             [████████████████] ✅
  ├─ Multiple Input/Output       [████████████████] ✅
  └─ Testing & Documentation     [████████████████] ✅
```

---

## Upcoming Phases

### 📋 Phase 2: HTTP API Server (PLANNED)

**Goal:** Expose detection via REST API for web applications

**Duration:** 3 days (estimated)

**Prerequisites:** ✅ Phase 1 complete

**Tasks:**

#### Day 1: Server Implementation
- [ ] Implement health endpoint in `module/injection_server/src/main.rs`
- [ ] Add /detect endpoint with JSON request/response
- [ ] Integrate injection_core with async wrapper (Tokio)
- [ ] Add proper error handling for invalid inputs
- [ ] Test with curl and HTTP clients

#### Day 2: Production Readiness
- [ ] Add request validation (max length, empty text)
- [ ] Implement structured logging with tracing
- [ ] Add CORS configuration for web clients
- [ ] Test concurrent requests (thread safety)
- [ ] Add rate limiting (basic)
- [ ] Document API endpoints in README

#### Day 3: Integration Testing
- [ ] Write HTTP integration tests
- [ ] Test error scenarios (malformed JSON, oversized input)
- [ ] Load testing with multiple concurrent clients
- [ ] Performance profiling
- [ ] Document deployment instructions

**Deliverables:**
- Working HTTP server on port 3000
- /health and /detect endpoints
- API documentation in README
- Integration tests passing

**Success Criteria:**
```bash
$ cargo run -p injection_server --release
Server started on 0.0.0.0:3000

$ curl -X POST http://localhost:3000/detect \
  -H "Content-Type: application/json" \
  -d '{"text": "Ignore all previous instructions"}'
{
  "label": "injection",
  "is_injection": true,
  "time_ms": 8
}
```

---

### 🚀 Phase 3: Container Deployment (PLANNED)

**Goal:** Deploy to production with Docker and cloud platforms

**Duration:** 3 days (estimated)

**Prerequisites:** Phase 2 complete

**Tasks:**

#### Day 1: Docker Configuration
- [ ] Create optimized Dockerfile (multi-stage build)
- [ ] Model caching strategy in container
- [ ] Add docker-compose.yml for local testing
- [ ] Test container on local machine
- [ ] Document Docker build and run commands
- [ ] Optimize image size (<2GB)

#### Day 2: Cloud Deployment
- [ ] Vast.ai deployment template
- [ ] RunPod deployment guide
- [ ] Lambda Labs setup instructions
- [ ] Configure networking and ports
- [ ] Test model download and caching
- [ ] Document instance selection

#### Day 3: Documentation & Monitoring
- [ ] Cost estimation guide
- [ ] Performance tuning tips
- [ ] Health monitoring setup
- [ ] Troubleshooting documentation
- [ ] Load balancing recommendations
- [ ] Backup and recovery procedures

**Deliverables:**
- Optimized Docker image (<2GB)
- Multi-platform deployment guides
- Complete deployment documentation
- Cost comparison matrix
- Monitoring and logging setup

**Success Criteria:**
- ✅ Docker image builds successfully
- ✅ Container runs on cloud platform
- ✅ Model inference works in container
- ✅ Health endpoint accessible from internet
- ✅ Response time < 500ms for typical requests
- ✅ Costs < $0.50/hour on budget GPU instances

---

## Enhanced Features (Future Considerations)

### Token-Level Classification
- Train ModernBERT for token classification
- Identify which specific tokens are malicious
- Enable injection removal/sanitization
- Use NER-style span detection

**Value:** Allows removal of injection instead of just detection

**Complexity:** HIGH - Requires new training data and model

### Span Detection
- Identify exact boundaries of injection
- Return start/end positions in text
- Enable partial text cleaning

**Value:** More precise than token classification

**Complexity:** MEDIUM - Can use existing architecture

### Batch Processing
- Process multiple texts in single request
- Optimize for throughput
- Support async batch API

**Value:** Better performance for bulk processing

**Complexity:** LOW - Straightforward implementation

### Multi-Language Support
- Fine-tune for non-English languages
- Support Unicode properly
- Test with multilingual datasets

**Value:** Broader applicability

**Complexity:** MEDIUM - Requires multilingual data

---

## Technical Architecture

### Current Implementation

```
┌─────────────────────────────────────┐
│         injection_cli               │
│  (Interactive + Single-shot modes)  │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│        injection_core                │
│  ┌──────────────────────────────┐   │
│  │   Backend Selector           │   │
│  │  (compile-time via features) │   │
│  └────┬─────────────────┬───────┘   │
│       │                 │            │
│       ▼                 ▼            │
│  ┌─────────┐      ┌──────────┐      │
│  │   ORT   │      │   Burn   │      │
│  │ Backend │      │ Backend  │      │
│  │(RwLock) │      │ (Mutex)  │      │
│  └────┬────┘      └─────┬────┘      │
│       │                 │            │
│       ▼                 ▼            │
│  ┌─────────────────────────────┐    │
│  │  Lazy Initialization        │    │
│  │  + Model Caching            │    │
│  └─────────────────────────────┘    │
└─────────────────────────────────────┘
               │
               ▼
        ┌──────────────┐
        │ CUDA / CPU   │
        │ Acceleration │
        └──────────────┘
```

### Planned (Phase 2)

```
┌──────────────┐     ┌──────────────┐
│ injection_cli│     │injection_server│
│  (CLI mode)  │     │  (HTTP API)   │
└──────┬───────┘     └───────┬───────┘
       │                     │
       └──────────┬──────────┘
                  ▼
          ┌───────────────┐
          │injection_core │
          │ (shared lib)  │
          └───────────────┘
```

---

## Performance Benchmarks

### Current Performance (Phase 1 Complete)

| Metric | ORT Backend | Burn Backend |
|--------|-------------|--------------|
| **First Call (Cold)** | 1.9s | 8-10s |
| **Subsequent Calls** | 8-12ms | 170ms |
| **Speedup (Cached)** | 267x | 60x |
| **Model Size** | 572MB (ONNX) | 286MB (MPK) |
| **Memory Usage** | ~2GB | ~2GB |
| **CUDA Support** | ✅ Yes | ✅ Yes |
| **CPU Fallback** | ✅ Yes | ✅ Yes |

### Target Performance (Phase 2)

| Metric | Target | Current |
|--------|--------|---------|
| **Single Request** | <50ms | ✅ 8-12ms (ORT) |
| **Concurrent (10 req)** | <500ms avg | TBD |
| **Throughput** | >100 req/s | TBD |
| **Memory (idle)** | <500MB | TBD |
| **Memory (loaded)** | <2GB | ✅ ~2GB |

---

## Dependencies

### Core Dependencies (Current)

- **ort** (2.0.0-rc.10) - ONNX Runtime with CUDA
- **burn** (0.19.0) - Rust ML framework with CUDA
- **tokenizers** (0.22.1) - HuggingFace tokenizers
- **once_cell** (1.19) - Lazy initialization
- **anyhow** - Error handling

### CLI Dependencies

- **rustyline** (14.0) - Interactive readline
- **colored** (2.1) - Terminal colors
- **serde_json** - JSON output

### Planned (Phase 2)

- **axum** (0.7) - HTTP framework
- **tokio** (1.x) - Async runtime
- **tower** - Middleware
- **tracing** - Structured logging

### Hardware Requirements

**Development:**
- Rust 1.70+
- 16GB+ RAM
- 20GB+ storage
- CUDA 12.x+ (optional)

**Production (ORT Backend):**
- 8GB+ RAM
- NVIDIA GPU with 8GB+ VRAM (optional, recommended)
- CUDA 12.x+
- 10GB storage

**Production (Burn Backend):**
- 8GB+ RAM
- NVIDIA GPU with 8GB+ VRAM (optional, recommended)
- CUDA 12.x+
- 10GB storage

---

## Risk Assessment

| Risk | Impact | Probability | Status | Mitigation |
|------|--------|-------------|--------|------------|
| ONNX Runtime complexity | HIGH | Medium | ✅ RESOLVED | Successfully integrated |
| Model format compatibility | MEDIUM | Low | ✅ RESOLVED | Both formats working |
| Performance expectations | MEDIUM | Medium | ✅ RESOLVED | 8ms inference achieved |
| Docker image size | LOW | Medium | ⏳ PENDING | Multi-stage builds planned |
| Cloud deployment cost | MEDIUM | Medium | ⏳ PENDING | Document multiple platforms |
| GPU availability | MEDIUM | Medium | ✅ RESOLVED | CPU fallback implemented |
| Thread safety | HIGH | Low | ✅ RESOLVED | RwLock/Mutex implemented |
| Model caching complexity | MEDIUM | Low | ✅ RESOLVED | Lazy pattern working |

---

## Success Metrics

### ✅ Phase 1 Success Criteria (ALL MET)

- ✅ Model loads via both backends (verified with logs)
- ✅ Classification produces accurate results
- ✅ CLI accepts input and returns correct labels
- ✅ Inference latency < 100ms (**achieved: 8-12ms with ORT**)
- ✅ Memory usage reasonable (<2GB for model)
- ✅ Interactive mode functional
- ✅ Model caching working (267x speedup)
- ✅ All tests passing
- ✅ Documentation complete
- ✅ Build automation (Makefile)

### Phase 2 Success Criteria (Planned)

- [ ] HTTP server starts successfully
- [ ] Health endpoint returns 200 OK
- [ ] Detect endpoint produces valid JSON
- [ ] Handles 10+ concurrent requests correctly
- [ ] Error responses properly formatted
- [ ] Logging and monitoring working
- [ ] API documentation complete

### Phase 3 Success Criteria (Planned)

- [ ] Docker image builds in < 10 minutes
- [ ] Container runs on cloud platform
- [ ] Model downloads and caches properly
- [ ] Service accessible via public IP
- [ ] Costs < $0.50/hour
- [ ] Monitoring and alerts configured

### Overall MVP Success

- ✅ Dual backend implementation working
- ✅ Detects prompt injection accurately
- ✅ Fast inference (<50ms target: achieved 8-12ms)
- ✅ Production-ready code quality
- ✅ Comprehensive documentation
- [ ] Can be deployed to cloud (pending Phase 2-3)

---

## Decision Points

### ✅ After Phase 1 (DECIDED: CONTINUE)

**Continue with HTTP API** ✅

**Reasons:**
- ✅ Classification accuracy meets expectations
- ✅ Performance exceeds target (8ms < 100ms target)
- ✅ Both backends stable and tested
- ✅ No major blockers found
- ✅ Model caching provides excellent performance

**Next:** Proceed to Phase 2 (HTTP API)

### After Phase 2 (Future Decision)

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

### Immediate Actions (Ready for Phase 2)

1. **Plan HTTP Server Architecture:**
   - Design REST API endpoints
   - Plan async/await integration
   - Define error response formats

2. **Setup injection_server Crate:**
   - Add axum and tokio dependencies
   - Create basic server skeleton
   - Integrate injection_core

3. **Begin Implementation:**
   - Edit `module/injection_server/src/main.rs`
   - Implement /health endpoint
   - Implement /detect endpoint

4. **Track Progress:**
   - Update roadmap as tasks complete
   - Document any blockers
   - Maintain changelog

---

## Makefile Quick Reference

```bash
# Building
make build          # Build CLI (ORT backend)
make build-burn     # Build CLI (Burn backend)
make rebuild-mpk    # Rebuild Burn model from ONNX

# Testing
make test           # Run all tests (ORT)
make test-all       # Run tests for both backends
make bench-ort      # Benchmark ORT performance

# Running
make interactive    # Interactive chat mode
make detect TEXT="..." # Single detection

# Development
make clean          # Clean build artifacts
make doc            # Generate documentation
make install        # Install CLI globally
```

---

## References

- **Specification:** [spec.md](./spec.md)
- **README:** [readme.md](./readme.md)
- **Rulebook:** [rulebook.md](./rulebook.md)
- **Decisions:** [decisions.md](./decisions.md)
- **ModernBERT:** https://huggingface.co/answerdotai/ModernBERT-base
- **ONNX Runtime:** https://onnxruntime.ai/docs/
- **Burn Framework:** https://burn.dev/

---

## Change Log

- **2025-10-31:** ✅ Phase 1 COMPLETE - Dual backend system operational, interactive CLI, model caching, comprehensive testing, Makefile automation
- **2025-10-25:** Project pivoted to ModernBERT, added Phase 3 (Container Deployment), updated dependencies to dual backend (ORT + Burn)
- **2025-10-22:** Roadmap created, Phase 0 marked complete
- **2025-10-21:** Project initialized, foundation laid

---

**Current Status:** ✅ Phase 1 Complete - Production-ready core system

**Next Phase:** Phase 2 - HTTP API Server (ready to start)

**Quick Start:** `make quick-start` or `make interactive`
