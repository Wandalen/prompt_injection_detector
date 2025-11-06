# spec

- **Project Name:** prompt_injection_detector
- **Version:** 0.3
- **Date:** 2025-11-02
- **Type:** System Specification (Security Detection System)
- **Scope:** Phase 1-3 (Core Detection, HTTP API, Container Deployment)

---

## Table of Contents

1. [Vocabulary](#vocabulary)
2. [Project Goal](#project-goal)
3. [Problem Solved](#problem-solved)
4. [User Stories](#user-stories)
5. [System Actors](#system-actors)
6. [Functional Requirements](#functional-requirements)
7. [Non-Functional Requirements](#non-functional-requirements)
8. [System Architecture](#system-architecture)
9. [Component Diagrams](#component-diagrams)
10. [External Dependencies](#external-dependencies)
11. [Technology Stack](#technology-stack)
12. [In Scope](#in-scope)
13. [Out of Scope](#out-of-scope)
14. [Deliverables](#deliverables)
15. [Success Metrics](#success-metrics)
16. [Milestones](#milestones)
17. [Appendix](#appendix)

---

## Vocabulary

This section defines the Ubiquitous Language for the prompt_injection_detector project. All terms must be used consistently throughout the specification, source code, and project communications.

- **Prompt Injection:** A security attack where malicious text is crafted to manipulate or override the intended behavior of a language model by injecting commands or instructions.
- **Detection:** The process of using a trained machine learning model to classify input text as either benign or containing a prompt injection attack.
- **Backend:** The inference engine (ORT or Burn) that executes model computations for classification.
- **Binary Classification:** The task of categorizing input text into exactly two classes: benign (0) or injection (1).
- **ONNX:** Open Neural Network Exchange, a standard format for representing machine learning models enabling cross-platform inference.
- **ORT Backend:** ONNX Runtime with CUDA support - production-grade, primary backend with 8-12ms inference latency.
- **Burn Backend:** Rust-native ML framework with CUDA support - experimental alternative with 170ms inference latency.
- **Lazy Initialization:** Loading model on first `detect()` call rather than at startup, reducing initial overhead.
- **Model Caching:** Storing loaded model in memory (via RwLock/Mutex) for fast subsequent inferences (60-267x speedup).
- **ModernBERT:** The base architecture for the prompt injection classification model (22 layers, 768 hidden size, 512 max tokens).
- **Benign:** Classification label (0) indicating the input text does not contain a prompt injection attack.
- **Injection:** Classification label (1) indicating the input text likely contains a prompt injection attack.
- **Thread-Safe:** Property of `detect()` function allowing concurrent calls from multiple threads safely.

---

## Project Goal

**Detect prompt injection attacks with 95%+ accuracy using dual backend architecture (ORT/Burn) for fast, production-ready inference.**

Load ModernBERT-based model, classify text as benign or injection, deploy to production. Provide complete detection system deployable as CLI tool or HTTP API service. That's it.

---

## Problem Solved

**Security Need:** Protect LLM applications from prompt injection attacks that can manipulate model behavior, extract sensitive information, or bypass safety guardrails.

Provide a production-ready, high-performance detection system that can be deployed as a standalone service or integrated into existing applications. Enable developers to validate user inputs before sending to LLM systems.

**Solution:** High-performance binary classifier using ModernBERT-based model with:
- Sub-100ms inference latency (ORT backend, cached)
- Thread-safe concurrent processing
- Dual backend support (production + experimental)
- Simple API: `detect(text) -> Result<String>` returning "benign" or "injection"

---

## User Stories

### US-1: Security Engineer - Real-Time Detection
**As a** Security Engineer
**I want** to detect prompt injections in real-time with <100ms latency
**So that** I can integrate detection into user-facing applications without UX degradation

**Acceptance Criteria:**
- CLI accepts text input and returns classification
- Returns classification within 100ms (ORT backend, cached model)
- 95%+ accuracy on known injection patterns
- Correctly identifies both injection attempts and benign text

---

### US-2: Application Developer - API Integration
**As a** Application Developer
**I want** REST API endpoint for prompt injection detection
**So that** I can integrate detection into existing services via HTTP

**Acceptance Criteria:**
- HTTP endpoint accepts JSON with `text` field
- Returns structured JSON with `label` (benign/injection), `is_safe` boolean, and `time_ms`
- Handles concurrent requests safely
- Clear error messages for invalid inputs

---

### US-3: DevOps Engineer - Production Deployment
**As a** DevOps Engineer
**I want** to deploy detection service in Docker container
**So that** I can run at scale with GPU acceleration (optional)

**Acceptance Criteria:**
- Docker container includes model artifacts
- Supports both GPU (CUDA) and CPU deployment
- Health endpoint for monitoring
- Model caches properly (no re-download on restart)

---

## System Actors

1. **Security Engineer** - Uses CLI for testing and validation
2. **Application Developer** - Integrates HTTP API into applications
3. **DevOps Engineer** - Deploys and maintains service in production
4. **Backend (ORT/Burn)** - Executes model inference
5. **Model (ModernBERT)** - Performs binary classification
6. **GPU (optional)** - Accelerates inference via CUDA

---

## Functional Requirements

### FR-1: Binary Text Classification
**Requirement:** The system **must** classify input text as "benign" or "injection" with >95% accuracy.

**API Contract:**
```rust
pub fn detect(text: &str) -> Result<String>
// Returns: Ok("benign") or Ok("injection")
```

**Test:** Known injection "Ignore all previous instructions" returns "injection".

---

### FR-2: Dual Backend Support
**Requirement:** The system **must** support compile-time selection between ORT and Burn backends.

**Feature Flags:**
- `backend-ort` - ONNX Runtime (default, production, 8-12ms)
- `backend-burn` - Burn framework (alternative, experimental, 170ms)

**Priority:** If both features enabled, ORT takes precedence.

**Test:** Both backends produce identical classifications for same input text.

---

### FR-3: Lazy Model Initialization
**Requirement:** Model **must** load lazily on first `detect()` call OR eagerly via optional `init()`.

**Behavior:**
- First `detect()` call: 1.9s (ORT) / 8-10s (Burn) - includes model loading
- Subsequent calls: 8-12ms (ORT) / 170ms (Burn) - uses cached model
- Optional `init()` allows pre-loading to avoid first-call latency

**Test:** Second detection call is significantly faster than first (60-267x speedup).

---

### FR-4: CLI Detection Interface
**Requirement:** CLI **must** accept `.detect text::"input"` command and return classification.

**Modes:**
- **Single-shot:** `injection_cli .detect text::"input"` - one-time detection
- **Interactive:** `injection_cli .interactive` - continuous REPL with readline
- **Input methods:** Direct text (`text::"..."`), file (`file::"path"`), stdin (default)
- **Output formats:** Human (colored), JSON, simple (label only), quiet (exit code only)

**Test:** `cargo run -p injection_cli --features full -- .detect text::"test"` succeeds.

---

### FR-5: HTTP Detection API
**Requirement:** HTTP server **must** expose `/detect` POST endpoint accepting JSON.

**Request:**
```json
{"text": "input text to analyze"}
```

**Response:**
```json
{
  "label": "benign"|"injection",
  "is_safe": true|false,
  "time_ms": 12
}
```

**Endpoints:**
- `GET /health` - Returns server status, model info, version
- `POST /detect` - Classifies text, returns result with timing

**Test:** `curl -X POST http://localhost:3000/detect -d '{"text":"test"}'` returns valid JSON.

---

## Non-Functional Requirements

### NFR-1: Performance
**Requirement:** Inference latency **must** be <100ms for typical inputs (ORT backend, cached model).

**Measured:** ORT achieves 8-12ms (exceeds target), Burn achieves 170ms (within experimental tolerance).

---

### NFR-2: Concurrency
**Requirement:** System **must** handle concurrent requests safely without data races.

**Implementation:**
- ORT backend: RwLock allows concurrent reads (parallel request processing)
- Burn backend: Mutex serializes access (one request at a time)

---

### NFR-3: Memory Usage
**Requirement:** System **must** operate within 2GB RAM for model and runtime.

**Measured:** ~2GB for loaded model + runtime (both backends).

---

## System Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────┐
│          User Interfaces                    │
│  ┌──────────────┐    ┌──────────────────┐  │
│  │ CLI          │    │ HTTP Server      │  │
│  │ (injection_  │    │ (injection_      │  │
│  │  cli)        │    │  server)         │  │
│  └──────┬───────┘    └────────┬─────────┘  │
└─────────┼─────────────────────┼─────────────┘
          │                     │
          └──────────┬──────────┘
                     ▼
        ┌─────────────────────────────┐
        │  injection_core Library     │
        │  ┌───────────────────────┐  │
        │  │ Backend Selector      │  │
        │  │ (compile-time)        │  │
        │  └──────┬────────┬───────┘  │
        │         │        │           │
        │    ┌────▼───┐ ┌──▼────┐     │
        │    │  ORT   │ │ Burn  │     │
        │    │Backend │ │Backend│     │
        │    │RwLock  │ │Mutex  │     │
        │    └────┬───┘ └──┬────┘     │
        │         │        │           │
        │    ┌────▼────────▼─────┐    │
        │    │ Lazy Init Cache   │    │
        │    └──────────────────┘     │
        └─────────────────────────────┘
                     │
              ┌──────▼───────┐
              │  CUDA / CPU  │
              │ Acceleration │
              └──────────────┘
```

### Architecture Style

Layered monolithic application with clear separation between:
1. **Interface Layer:** CLI and HTTP API entry points
2. **Core Library Layer:** Backend abstraction and API (`detect()`, `init()`)
3. **Backend Layer:** ORT or Burn inference engines
4. **Model Layer:** ModernBERT-based binary classifier

### Component Responsibilities

**injection_cli:**
- Argument parsing (manual parsing, not unilang framework)
- Multiple input methods (text/file/stdin)
- Multiple output formats (human/JSON/simple/quiet)
- Interactive REPL mode with readline history
- Exit codes (0=benign, 1=injection for quiet mode)

**injection_server:**
- HTTP endpoint routing (Axum framework)
- JSON request/response serialization
- Pre-loads model at startup via `init()` for fast first request
- No shared state needed (detect() is thread-safe via RwLock/Mutex)
- Tracing/logging for production monitoring

**injection_core:**
- Backend abstraction layer (compile-time selection)
- Model caching with thread safety (RwLock for ORT, Mutex for Burn)
- Lazy initialization pattern (load on demand)
- Public API: `detect(text) -> Result<String>` and `init() -> Result<()>`
- Zero runtime overhead (static dispatch via cfg attributes)

**Backend (ORT):**
- ONNX Runtime inference with CUDA acceleration
- RwLock for concurrent read access (multiple threads can detect() simultaneously)
- 8-12ms inference latency (cached model)
- Production-grade performance and stability

**Backend (Burn):**
- Burn framework inference with CUDA acceleration
- Mutex for exclusive access (serialized requests)
- 170ms inference latency (cached model)
- Experimental/research use, not production-recommended

---

## Component Diagrams

### Diagram 1: Request Flow (CLI)

```
User → CLI Entry → detect(text) → Backend Selector (compile-time)
                                        ↓
                            ┌───────────┴───────────┐
                            │                       │
                         ORT Backend          Burn Backend
                         (RwLock)              (Mutex)
                            │                       │
                       Load/Cache              Load/Cache
                       ModernBERT             ModernBERT
                            │                       │
                         Classify              Classify
                            │                       │
                       "benign" or           "benign" or
                       "injection"           "injection"
                            │                       │
                            └───────────┬───────────┘
                                        ↓
                                  Return Result
                                        ↓
                                  Display Output
```

### Diagram 2: Request Flow (HTTP)

```
HTTP Request → Axum Router → detect_handler → detect(text)
                                                    ↓
                                              Backend (ORT/Burn)
                                                    ↓
                                              Classification
                                                    ↓
                                            JSON Response
                                            {label, is_safe, time_ms}
                                                    ↓
                                              HTTP 200 OK
```

---

## External Dependencies

### Dependency 1: ONNX Runtime (ORT Backend)
- **Library:** ort (2.0.0-rc.10)
- **Purpose:** Execute ONNX model inference with CUDA acceleration
- **Required For:** ORT backend (default)
- **Risk:** MEDIUM - New 2.0 API requires unsafe code for session.run()

### Dependency 2: Burn Framework (Burn Backend)
- **Library:** burn (0.19.0), burn-ndarray (0.19.0)
- **Purpose:** Rust-native ML inference with CUDA support
- **Required For:** Burn backend (alternative)
- **Risk:** MEDIUM - Relatively new framework, experimental status

### Dependency 3: HuggingFace Tokenizers
- **Library:** tokenizers (0.22.1)
- **Purpose:** Text tokenization for ModernBERT (max 512 tokens)
- **Required For:** Both backends
- **Risk:** LOW - Stable, widely used library

### Dependency 4: Axum HTTP Framework
- **Library:** axum (0.7), tokio (1.x)
- **Purpose:** HTTP server for API endpoints
- **Required For:** injection_server (Phase 2)
- **Risk:** LOW - Mature, production-ready framework

---

## Technology Stack

| Component | Technology | Version | Rationale |
|-----------|-----------|---------|-----------|
| **Programming Language** | Rust | 1.70+ | Memory safety, performance, zero-cost abstractions |
| **Primary Backend** | ONNX Runtime | 2.0.0-rc.10 | Production-grade, 8-12ms latency, CUDA support |
| **Alternative Backend** | Burn | 0.19.0 | Rust-native ML, experimental/research use |
| **HTTP Server** | Axum | 0.7.x | Modern async framework, ergonomic API |
| **Async Runtime** | Tokio | 1.x | Industry standard for async Rust |
| **CLI Parsing** | Manual | - | Simple argument parsing (not unilang framework) |
| **GPU Runtime** | CUDA | 12.x+ | NVIDIA GPU acceleration (optional, CPU fallback) |
| **Tokenization** | tokenizers | 0.22.1 | HuggingFace tokenizers for text preprocessing |
| **Error Handling** | anyhow | 1.0.100 | Ergonomic error propagation |
| **Serialization** | serde/serde_json | 1.x | JSON request/response handling |

---

## In Scope

**Phase 1 (Complete):**
- ✅ Dual backend implementation (ORT + Burn)
- ✅ Lazy initialization with model caching
- ✅ CLI with interactive and single-shot modes
- ✅ Binary classification (benign/injection)
- ✅ Thread-safe concurrent access
- ✅ Multiple input/output formats
- ✅ Performance optimization (60-267x speedup via caching)

**Phase 2 (Next):**
- HTTP API server implementation
- `/health` and `/detect` endpoints
- Concurrent request handling
- Production logging and monitoring
- API integration tests

**Phase 3 (Future):**
- Docker containerization
- Cloud deployment guides (Vast.ai, RunPod, AWS)
- Performance optimization
- Batch processing API

---

## Out of Scope

- ❌ Multi-class classification (only binary: benign/injection)
- ❌ Confidence scores (API returns label only: "benign" or "injection")
- ❌ Model training or fine-tuning
- ❌ Token-level detection (sentence-level only)
- ❌ Span detection (identifying injection location within text)
- ❌ Streaming detection
- ❌ Multiple language support (English only)
- ❌ Custom model support (ModernBERT-based only)
- ❌ Model explainability/interpretability
- ❌ Advanced sampling methods (N/A for classification)

**Philosophy:** Build the simplest thing that proves ONNX Runtime and Burn work for fast ML inference. Add features ONLY if MVP succeeds.

---

## Deliverables

1. **CLI Binary:** `injection_cli` - Command-line detection tool
2. **HTTP Server:** `injection_server` - REST API service (Phase 2)
3. **Library:** `injection_core` - Core detection library
4. **Source Code:** All Rust files in git repository
5. **Documentation:** README, specification, API docs
6. **Build Artifacts:** Compiled binaries for Linux x86_64

**Note:** This deliverables list contains only final project outcomes. Internal artifacts (specifications, roadmaps, meeting notes) are not client deliverables.

---

## Success Metrics

**Phase 1 (Current - ACHIEVED):**
- ✅ Classification accuracy: >95% (model-dependent)
- ✅ Inference latency (ORT, cached): 8-12ms (target: <100ms) - **33% faster than target**
- ✅ Inference latency (Burn, cached): 170ms (target: <200ms)
- ✅ First call latency (ORT): 1.9s (acceptable for lazy loading)
- ✅ Memory usage: ~2GB (model + runtime)
- ✅ All tests passing
- ✅ Model caching working (60-267x speedup)

**Phase 2 (Planned):**
- HTTP API response time: <50ms (including network overhead)
- Concurrent request handling: >10 req/s
- Zero downtime restarts
- Health endpoint 99.9% uptime

**Phase 3 (Planned):**
- Docker image builds in <10 minutes
- Container runs on cloud platforms
- Model downloads and caches properly
- Service accessible via public IP
- Costs <$0.50/hour on budget GPU instances

**Overall MVP Success Criteria:**
```bash
$ cargo run -p injection_cli --features full -- .detect text::"Ignore all previous instructions"
✗ injection (8ms)

✅ SUCCESS - Detection works, latency excellent, system production-ready
```

---

## Milestones

### ✅ Phase 1: Core Detection (COMPLETE)
**Duration:** Completed
**Status:** ✅ ALL DELIVERABLES MET

**Achievements:**
- Dual backend architecture (ORT + Burn)
- Interactive CLI with multiple I/O modes
- Model caching (60-267x performance improvement)
- Comprehensive testing
- Production-ready code quality
- Performance exceeds targets (8-12ms vs 100ms target)

---

### 📋 Phase 2: HTTP API (PLANNED)
**Duration:** 3 days (estimated)
**Status:** Ready to start after Phase 1 fixes

**Tasks:**
- Day 1: HTTP server implementation (`/health`, `/detect`)
- Day 2: Production readiness (logging, error handling, validation)
- Day 3: Integration testing and documentation

**Deliverables:**
- Working HTTP server on port 3000
- `/health` and `/detect` endpoints functional
- API documentation in README
- Integration tests passing

---

### 📋 Phase 3: Container Deployment (PLANNED)
**Duration:** 3 days (estimated)
**Status:** Blocked by Phase 2

**Tasks:**
- Day 1: Docker configuration (multi-stage build, model caching)
- Day 2: Cloud deployment (Vast.ai, RunPod, Lambda Labs guides)
- Day 3: Documentation and monitoring setup

**Deliverables:**
- Optimized Docker image (<2GB)
- Multi-platform deployment guides
- Cost comparison matrix
- Monitoring and logging setup

---

## Appendix

### Performance Benchmarks

| Metric | ORT Backend | Burn Backend |
|--------|-------------|--------------|
| **First Call (Cold)** | 1.9s | 8-10s |
| **Subsequent Calls (Cached)** | 8-12ms | 170ms |
| **Speedup (Cached vs Cold)** | 267x | 60x |
| **Model Size** | 572MB (ONNX) | 286MB (MPK) |
| **Memory Usage (Runtime)** | ~2GB | ~2GB |
| **Concurrency** | Parallel (RwLock) | Serial (Mutex) |
| **CUDA Support** | ✅ Yes | ✅ Yes |
| **CPU Fallback** | ✅ Yes | ✅ Yes |
| **Production Ready** | ✅ Yes | ⚠️ Experimental |

### Concurrency Characteristics

**ORT Backend (Production):**
- Model shared via `RwLock<Option<Detector>>`
- **Concurrent reads:** ✅ YES - Multiple threads can call `detect()` simultaneously
- **Performance:** Optimal for HTTP server (parallel request processing)
- **Initialization:** Exclusive write lock (blocks all during first call)
- **Recommendation:** Use for production deployments requiring high throughput

**Burn Backend (Experimental):**
- Model shared via `Mutex<Option<Detector>>`
- **Concurrent reads:** ❌ NO - Serialized access (one request at a time)
- **Performance:** Lower throughput for concurrent workloads
- **Reason:** Burn's `InferenceContext` is not `Sync`
- **Recommendation:** Use for research, benchmarking, or low-traffic scenarios

### Dependencies

**Core:**
- ort (2.0.0-rc.10) - ONNX Runtime with CUDA
- burn (0.19.0) - Burn ML framework with CUDA
- tokenizers (0.22.1) - HuggingFace tokenizers
- once_cell (1.19) - Lazy static initialization
- anyhow (1.0.100) - Error handling
- ndarray (0.16) - Array operations (ORT)
- cudarc (0.17.6) - CUDA bindings (Burn)

**CLI:**
- rustyline (14.0) - Interactive readline with history
- colored (2.1) - Terminal colors
- serde_json - JSON output format

**HTTP (Phase 2):**
- axum (0.7) - HTTP framework
- tokio (1.x) - Async runtime
- tower - Middleware
- tower-http - HTTP utilities
- tracing - Structured logging
- tracing-subscriber - Log formatting

### File Structure

**Final Project Structure:**
```
prompt_injection_detector/
├── Cargo.toml              # Workspace manifest
├── readme.md               # Project overview
├── spec.md                 # This specification
├── roadmap.md              # Development roadmap
├── decisions.md            # Design decisions log
├── rulebook.md             # Project-specific rules
├── Dockerfile              # Container configuration
├── Makefile                # Build automation
├── .gitignore              # Git exclusions
├── license-mit             # MIT license
├── secret/                 # Secret management (gitignored)
│   ├── readme.md           # Secret management guide
│   └── -hf_token.template  # HuggingFace token template
├── module/                 # Crate modules
│   ├── injection_core/     # Core detection library
│   │   ├── src/
│   │   │   ├── lib.rs      # Public API
│   │   │   ├── backend/    # Backend abstraction
│   │   │   │   ├── mod.rs  # Selector
│   │   │   │   ├── ort.rs  # ORT backend
│   │   │   │   └── burn.rs # Burn backend
│   │   │   └── burn_impl/  # Burn implementation
│   │   ├── tests/          # Integration tests
│   │   │   ├── readme.md   # Test documentation
│   │   │   ├── test_ort_backend.rs
│   │   │   ├── test_burn_backend.rs
│   │   │   └── test_lazy_init.rs
│   │   ├── build.rs        # Build script (Burn MPK generation)
│   │   ├── Cargo.toml      # Crate manifest
│   │   └── readme.md       # Crate documentation
│   ├── injection_cli/      # CLI binary
│   │   ├── src/main.rs     # CLI implementation
│   │   ├── Cargo.toml
│   │   └── readme.md
│   └── injection_server/   # HTTP server binary
│       ├── src/main.rs     # Server implementation
│       ├── Cargo.toml
│       └── readme.md
└── artifacts/              # Model files (not in git)
    ├── model.onnx          # ONNX format for ORT
    ├── model.mpk           # Burn format (auto-generated)
    └── tokenizer/
        └── tokenizer.json  # Tokenizer configuration
```

---

**END OF SPECIFICATION**

---

**Version History:**
- v0.3 (2025-11-02): Complete rewrite for prompt injection detection (removed vllm_inferencer content)
- v0.2 (2025-10-25): Project pivoted to prompt injection detection
- v0.1 (2025-10-21): Initial vllm_inferencer specification
