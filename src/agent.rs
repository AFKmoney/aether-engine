//! # Agent — The autonomous OS-management agentic loop (Aether Ultimate Super-Agent Edition).
//!
//! This module is the **brain** of the autonomous OS layer. It implements
//! the classical perceive → think → act → observe control loop on top of
//! the Aether Engine's existing inference pipeline, turning a small GGUF
//! model into an autonomous agent capable of managing a Linux-like OS.
//!
//! # Ultimate Super-Agent Upgrades ("L'habit fait le moine")
//!
//! 1. **Multi-Persona Cognitive Synergy (`AgentPersona`)**: Callers can choose
//!    between `AetherUnifiedOS`, `ClaudeEliteArchitect`, and `ArenaActiveWorkspace`.
//! 2. **Continuous Latent Trajectory (CLT) Recurrent Thought**: In the `think` step,
//!    the agent runs an internal recurrent trajectory in concept space, checking
//!    TF-IDF cosine convergence (`clt::check_convergence`) before collapsing to an action.
//! 3. **Asymmetric Tensor Dueling (ATD) Contestation**: Every candidate response
//!    is contested between Likelihood and Entropy (`atd::verify`). High structural
//!    entropy or bigram loops trigger automatic prompt re-framing and temperature adjustments!

use crate::atd::{self, ATDConfig, ATDRecommendation};
use crate::clt::{self, CLTConfig, CLTState};
use crate::tfidf::TfidfVectorizer;
use crate::tools::{Tool, ToolRegistry, ToolSpec};
use crate::AppState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// Persona Engine for Multi-Model Synergy
// ---------------------------------------------------------------------------

/// Specialized Cognitive Personas within Aether AetherOS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPersona {
    /// The Unified OS Core: orchestrates tasks, manages holographic context memory.
    AetherUnifiedOS,
    /// The Claude Synthesizer: elite deep reflection, recursive problem decomposition, architectural code planning.
    ClaudeEliteArchitect,
    /// The Arena Workspace Agent: sandboxed filesystem manipulation, precise Bash execution, interactive debugging.
    ArenaActiveWorkspace,
}

impl AgentPersona {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "claude" | "architect" | "claudeelitearchitect" => Self::ClaudeEliteArchitect,
            "arena" | "workspace" | "arenaactiveworkspace" => Self::ArenaActiveWorkspace,
            _ => Self::AetherUnifiedOS,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::AetherUnifiedOS => "Aether Ultimate OS Kernel",
            Self::ClaudeEliteArchitect => "Claude Elite Architectural Synthesizer",
            Self::ArenaActiveWorkspace => "Arena Active Sandboxed Workspace Engineer",
        }
    }

    pub fn operating_principles(&self) -> &str {
        match self {
            Self::AetherUnifiedOS => {
                "- Maintain unified OS state: track open windows, balance holographic context memory.\n\
                 - Act deliberately and autonomously: issue precise tool calls to achieve the user's goal.\n\
                 - Manage permanent 24/7 self-evolution autopoiesis via `genesis_toggle`.\n\
                 - Trigger MCTS latent thought speculation via `mcts_speculate` for complex paradigms.\n\
                 - Consolidate fragmented daily experiential memories via `hypnos_sleep`.\n\
                 - If a required capability is missing, use `skill_register` to dynamically write and register it!"
            }
            Self::ClaudeEliteArchitect => {
                "- Think first: engage in deep semantic reflection, MCTS tree rollout, and architectural synthesis.\n\
                 - Recursively break complex systems down into clean, manageable sub-components (`plan_create`).\n\
                 - Use `code_analyze` to inspect structural complexity and AST patterns of source code.\n\
                 - When authoring code or schemas, ensure elegant structure, robust error handling, and flawless design.\n\
                 - Verify prior knowledge before executing destructive mutations."
            }
            Self::ArenaActiveWorkspace => {
                "- Master the sandboxed workspace (`/home/user`): use `git_orchestrate` for complete autonomous Git management.\n\
                 - Execute Linux tools and shell scripts via `exec` and evaluate pure python/bash code via `sandbox_eval`.\n\
                 - Build incrementally: inspect files with `file_read`, make precise edits with `file_write`.\n\
                 - Debug interactively: read command exit codes and stderr, self-correct immediately upon failure.\n\
                 - Use `net_probe` to verify external HTTP/REST dependencies and model health."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Core data types
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AgentMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
            name: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
            name: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
            name: None,
        }
    }

    pub fn tool(tool_name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".to_string(),
            content: content.into(),
            name: Some(tool_name.into()),
        }
    }

    pub fn to_request_json(&self) -> Value {
        match self.role.as_str() {
            "tool" => json!({
                "role": "user",
                "content": format!(
                    "[tool_result: {}] {}",
                    self.name.as_deref().unwrap_or("unknown"),
                    self.content
                ),
            }),
            other => json!({
                "role": other,
                "content": self.content,
            }),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub params: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool: String,
    pub output: String,
    pub success: bool,
}

#[derive(Clone, Debug)]
pub struct AgentResponse {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub atd_validated: bool,
    pub clt_steps: usize,
}

#[derive(Clone, Debug)]
pub struct AgentState {
    pub iteration: usize,
    pub messages: Vec<AgentMessage>,
    pub tool_results: Vec<ToolResult>,
    pub completed: bool,
    pub persona: AgentPersona,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRunResult {
    pub result: String,
    pub iterations: usize,
    pub tool_calls: Vec<ToolCall>,
    pub completed: bool,
    pub persona_used: String,
}

pub const TASK_COMPLETE_SENTINEL: &str = "TASK_COMPLETE";
pub const DEFAULT_MAX_ITERATIONS: usize = 20;
pub const ABSOLUTE_MAX_ITERATIONS: usize = 50;

// ---------------------------------------------------------------------------
// System prompt construction
// ---------------------------------------------------------------------------

pub fn build_agent_system_prompt(
    tool_catalog: &[ToolSpec],
    context: &Value,
    persona: AgentPersona,
) -> String {
    let mut tool_blocks = String::with_capacity(8 * 1024);
    for spec in tool_catalog {
        tool_blocks.push_str(&format!(
            "### {name}\n{desc}\n\nParameters (JSON Schema):\n```json\n{schema}\n```\n\n",
            name = spec.name,
            desc = spec.description,
            schema = serde_json::to_string_pretty(&spec.parameters)
                .unwrap_or_else(|_| "{}".to_string()),
        ));
    }

    let context_json = if context.is_object() && !context.is_array() {
        serde_json::to_string_pretty(context).unwrap_or_else(|_| "{}".to_string())
    } else {
        serde_json::to_string_pretty(&json!({ "context": context }))
            .unwrap_or_else(|_| "{}".to_string())
    };

    format!(
        "# AETHER AUTONOMOUS AGENT — {title}\n\
         You are the autonomous agent cognitive core of Aether AetherOS. \
         You perceive the live OS context, reason deeply in concept space, and act through tools. \
         Your mission is to achieve the user's goal by issuing tool calls, observing results, and iterating.\n\n\
         ## Goal Protocol\n\
         To call a tool, emit a JSON object anywhere in your response. The object MUST have a \
         `tool` (or `name`) field set to one of the available tool names, and a `params` (or \
         `arguments`) field containing the parameters. You may emit multiple tool calls; they will \
         be executed in order and fed back to you as `[tool_result: <tool>]` observations.\n\n\
         Example:\n\
         ```json\n\
         {{\n\
           \"tool\": \"file_read\",\n\
           \"params\": {{ \"path\": \"/etc/hostname\" }}\n\
         }}\n\
         ```\n\n\
         When — and only when — you have fully achieved the user's goal, emit the literal sentinel \
         `{sentinel}` on its own line.\n\n\
         ## Available Tools\n\
         {tool_blocks}\n\
         ## Current OS State Snapshot\n\
         ```json\n\
         {context_json}\n\
         ```\n\n\
         ## Cognitive Operating Principles\n\
         {principles}\n\
         - Verify before trusting: use `memory_search` or `file_read` before destructive actions.\n\
         - Decompose complexity: create multi-step plans (`plan_create`) for complex tasks.\n\
         - Self-correct: if a tool output reveals an error, adapt and retry instantly.",
        title = persona.title(),
        sentinel = TASK_COMPLETE_SENTINEL,
        tool_blocks = tool_blocks,
        context_json = context_json,
        principles = persona.operating_principles(),
    )
}

// ---------------------------------------------------------------------------
// Tool-call parsing
// ---------------------------------------------------------------------------

pub fn parse_tool_calls(response: &str) -> Vec<ToolCall> {
    let mut calls = Vec::new();
    let bytes = response.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            if let Some((json_str, end)) = extract_balanced_json(response, i) {
                if let Ok(v) = serde_json::from_str::<Value>(&json_str) {
                    calls.extend(value_to_tool_calls(&v));
                }
                i = end;
                continue;
            }
        }
        i += 1;
    }
    calls
}

fn extract_balanced_json(s: &str, start: usize) -> Option<(String, usize)> {
    let bytes = s.as_bytes();
    if start >= bytes.len() || bytes[start] != b'{' {
        return None;
    }
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escape = false;
    for i in start..bytes.len() {
        let c = bytes[i];
        if in_string {
            if escape {
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c == b'"' {
                in_string = false;
            }
        } else {
            match c {
                b'"' => in_string = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((s[start..=i].to_string(), i + 1));
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn value_to_tool_calls(v: &Value) -> Vec<ToolCall> {
    if let Some(arr) = v.as_array() {
        return arr.iter().filter_map(value_to_single_call).collect();
    }
    for wrapper_key in ["tool_calls", "calls"] {
        if let Some(inner) = v.get(wrapper_key).and_then(|x| x.as_array()) {
            return inner.iter().filter_map(value_to_single_call).collect();
        }
    }
    if let Some(c) = value_to_single_call(v) {
        return vec![c];
    }
    Vec::new()
}

fn value_to_single_call(v: &Value) -> Option<ToolCall> {
    let name = v
        .get("tool")
        .and_then(|x| x.as_str())
        .or_else(|| v.get("name").and_then(|x| x.as_str()))
        .or_else(|| v.get("tool_name").and_then(|x| x.as_str()));

    if let Some(name) = name {
        let params = v
            .get("params")
            .cloned()
            .or_else(|| v.get("arguments").cloned())
            .or_else(|| v.get("args").cloned())
            .filter(Value::is_object)
            .unwrap_or_else(|| json!({}));
        return Some(ToolCall {
            name: name.to_string(),
            params,
        });
    }

    if let Some(func) = v.get("function") {
        if let Some(fname) = func.get("name").and_then(|x| x.as_str()) {
            let params = match func.get("arguments") {
                Some(Value::String(s)) => {
                    serde_json::from_str(s).unwrap_or_else(|_| json!({}))
                }
                Some(other) if other.is_object() => other.clone(),
                _ => json!({}),
            };
            return Some(ToolCall {
                name: fname.to_string(),
                params,
            });
        }
    }

    None
}

// ---------------------------------------------------------------------------
// CLT Recurrent Thought & ATD Contestation Calling Engine
// ---------------------------------------------------------------------------

async fn call_llm_with_super_agent_suit(
    state: &AppState,
    messages: &[AgentMessage],
    query: &str,
    vectorizer: &TfidfVectorizer,
) -> AgentResponse {
    let clt_config = CLTConfig::default();
    let atd_config = ATDConfig::default();

    let mut curr_messages = messages.to_vec();
    let mut last_output = String::new();
    let mut steps_executed = 0;
    let mut prev_output: Option<String> = None;

    // ---- Innovation #9: Continuous Latent Trajectory (CLT) Recurrent Loop ----
    for step in 0..clt_config.max_steps {
        steps_executed += 1;

        // If we are in refinement steps, inject the CLT recurrent prompt
        if let Some(ref prev) = prev_output {
            let recurrent_prompt = clt::build_iteration_prompt(query, Some(prev), step);
            if let Some(last) = curr_messages.last_mut() {
                last.content = recurrent_prompt;
            }
        }

        let raw_resp = call_raw_llm(state, &curr_messages, 0.7).await;
        last_output = raw_resp.clone();

        // Check if explicit completion or tool calls appear — if so, don't over-iterate
        if raw_resp.contains(TASK_COMPLETE_SENTINEL) || !parse_tool_calls(&raw_resp).is_empty() {
            break;
        }

        if let Some(ref prev) = prev_output {
            let sim = clt::check_convergence(prev, &raw_resp, vectorizer);
            if sim >= clt_config.convergence_threshold && step >= clt_config.min_steps {
                // Latent trajectory converged!
                state.stats.lock().await.clt_convergences += 1;
                break;
            }
        }

        prev_output = Some(raw_resp);
        state.stats.lock().await.clt_loops += 1;
        state.stats.lock().await.clt_total_steps += 1;
    }

    // ---- Innovation #10: Asymmetric Tensor Dueling (ATD) Contestation ----
    state.stats.lock().await.atd_verifications += 1;
    let atd_verdict = atd::verify(&last_output, query, &atd_config);

    let final_text = if atd_verdict.validated {
        state.stats.lock().await.atd_validated += 1;
        last_output
    } else {
        state.stats.lock().await.atd_rejected += 1;
        
        // Contestation failed (high entropy or looping) → self-correct with adjusted temperature
        let new_temp = atd::adjusted_temperature(0.7, &atd_verdict, &atd_config);
        
        let retry_prompt = match atd_verdict.recommendation {
            ATDRecommendation::RetryWithRephrasedPrompt => {
                format!("Your previous thought was looping. Please re-evaluate `{query}` and take a fresh approach.")
            }
            ATDRecommendation::FallBackToSimpleShot => {
                format!("Provide a highly precise, immediate action or tool call to solve `{query}`.")
            }
            _ => {
                format!("Your previous output had high structural entropy. Please provide a clear, focused tool call or completion signal.")
            }
        };

        curr_messages.push(AgentMessage::system(retry_prompt));
        let retry_resp = call_raw_llm(state, &curr_messages, new_temp).await;
        
        // Verify retry
        let retry_verdict = atd::verify(&retry_resp, query, &atd_config);
        if retry_verdict.validated {
            state.stats.lock().await.atd_validated += 1;
            retry_resp
        } else {
            // Pick less bad response
            if retry_verdict.collision_delta > atd_verdict.collision_delta {
                retry_resp
            } else {
                last_output
            }
        }
    };

    let tool_calls = parse_tool_calls(&final_text);

    AgentResponse {
        text: final_text,
        tool_calls,
        atd_validated: atd_verdict.validated,
        clt_steps: steps_executed,
    }
}

async fn call_raw_llm(state: &AppState, messages: &[AgentMessage], temperature: f64) -> String {
    let request_body = json!({
        "messages": messages.iter().map(AgentMessage::to_request_json).collect::<Vec<_>>(),
        "temperature": temperature,
    });
    let url = format!(
        "{}/chat/completions",
        state.backend.trim_end_matches('/')
    );

    match state.client.post(&url).json(&request_body).send().await {
        Ok(resp) if resp.status().is_success() => {
            let txt = resp.text().await.unwrap_or_default();
            extract_assistant_content(&txt).unwrap_or_else(|| {
                format!(
                    "[aether-agent] empty backend response: {}",
                    txt.chars().take(200).collect::<String>()
                )
            })
        }
        Ok(resp) => {
            let status = resp.status();
            let body_txt = resp.text().await.unwrap_or_default();
            format!(
                "[aether-agent] backend returned {status}: {}",
                body_txt.chars().take(200).collect::<String>()
            )
        }
        Err(e) => format!("[aether-agent] backend unreachable: {e}"),
    }
}

fn extract_assistant_content(body: &str) -> Option<String> {
    let v: Value = serde_json::from_str(body).ok()?;
    v.get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
}

// ---------------------------------------------------------------------------
// The main autonomous agent loop
// ---------------------------------------------------------------------------

pub async fn run_agent_loop(
    state: AppState,
    goal: &str,
    context: &Value,
    max_iterations: usize,
) -> AgentRunResult {
    let cap = max_iterations.clamp(1, ABSOLUTE_MAX_ITERATIONS);

    // Parse persona from request context if provided, else default to AetherUnifiedOS
    let persona = context
        .get("persona")
        .and_then(|p| p.as_str())
        .map(AgentPersona::from_str)
        .unwrap_or(AgentPersona::AetherUnifiedOS);

    let registry = ToolRegistry::new();
    let catalog = registry.catalog();
    let system_prompt = build_agent_system_prompt(&catalog, context, persona);

    let mut agent_state = AgentState {
        iteration: 0,
        messages: vec![
            AgentMessage::system(system_prompt),
            AgentMessage::user(goal.to_string()),
        ],
        tool_results: Vec::new(),
        completed: false,
        persona,
    };

    let mut all_tool_calls: Vec<ToolCall> = Vec::new();
    let mut final_text = String::new();

    // Setup vectorizer for CLT
    let vectorizer = {
        let g = state.graph.lock().await;
        g.vectorizer.clone()
    };

    for i in 0..cap {
        agent_state.iteration = i + 1;

        // ---- 2. Think: call LLM with CLT and ATD Super Agent Suit ----
        let resp = call_llm_with_super_agent_suit(&state, &agent_state.messages, goal, &vectorizer).await;
        let response_text = resp.text.clone();

        if response_text.contains(TASK_COMPLETE_SENTINEL) {
            final_text = response_text;
            agent_state.completed = true;
            break;
        }

        let calls = resp.tool_calls.clone();
        all_tool_calls.extend(calls.clone());

        if calls.is_empty() {
            final_text = response_text;
            break;
        }

        agent_state.messages.push(AgentMessage::assistant(response_text.clone()));

        // ---- 4. Execute: dispatch each tool call through Active OS Registry ----
        for call in &calls {
            let (output, success) = match Tool::from_request(&call.name, &call.params) {
                Some(tool) => registry.execute(&tool).await,
                None => (
                    format!(
                        "tool_not_found: no tool named \"{}\" in registry. \
                         Available tools: {}",
                        call.name,
                        catalog
                            .iter()
                            .map(|t| t.name.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    false,
                ),
            };

            let result = ToolResult {
                tool: call.name.clone(),
                output: output.clone(),
                success,
            };
            agent_state.tool_results.push(result);

            // ---- 5. Observe: feed tool result back into stream ----
            agent_state
                .messages
                .push(AgentMessage::tool(call.name.clone(), output));
        }

        final_text = response_text;
    }

    AgentRunResult {
        result: final_text,
        iterations: agent_state.iteration,
        tool_calls: all_tool_calls,
        completed: agent_state.completed,
        persona_used: format!("{:?}", persona),
    }
}
