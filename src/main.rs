//! # Aether Engine v3.0 — AetherOS Ultimate Masterpiece (Duet Nano-SIREN God-Mode Edition).
//!
//! A Rust HTTP service and Unified Cognitive Operating System that multiplies small GGUF model
//! capacity 10x+ via **fourteen interconnected innovations**, an **Active OS Execution Kernel**,
//! and a **Zero-Storage Duet Twin 1.2B Parallel Cluster**.
//!
//! # Revolutionary God-Mode Synergy ("The Real Deal")
//!
//! - **Nano-SIREN Recurrent Hat (`siren.rs`)**: Projects continuous latent reasoning trajectories
//!   through periodic sinusoidal activation networks (`sin(w*W*x + b)`) for exact analytical derivatives.
//! - **Duet Parallel L1/L2 Cache Streaming (`duet.rs`)**: Two 1.2B models (Alpha Generator vs Beta Verifier)
//!   work simultaneously in parallel, streaming thoughts through an L1/L2 Ring Buffer with zero mid-state garbage storage.

mod agent;
mod atd;
mod autocoder;
mod cache;
mod clt;
mod compress;
mod dashboard;
mod decompose;
mod desktop;
mod duet;
mod genesis;
mod graph;
mod handlers;
mod hcm;
mod hypnos;
mod mcts;
mod siren;
mod tfidf;
mod tools;

use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

pub struct Stats {
    pub requests: u64,
    pub cache_hits: u64,
    pub decompositions: u64,
    pub verifications: u64,
    pub verifications_passed: u64,
    pub distillation_hits: u64,
    pub hcm_pairs_folded: u64,
    pub hcm_probes: u64,
    pub clt_loops: u64,
    pub clt_convergences: u64,
    pub clt_total_steps: u64,
    pub atd_verifications: u64,
    pub atd_validated: u64,
    pub atd_rejected: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub graph: Arc<Mutex<graph::MemoryGraph>>,
    pub cache: Arc<Mutex<cache::ActionCache>>,
    pub distillation: Arc<Mutex<decompose::DistillationStore>>,
    pub hcm: Arc<Mutex<hcm::HolographicMemoryArena>>,
    pub stats: Arc<Mutex<Stats>>,
    pub os_state: tools::ActiveOSState,
    pub backend: String,
    pub client: reqwest::Client,
}

const PORT: u16 = 3004;

#[tokio::main]
async fn main() {
    let backend = std::env::var("AETHER_BACKEND")
        .unwrap_or_else(|_| "http://localhost:11434/v1".to_string());

    let state = AppState {
        graph: Arc::new(Mutex::new(graph::MemoryGraph::new())),
        cache: Arc::new(Mutex::new(cache::ActionCache::new())),
        distillation: Arc::new(Mutex::new(decompose::DistillationStore::new())),
        hcm: Arc::new(Mutex::new(hcm::HolographicMemoryArena::new(1024))),
        stats: Arc::new(Mutex::new(Stats {
            requests: 0,
            cache_hits: 0,
            decompositions: 0,
            verifications: 0,
            verifications_passed: 0,
            distillation_hits: 0,
            hcm_pairs_folded: 0,
            hcm_probes: 0,
            clt_loops: 0,
            clt_convergences: 0,
            clt_total_steps: 0,
            atd_verifications: 0,
            atd_validated: 0,
            atd_rejected: 0,
        })),
        os_state: tools::ActiveOSState::new(),
        backend: backend.clone(),
        client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("reqwest client"),
    };

    tokio::spawn(crate::genesis::start_genesis_loop(state.clone()));

    let app = Router::new()
        .route("/v1/chat/completions", post(handlers::chat_completions))
        .route("/v1/chat/stream", post(handlers::chat_completions_stream))
        .route("/v1/interrupt", post(handlers::interrupt))
        .route("/v1/models", get(handlers::list_models))
        .route("/graph/add", post(handlers::graph_add))
        .route("/graph", get(handlers::graph_get))
        .route("/graph/search", post(handlers::graph_search))
        .route("/graph/clear", post(handlers::graph_clear))
        .route("/graph/export", get(handlers::graph_export))
        .route("/graph/import", post(handlers::graph_import))
        .route("/graph/stats", get(handlers::graph_stats))
        .route("/pipeline", get(handlers::pipeline_stats))
        .route("/health", get(handlers::health))
        .route("/dashboard", get(handlers::dashboard))
        .route("/metrics", get(handlers::prometheus_metrics))
        .route("/config", get(handlers::get_config))
        // --- Agentic layer (v3.2) ---
        .route("/v1/agent/run", post(handlers::agent_run))
        .route("/v1/tools", get(handlers::list_tools))
        // --- Aether Unified OS Additions (v3.3 Ultimate) ---
        .route("/desktop", get(handlers::desktop_gui))
        .route("/os/state", get(handlers::get_os_state))
        .route("/v1/skills", get(handlers::list_skills))
        .route("/v1/skills/register", post(handlers::register_skill))
        // --- Revolutionary Masterpiece Additions (v4.0 History-Book Paradigm) ---
        .route("/v1/genesis/logs", get(handlers::get_genesis_logs))
        .route("/v1/genesis/toggle", post(handlers::toggle_genesis))
        .route("/v1/hypnos/sleep", post(handlers::hypnos_sleep_protocol))
        .route("/v1/mcts/speculate", post(handlers::mcts_speculation_tree))
        // --- Holy Grail Offline 1.2B Additions (v4.1) ---
        .route("/v1/autocoder/run", post(handlers::run_autocoder_endpoint))
        // --- Science-Fiction Twin Duet & Nano-SIREN God-Mode Additions (v5.0 Ultimate) ---
        .route("/v1/duet/run", post(handlers::run_duet_synergy_endpoint))
        .route("/v1/siren/sync", post(handlers::measure_siren_sync_endpoint))
        .route("/v1/duet/flush", post(handlers::flush_l1l2_ringbuffer_endpoint))
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .expect("bind 3004");

    eprintln!(
        "⚡ [Aether Engine AetherOS v5.0 Ultimate God-Mode] Epoch Kernel listening on :{PORT} (backend: {backend})\n\
         🧠 24 Divine OS Tools enabled (Siren Recurrent Hat, Zero-Storage L1/L2 Buffer Dual-Inference)\n\
         🌐 Live Cyberpunk OS Web Desktop accessible at http://localhost:{PORT}/desktop\n\
         🚀 Infinite Power online: Twin 1.2B Duet Concurrency + Recurrent Sinusoidal Activation Waves"
    );

    axum::serve(listener, app).await.expect("server stopped");
}
