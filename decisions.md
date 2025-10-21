# Specification Auto-Decisions Log

This file records all automatic decisions made during the specification creation process for vllm_inferencer.

---

## Decision 1: Ultra-Minimal MVP Scope

**Date:** 2025-10-21 (Updated)
**Category:** Scope

**Context:** User requested "simplest and easiest to reach" MVP goals after reviewing initial specification.

**Options Considered:**
1. **Single Milestone: Prove Candle Works (3-5 days)**
   - Pros: Absolute minimum, fast validation, low risk
   - Cons: Very limited functionality
2. **Three Milestones: CLI + HTTP + Cache (3 weeks)**
   - Pros: More features, production-ready
   - Cons: Too complex, high risk, 6x longer timeline
3. **Two Milestones: CLI + HTTP (2 weeks)**
   - Pros: Usable service
   - Cons: Still too ambitious for proof-of-concept

**Selected:** Option 1 - Single Milestone (3-5 days)

**Rationale:** User explicitly requested "make goals simpler and easier to reach." The ONLY goal is to prove Candle framework works for LLM inference. Everything else (HTTP, caching, optimization) can be added AFTER validating the core technology choice. This is true MVP thinking: validate the riskiest assumption first with minimum investment.

---

## Decision 2: GPT-2 Small Model

**Date:** 2025-10-21
**Category:** Scope

**Context:** Need to select which LLM model to use for the ultra-minimal MVP.

**Options Considered:**
1. **GPT-2 Small (124M parameters)**
   - Pros: Fast download (~500MB), fits in any GPU (2GB VRAM), well-supported in Candle, proven in examples
   - Cons: Not as capable as larger models, not the Ukrainian-specific model from original requirements
2. **Gemma-3B (3B parameters)**
   - Pros: More capable, closer to original requirements (MamayLM-Gemma-3-4B-IT)
   - Cons: Larger download (~12GB), requires more VRAM (12-16GB), slower inference, may not be in Candle examples
3. **Llama-7B (7B parameters)**
   - Pros: Very capable, widely used
   - Cons: Very large (~28GB FP32), requires significant VRAM (14-28GB), complex to implement

**Selected:** Option 1 - GPT-2 Small

**Rationale:** For ultra-minimal MVP focused on proving technical feasibility in 3-5 days, GPT-2 Small minimizes risk and maximizes learning speed. It's well-documented in Candle examples, fits in any modern GPU, and allows rapid iteration. The project can upgrade to Gemma-3B or MamayLM after the MVP proves the architecture works.

---

## Decision 3: Specification File Structure

**Date:** 2025-10-21
**Category:** File Structure

**Context:** Need to determine whether to use single-file (`spec.md`) or multi-file (`spec/` directory) structure for the specification.

**Options Considered:**
1. **Single-File Structure (`spec.md`)**
   - Pros: Simpler navigation, all content in one place, easier maintenance for small projects
   - Cons: Can become unwieldy for large projects
2. **Multi-File Structure (`spec/` directory)**
   - Pros: Better organization for complex systems with multiple independent components
   - Cons: More overhead, requires navigation between files, not needed for simple projects

**Selected:** Option 1 - Single-File Structure

**Rationale:** The vllm_inferencer ultra-minimal MVP is a standalone, single-component proof-of-concept with very limited scope (single milestone, 3-5 days). A single-file structure simplifies access and reduces complexity. Per rulebook: "Single-file structure simplifies navigation and maintenance for smaller projects where all specification content can be logically contained in one document without becoming unwieldy."

---

## Decision 4: Feature Scope (Drastically Reduced)

**Date:** 2025-10-21 (Updated)
**Category:** Scope

**Context:** Need to define which features are in-scope vs out-of-scope for the ultra-minimal MVP after user feedback to simplify.

**Options Considered:**
1. **Absolute Minimum (CLI text generation only)**
   - Pros: Achievable in 3-5 days, proves core concept only
   - Cons: No production features
2. **Minimal Scope (CLI + HTTP API)**
   - Pros: Usable service
   - Cons: 2x complexity, 2-3 weeks timeline
3. **Extended Scope (Add caching, sampling)**
   - Pros: Production-ready
   - Cons: 6-8 weeks, significantly more complex

**Selected:** Option 1 - Absolute Minimum

**Rationale:** After user feedback to "make goals simpler and easier to reach," cut everything except the core proof-of-concept: load model, generate text, print to console. HTTP API, KV cache, advanced sampling, monitoring - all deferred until AFTER validating Candle works. This is the fastest path to answer the question: "Should we use Candle or PyTorch?"

---

## Decision 5: Diagram Selection (Reduced)

**Date:** 2025-10-21 (Updated)
**Category:** Diagram Selection

**Context:** Original spec had 5 diagrams. For ultra-minimal MVP, evaluate if all are necessary.

**Options Considered:**
1. **Keep all 5 diagrams** (Component, Sequence, State, Deployment, Data Flow)
   - Pros: Comprehensive coverage
   - Cons: Potentially overkill for simple proof-of-concept
2. **Reduce to 2-3 essential diagrams**
   - Pros: Simpler spec, faster to read
   - Cons: May miss some perspectives
3. **Remove all diagrams**
   - Pros: Absolute simplicity
   - Cons: Violates rulebook minimum (3 diagrams)

**Selected:** Option 1 - Keep 5 diagrams (no change)

**Rationale:** Diagrams help understanding even for simple projects. The 5 diagrams provide balanced coverage and don't add significant complexity to implementation - they're just documentation. Keeping them maintains spec quality while implementation remains ultra-minimal.

---

## Decision 6: Testing Strategy (Simplified)

**Date:** 2025-10-21 (Updated)
**Category:** Process

**Context:** Define testing approach for ultra-minimal MVP.

**Options Considered:**
1. **Manual Testing Only**
   - Pros: Fastest, most flexible
   - Cons: Not repeatable
2. **Minimal Automated Tests**
   - Pros: Some automation, validation
   - Cons: Takes time to write
3. **Comprehensive Test Suite**
   - Pros: Full coverage
   - Cons: Too much for 3-5 day proof-of-concept

**Selected:** Option 1 - Manual Testing Only (for MVP)

**Rationale:** For 3-5 day proof-of-concept, manual verification is sufficient: "Does text appear? Is it coherent? Does nvidia-smi show GPU usage?" Automated tests can be added AFTER proving Candle works. Focus all time on getting it working, not on test infrastructure.

---

## Decision 7: Addendum Location

**Date:** 2025-10-21
**Category:** File Structure

**Context:** Determine whether to embed addendum in spec.md or create separate spec_addendum.md.

**Options Considered:**
1. **Embedded Addendum (default)**
   - Pros: Single file, simpler access, prevents documentation scatter
   - Cons: Makes spec.md longer
2. **Separate spec_addendum.md**
   - Pros: Cleaner spec.md, independent evolution
   - Cons: Two files to maintain

**Selected:** Option 1 - Embedded Addendum

**Rationale:** Per rulebook default: "By default, the developer-focused addendum must be embedded at the end of the specification file." For a simple proof-of-concept, single-file structure is preferred. All project documentation remains in one place.

---

## Decision 8: Deliverables Definition (Minimized)

**Date:** 2025-10-21 (Updated)
**Category:** Deliverables

**Context:** Define what will be delivered upon project completion.

**Options Considered:**
1. **Minimal: Binary + Source + Proof**
   - Deliverables: CLI binary, source code, screenshot/video of it working
   - Pros: Appropriate for proof-of-concept
   - Cons: No formal documentation
2. **Standard: Binary + Source + Docs**
   - Deliverables: Binary, source code, API docs, deployment guide
   - Pros: Production-ready package
   - Cons: Documentation overhead for 3-5 day MVP
3. **Full: Everything**
   - Deliverables: Binary, docs, training, dashboards
   - Cons: Massive overkill

**Selected:** Option 1 - Minimal

**Rationale:** For ultra-minimal proof-of-concept, deliverables should match the scope: working CLI binary, source code for review, and proof it works (screenshot/recording). No need for formal API documentation or deployment guides for a 3-5 day validation project.

---

## Decision 9: Timeline Reduction

**Date:** 2025-10-21 (Updated)
**Category:** Scope

**Context:** Original timeline was 2-3 weeks (3 milestones). User requested simpler, easier goals.

**Options Considered:**
1. **3-5 days (single milestone)**
   - Pros: Fast validation, minimal investment
   - Cons: Very limited scope
2. **2 weeks (CLI + HTTP)**
   - Pros: Usable service
   - Cons: Still complex for proof-of-concept
3. **3 weeks (original plan)**
   - Pros: Full feature set
   - Cons: Too long to validate core risk

**Selected:** Option 1 - 3-5 days

**Rationale:** User wants "simpler and easier to reach" MVP. The core question is: "Does Candle work for LLM inference?" This can be answered in 3-5 days with working code. Spending 3 weeks before knowing if the technology choice is viable is wasteful. Fast validation, then decide next steps.

---

## Summary of Simplifications

**Original Spec:**
- 3 Milestones (3 weeks)
- HTTP API + KV Cache + CLI
- 7 Functional Requirements
- 6 Non-Functional Requirements
- Complex deliverables

**Simplified Spec (Post-User Feedback):**
- 1 Milestone (3-5 days)
- CLI ONLY (no HTTP, no cache)
- 3 Functional Requirements
- 3 Non-Functional Requirements
- Minimal deliverables (binary + proof)

**Goal:** Prove Candle works. That's it.

---
