# ⚡ Aether Engine

> **Proprietary Rust inference engine that makes a small GGUF model into an autonomous agent.**

Turns a 1.2B model into an autonomous OS-managing agent via 10 middleware innovations + an agentic loop with 12 OS-control tools. The AI builds its own house.

---

## Architecture

```
Goal + Context
    │
    ▼
┌─────────────────────────────────────────────┐
│  AGENT LOOP (agent.rs)                      │
│  1. Perceive  — OS state in system prompt   │
│  2. Think     — call backend LLM            │
│  3. Act       — parse tool calls            │
│  4. Execute   — ToolRegistry dispatch       │
│  5. Observe   — feed results back           │
│  6. Repeat    — until TASK_COMPLETE         │
└─────────────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────────────┐
│  10-STAGE INFERENCE PIPELINE (handlers.rs)  │
│  Action Cache → Graph Retrieve → Compress   │
│  → Decompose → Solve → Synthesize → ATD     │
│  → Distill → Prefetch                       │
└─────────────────────────────────────────────┘
    │
    ▼
OpenAI-compatible backend (Ollama, llama.cpp, vLLM)
```

## The 12 agent tools

`file_read` · `file_write` · `file_list` · `file_delete` · `exec` · `window_open` · `window_close` · `memory_add` · `memory_search` · `web_search` · `plan_create` · `plan_update`

## The 10 innovations

| # | Innovation | Role |
|---|-----------|------|
| 1 | Semantic Memory Graph | TF-IDF vectors, cosine edges, graph retrieval |
| 2 | Cognitive Decomposer | Break complex → sub-questions |
| 3 | Self-Verification | Quality check + retry |
| 4 | Knowledge Distillation | Store winning patterns |
| 5 | Context Compressor | 40K → 4K tokens |
| 6 | Action Cache | Cosine > 0.95 → instant |
| 7 | Speculative Prefetch | Warm adjacent caches |
| 8 | HCM | FFT holographic memory, fixed 16KB |
| 9 | CLT | N-step latent trajectory, cosine convergence |
| 10 | ATD | Likelihood vs entropy verification |

## API

| Method | Endpoint | Purpose |
|--------|----------|---------|
| `POST` | `/v1/agent/run` | Run the agentic loop (goal + context → result + tool_calls) |
| `GET`  | `/v1/tools` | List available tools + schemas |
| `POST` | `/v1/chat/completions` | OpenAI-compatible inference |
| `POST` | `/v1/chat/stream` | SSE streaming |
| `GET`  | `/dashboard` | HTML status dashboard |
| `GET`  | `/metrics` | Prometheus metrics |
| `GET`  | `/graph/export` | Graph JSON |
| `GET`  | `/config` | Runtime config |
| `GET`  | `/v1/models` | OpenAI-compatible model list |

## Quick start

```bash
cargo build --release
AETHER_BACKEND=http://localhost:11434/v1 ./target/release/aether-engine
```

## Project layout

```
src/
├── main.rs        HTTP server + routes
├── handlers.rs    Endpoint handlers + 10-stage pipeline
├── agent.rs       Agentic loop (perceive→think→act→observe)
├── tools.rs       12-tool registry for OS control
├── graph.rs       Semantic memory graph (#1)
├── tfidf.rs       TF-IDF vectorizer
├── cache.rs       Action cache (#6)
├── compress.rs    Context compressor (#5)
├── decompose.rs   Cognitive decomposer + distillation (#2, #4)
├── hcm.rs         Holographic Context Memory — FFT (#8)
├── clt.rs         Continuous Latent Trajectory (#9)
├── atd.rs         Asymmetric Tensor Dueling (#10)
└── dashboard.rs   HTML dashboard
```

## License

MIT — © AFKmoney 2025.
