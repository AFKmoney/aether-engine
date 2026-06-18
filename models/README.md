# 📁 Aether Embedded GGUF Model Repository (`./models`)

Drop your raw `.gguf` offline edge files (e.g., `qwen2.5-coder-1.5b.gguf`, `starcoder2-3b.gguf`, `deepseek-coder-1.3b.gguf`) directly into this directory.

Aether Engine's internal **GGUF Native Auto-Loader (`Innovation #15`)** actively scans this directory at startup and request runtime, automatically exposing your local models to the execution pipeline and the `GET /v1/models` REST API.

---

## Supported GGUF Targets

- Code Specialized: `Qwen2.5-Coder`, `StarCoder2`, `DeepSeek-Coder`
- Multi-Modal Edge: `LLaMA-3.2-1B`, `Vicuna-3B`
- Fast Probers: `TinyLlama-1.1B`, `SmolLM-135M`
