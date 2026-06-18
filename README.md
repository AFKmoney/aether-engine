# ⚡ Aether Engine — AetherOS (Project Hermes Ultra Ultimate)

> **Proprietary Rust inference engine and Unified Cognitive OS that gives a small GGUF model the suit of an Elite AI Super Agent.**

Turns a 1.2B–8B parameter model into an autonomous OS-managing agent via **10 interconnected middleware innovations**, an **Active OS Execution Kernel**, and **Multi-Persona Cognitive Synergy** (Arena Workspace Engineer + Claude Elite Architect + Hermes Core).

The AI builds its own house, writes its own tools, and runs its own recurrent thought loops.

---

## 👑 The Hermes Ultra "Super Agent Suit" (*L'habit fait le moine*)

Small models struggle with tokenization quantization and long-term logical drift. AetherOS outthinks how small models operate by giving them a **Triple-Layer Super Agent Suit**:

1. **Multi-Persona Cognitive Synergy (`AgentPersona`)**: Callers or the OS can seamlessly switch between three elite cognitive frameworks:
   - ⚡ `Hermes Ultimate OS Kernel` (Task routing, holographic context memory arbitration)
   - 🧠 `Claude Elite Architectural Synthesizer` (Deep reflection, high-level code planning, recursive problem decomposition)
   - 🛠️ `Arena Active Workspace Engineer` (Sandboxed Linux bash execution, incremental debugging, precise file crafting)
2. **Continuous Latent Trajectory (CLT) Recurrent Thought**: When the agent reasons, it runs an internal N-step recurrent trajectory in concept space, measuring TF-IDF cosine stability (`clt::check_convergence`) before ever collapsing to a discrete action.
3. **Asymmetric Tensor Dueling (ATD) Contestation**: Every candidate response or action is contested between an Exploitation Graph (Likelihood) and an Exploration/Safety Graph (Structural Entropy). Hallucinated commands or looping bigrams are instantly caught and forced through an active self-correction pass with scaled temperature (`atd::adjusted_temperature`).

## ⚙️ Core Architecture

```text
Natural Language Goal + Live Context
                │
                ▼
┌────────────────────────────────────────────────────────┐
│  MULTI-PERSONA AGENTIC KERNEL (agent.rs)               │
│  1. Perceive — Open Windows, Akasha Memory, Plan Tree  │
│  2. Priming  — Inject Persona Guidelines & Principles  │
│  3. Think    — CLT Latent Concept Trajectory Loop      │
│  4. Verify   — ATD Likelihood vs. Entropy Contestation │
│  5. Act      — Dynamic Tool & Custom Skill Parsing     │
│  6. Repeat   — Iterative execution until TASK_COMPLETE │
└────────────────────────────────────────────────────────┘
                │
                ▼
┌────────────────────────────────────────────────────────┐
│  13 ACTIVE OS-CONTROL TOOLS + DYNAMIC SKILLS (tools.rs)│
│  Real Filesystem IO (Sandboxed in /home/user)          │
│  Real PTY Shell Execution (std::process::Command)      │
│  Active In-Memory Window Manager & Plan Store          │
│  Dynamic Tool Auto-Creation (skill_register)           │
└────────────────────────────────────────────────────────┘
                │
                ▼
OpenAI-Compatible Backend (Ollama, llama.cpp, vLLM)
```

## 🛠️ The 13+ Active OS Tools

In AetherOS, tools are fully active and execute real side-effects inside the sandboxed workspace:

`file_read` · `file_write` · `file_list` · `file_delete` · `exec` · `window_open` · `window_close` · `memory_add` · `memory_search` · `web_search` · `plan_create` · `plan_update` · ⚡ **`skill_register`** (Dynamic Capability Auto-Creation)

## 🌟 The 10 Middleware Innovations

| # | Innovation | Description |
|---|-----------|-------------|
| 1 | **Semantic Memory Graph** | TF-IDF vectors, cosine edges, multi-hop graph retrieval |
| 2 | **Cognitive Decomposer** | Recursively breaks complex queries into topological sub-questions |
| 3 | **Self-Verification Loop** | Quality check + immediate self-correcting retry pass |
| 4 | **Knowledge Distillation** | Caches successful cognitive decomposition patterns for reuse |
| 5 | **Context Compressor** | Two-tier 40K → 4K token signal-preserving context compressor |
| 6 | **Action Cache** | Instant responses for repeated queries (Cosine similarity > 0.95) |
| 7 | **Speculative Prefetch** | Detached worker that warms adjacent memory graph caches |
| 8 | **HCM Arena** | FFT holographic state matrix (Infinite context in fixed 16KB) |
| 9 | **CLT Reasoning** | Recurrent reasoning loops in pure concept space |
| 10 | **ATD Contestation** | Dual-graph validation (Likelihood vs. Structural Entropy collision) |

## 🌐 The Masterpiece Web Desktop GUI (`GET /desktop`)

AetherOS features an unmatchable, self-contained HTML5/Canvas interactive OS Web Desktop interface served directly by the Rust engine:

- **Floating OS Windows**: Fully draggable, resizable application windows inside the browser.
- **Window 1: Advanced AI Autonomous Shell**: Interact with the OS in real-time, trigger multi-turn goals, select your desired Cognitive Persona, watch tool transcripts unfold live.
- **Window 2: Akasha Active Graph Visualizer**: A stunning interactive force-directed canvas visualizing semantic memory nodes, cosine edges, and Holographic Context Memory folds.
- **Window 3: Cognitive Dual-Reactor HUD**: Real-time visualization of ATD Likelihood-Entropy radar bars and CLT latent recurrence steps.
- **Window 4: Capability Studio**: Inspect the 13 active JSON schemas, invoke tools interactively, and watch auto-authored Dynamic Skills appear in real-time.

## 🚀 API Surface

| Method | Endpoint | Purpose |
|--------|----------|---------|
| `POST` | `/v1/agent/run` | Execute autonomous multi-persona agentic loop |
| `GET`  | `/desktop` | **HermesOS Unified Interactive Web Desktop Application** |
| `GET`  | `/os/state` | Retrieve live active OS Execution Kernel state |
| `GET`  | `/v1/skills` | Enumerate all dynamically registered runtime skills |
| `POST` | `/v1/skills/register` | Author and register custom dynamic skills (Bash/Python) |
| `GET`  | `/v1/tools` | Enumerate standard + dynamic tools with JSON Schema |
| `POST` | `/v1/chat/completions` | Run 10-stage cognitive inference pipeline |
| `POST` | `/v1/chat/stream` | Server-Sent Events (SSE) streaming inference |
| `GET`  | `/dashboard` | Monospace tactical ASCII liveness dashboard |
| `GET`  | `/health` | Telemetry HUD counters (Baseline + HCM + CLT + ATD) |
| `GET`  | `/metrics` | Prometheus Grafana exposition metrics |
| `GET`  | `/graph/export` | Downloadable semantic memory graph snapshot |

## 📦 Quick Start

```bash
# Compile optimized release kernel
cargo build --release

# Launch AetherOS Hermes Ultimate Kernel
AETHER_BACKEND=http://localhost:11434/v1 ./target/release/aether-engine

# Open Live OS Desktop in your browser
open http://localhost:3004/desktop
```

## 📜 License

MIT — © AFKmoney & Aether Community 2025–2026.
