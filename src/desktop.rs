//! # HermesOS v4.0 Masterpiece Unified Web Desktop Application (`GET /desktop`)
//!
//! An absolute disruptive, epoch-defining Science-Fiction GUI served directly by Aether Engine.
//! Features floating window management, real-time multi-persona shell interaction,
//! Akasha semantic memory graph visualization, Triple-Reactor (ATD/CLT/MCTS) cognitive matrix,
//! 24/7 Genesis Autopoiesis HUD, and Hypnos Neural Memory Consolidation protocol studio.

pub fn render_desktop_gui() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>⚡ Aether HermesOS v4.0 — Disruptive Cognitive Operating System Masterpiece</title>
  <style>
    :root {
      --bg-desktop: #07090e;
      --bg-win-header: #121522;
      --bg-win-body: #0d0f18;
      --border-win: #22273b;
      --accent-purple: #a78bfa;
      --accent-cyan: #22d3ee;
      --accent-green: #34d399;
      --accent-pink: #f472b6;
      --accent-gold: #f59e0b;
      --text-main: #f3f4f6;
      --text-dim: #9ca3af;
      --font-mono: 'SF Mono', 'JetBrains Mono', 'Fira Code', Consolas, monospace;
    }

    * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
    body {
      background: var(--bg-desktop);
      color: var(--text-main);
      font-family: var(--font-mono);
      overflow: hidden;
      height: 100vh;
      display: flex;
      flex-direction: column;
    }

    /* Top Navigation HUD */
    .nav-hud {
      height: 42px;
      background: rgba(13, 15, 24, 0.9);
      backdrop-filter: blur(10px);
      border-bottom: 1px solid var(--border-win);
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 20px;
      z-index: 10000;
      font-size: 12px;
      letter-spacing: 0.05em;
      box-shadow: 0 4px 20px rgba(0,0,0,0.5);
    }

    .nav-hud .brand { display: flex; align-items: center; gap: 8px; font-weight: 800; color: var(--accent-cyan); text-transform: uppercase; font-size: 13px; }
    .nav-hud .brand span.ultra { color: var(--accent-purple); text-shadow: 0 0 10px rgba(167,139,250,0.5); }
    .nav-hud .brand span.ver { color: var(--accent-gold); font-size: 10px; background: rgba(245,158,11,0.1); padding: 2px 5px; border-radius: 4px; border: 1px solid var(--accent-gold); }
    
    .nav-hud .telemetry-pills { display: flex; align-items: center; gap: 14px; }
    .nav-hud .pill { display: flex; align-items: center; gap: 6px; background: rgba(255,255,255,0.02); padding: 4px 10px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.08); font-size: 11px; }
    .nav-hud .pill span.val { color: var(--accent-green); font-weight: 700; }
    .nav-hud .pill span.val.gold { color: var(--accent-gold); }

    .nav-hud .tools-hud { display: flex; align-items: center; gap: 10px; }
    .nav-hud select { background: #1a1e2f; color: var(--accent-cyan); border: 1px solid var(--border-win); padding: 4px 10px; border-radius: 6px; font-family: var(--font-mono); font-size: 11px; font-weight: bold; outline: none; cursor: pointer; }

    /* Desktop Area */
    .desktop-area {
      flex: 1;
      position: relative;
      overflow: hidden;
      background: radial-gradient(circle at 50% 50%, rgba(167, 139, 250, 0.03) 0%, rgba(7, 9, 14, 1) 100%);
    }

    /* Quantum Cyberpunk Grid */
    .desktop-grid {
      position: absolute;
      top: 0; left: 0; width: 100%; height: 100%;
      background-size: 50px 50px;
      background-image: linear-gradient(to right, rgba(255, 255, 255, 0.015) 1px, transparent 1px),
                        linear-gradient(to bottom, rgba(255, 255, 255, 0.015) 1px, transparent 1px);
      z-index: 1;
    }

    /* OS Floating Windows */
    .os-win {
      position: absolute;
      background: rgba(13, 15, 24, 0.85);
      backdrop-filter: blur(16px);
      border: 1px solid var(--border-win);
      border-radius: 12px;
      display: flex;
      flex-direction: column;
      box-shadow: 0 16px 40px rgba(0,0,0,0.7), 0 0 0 1px rgba(255,255,255,0.05);
      z-index: 10;
      overflow: hidden;
      min-width: 320px;
      min-height: 220px;
      transition: border-color 0.2s, box-shadow 0.2s;
    }

    .os-win.active { z-index: 100; border-color: var(--accent-cyan); box-shadow: 0 20px 60px rgba(0,0,0,0.9), 0 0 0 1px var(--accent-cyan); }

    .win-header {
      height: 38px;
      background: var(--bg-win-header);
      border-bottom: 1px solid var(--border-win);
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 14px;
      cursor: grab;
      font-size: 12px;
      color: var(--text-dim);
    }

    .win-header:active { cursor: grabbing; }
    .win-header .title { display: flex; align-items: center; gap: 8px; font-weight: 700; color: var(--text-main); letter-spacing: 0.03em; }
    .win-header .title-dot { width: 8px; height: 8px; border-radius: 50%; box-shadow: 0 0 8px currentColor; }
    
    .win-controls { display: flex; align-items: center; gap: 8px; }
    .win-btn { width: 13px; height: 13px; border-radius: 50%; border: none; cursor: pointer; transition: transform 0.1s; }
    .win-btn:hover { transform: scale(1.2); }
    .win-btn.close { background: #ef4444; box-shadow: 0 0 6px #ef4444; }
    .win-btn.min { background: #f59e0b; box-shadow: 0 0 6px #f59e0b; }
    .win-btn.max { background: #10b981; box-shadow: 0 0 6px #10b981; }

    .win-body {
      flex: 1;
      display: flex;
      flex-direction: column;
      position: relative;
      overflow: auto;
      user-select: text;
    }

    /* Terminal Window */
    .term-container { display: flex; flex-direction: column; height: 100%; padding: 14px; gap: 12px; font-size: 12px; }
    .term-output { flex: 1; overflow-y: auto; background: rgba(5,6,10,0.9); border-radius: 8px; padding: 14px; display: flex; flex-direction: column; gap: 10px; border: 1px solid rgba(255,255,255,0.06); font-family: var(--font-mono); }
    .term-line { display: flex; gap: 10px; line-height: 1.5; }
    .term-line .prompt { color: var(--accent-pink); font-weight: 800; text-transform: uppercase; }
    .term-line .user-txt { color: var(--text-main); font-weight: 600; }
    .term-line .ai-txt { color: var(--accent-cyan); white-space: pre-wrap; }
    .term-line .tool-txt { color: var(--accent-green); background: rgba(52, 211, 153, 0.08); padding: 6px 10px; border-radius: 6px; border-left: 3px solid var(--accent-green); white-space: pre-wrap; width: 100%; font-size: 11px; }

    .term-input-box { display: flex; gap: 10px; }
    .term-input-box input { flex: 1; background: #141724; border: 1px solid var(--border-win); color: var(--text-main); padding: 12px 16px; border-radius: 8px; font-family: var(--font-mono); font-size: 12px; font-weight: 600; outline: none; box-shadow: inset 0 2px 6px rgba(0,0,0,0.5); }
    .term-input-box input:focus { border-color: var(--accent-purple); box-shadow: 0 0 12px rgba(167,139,250,0.3); }
    .term-input-box button { background: linear-gradient(135deg, var(--accent-purple), var(--accent-pink)); color: #000; border: none; padding: 0 22px; border-radius: 8px; font-family: var(--font-mono); font-weight: 800; font-size: 12px; letter-spacing: 0.05em; cursor: pointer; box-shadow: 0 4px 14px rgba(167,139,250,0.4); transition: all 0.2s; }
    .term-input-box button:hover { transform: translateY(-1px); box-shadow: 0 6px 20px rgba(167,139,250,0.6); }

    /* Graph Visualizer Window */
    .graph-canvas { width: 100%; height: 100%; display: block; background: #05060a; }
    .graph-hud { position: absolute; top: 14px; left: 14px; pointer-events: none; background: rgba(13,15,24,0.85); backdrop-filter: blur(8px); padding: 10px 14px; border-radius: 8px; border: 1px solid var(--border-win); font-size: 11px; display: flex; flex-direction: column; gap: 4px; box-shadow: 0 4px 16px rgba(0,0,0,0.6); }
    .graph-hud .g-title { font-weight: 800; color: var(--accent-cyan); text-transform: uppercase; }

    /* Triple Reactor MCTS/ATD/CLT HUD */
    .reactor-container { display: flex; flex-direction: column; gap: 16px; padding: 16px; height: 100%; background: #05060a; overflow-y: auto; }
    .reactor-card { background: #10131e; border: 1px solid var(--border-win); border-radius: 10px; padding: 16px; display: flex; flex-direction: column; gap: 12px; box-shadow: inset 0 2px 8px rgba(0,0,0,0.3); }
    .reactor-card h3 { font-size: 12px; color: var(--accent-cyan); text-transform: uppercase; letter-spacing: 0.05em; display: flex; justify-content: space-between; align-items: center; }
    .reactor-card h3 span.badge { background: rgba(52,211,153,0.15); color: var(--accent-green); padding: 2px 8px; border-radius: 10px; font-size: 10px; border: 1px solid var(--accent-green); }
    
    .bar-row { display: flex; flex-direction: column; gap: 6px; }
    .bar-label { display: flex; justify-content: space-between; font-size: 11px; color: var(--text-dim); font-weight: 600; }
    .bar-track { height: 10px; background: rgba(255,255,255,0.04); border-radius: 5px; overflow: hidden; border: 1px solid rgba(255,255,255,0.05); }
    .bar-fill { height: 100%; transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1); }
    .bar-fill.like { background: linear-gradient(90deg, #10b981, var(--accent-green)); }
    .bar-fill.ent { background: linear-gradient(90deg, #ef4444, var(--accent-pink)); }
    .bar-fill.mcts { background: linear-gradient(90deg, #6366f1, var(--accent-purple)); }

    /* MCTS Speculation Tree HUD */
    .mcts-tree-box { display: flex; flex-direction: column; gap: 8px; background: #0a0b12; padding: 12px; border-radius: 8px; border: 1px solid var(--border-win); }
    .mcts-node { display: flex; align-items: center; justify-content: space-between; font-size: 11px; padding: 6px 10px; background: #131622; border-radius: 6px; border-left: 3px solid var(--accent-purple); }
    .mcts-node span.t-name { font-weight: bold; color: var(--text-main); }
    .mcts-node span.t-score { color: var(--accent-purple); font-family: var(--font-mono); font-weight: bold; }

    /* Autonomous Capability & Genesis Studio */
    .studio-container { display: flex; height: 100%; overflow: hidden; }
    .studio-left { flex: 1; display: flex; flex-direction: column; padding: 14px; gap: 12px; overflow-y: auto; border-right: 1px solid var(--border-win); }
    .studio-right { width: 320px; background: rgba(7,8,14,0.9); padding: 14px; display: flex; flex-direction: column; gap: 14px; overflow-y: auto; }
    
    .section-title { font-size: 11px; font-weight: 800; color: var(--accent-gold); text-transform: uppercase; letter-spacing: 0.1em; display: flex; align-items: center; gap: 6px; }
    
    /* 20 Tools List */
    .tools-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 10px; }
    .tool-kpi-card { background: #121522; border: 1px solid var(--border-win); padding: 10px 12px; border-radius: 8px; display: flex; flex-direction: column; gap: 6px; transition: all 0.2s; }
    .tool-kpi-card:hover { border-color: var(--accent-cyan); transform: translateY(-1px); }
    .tool-kpi-card .tk-name { font-weight: 800; color: var(--accent-cyan); font-size: 12px; display: flex; justify-content: space-between; }
    .tool-kpi-card .tk-badge { font-size: 8px; background: rgba(34,211,238,0.1); color: var(--accent-cyan); padding: 2px 5px; border-radius: 3px; text-transform: uppercase; }
    .tool-kpi-card .tk-desc { font-size: 10px; color: var(--text-dim); line-height: 1.3; }

    /* Chronos Genesis Console */
    .genesis-console { flex: 1; background: #05060a; border: 1px solid var(--border-win); border-radius: 8px; padding: 12px; display: flex; flex-direction: column; gap: 8px; font-family: var(--font-mono); font-size: 11px; overflow-y: auto; box-shadow: inset 0 0 10px rgba(0,0,0,0.8); }
    .gen-log { display: flex; flex-direction: column; gap: 2px; border-bottom: 1px solid rgba(255,255,255,0.04); padding-bottom: 6px; }
    .gen-log .gl-hdr { display: flex; justify-content: space-between; color: var(--accent-gold); font-weight: bold; font-size: 10px; }
    .gen-log .gl-txt { color: var(--text-main); white-space: pre-wrap; }

    .action-btn { background: linear-gradient(90deg, #10b981, #059669); color: #000; border: none; padding: 10px; border-radius: 6px; font-weight: 800; font-family: var(--font-mono); font-size: 11px; letter-spacing: 0.05em; cursor: pointer; box-shadow: 0 0 12px rgba(16,185,129,0.3); transition: all 0.2s; }
    .action-btn:hover { box-shadow: 0 0 18px rgba(16,185,129,0.6); transform: scale(1.02); }
    .action-btn.hypnos { background: linear-gradient(90deg, #6366f1, #4f46e5); box-shadow: 0 0 12px rgba(99,102,241,0.3); }
    .action-btn.hypnos:hover { box-shadow: 0 0 18px rgba(99,102,241,0.6); }

    /* Dock Taskbar */
    .dock {
      position: absolute;
      bottom: 20px;
      left: 50%;
      transform: translateX(-50%);
      background: rgba(13, 15, 24, 0.85);
      backdrop-filter: blur(16px);
      border: 1px solid var(--border-win);
      padding: 8px 16px;
      border-radius: 24px;
      display: flex;
      gap: 16px;
      z-index: 10000;
      box-shadow: 0 16px 48px rgba(0,0,0,0.9), 0 0 0 1px rgba(255,255,255,0.05);
    }

    .dock-item {
      display: flex; align-items: center; gap: 8px; color: var(--text-dim); cursor: pointer; font-size: 12px; font-weight: 700; transition: all 0.2s; padding: 6px 12px; border-radius: 14px;
    }
    .dock-item:hover { color: var(--accent-cyan); background: rgba(255,255,255,0.05); transform: translateY(-2px); }
    .dock-item.active { color: var(--accent-purple); background: rgba(167,139,250,0.1); box-shadow: 0 0 14px rgba(167,139,250,0.3); }
    .dock-item .d-icon { width: 14px; height: 14px; border-radius: 4px; background: var(--text-dim); transition: background 0.2s, box-shadow 0.2s; }
    .dock-item:hover .d-icon { background: var(--accent-cyan); box-shadow: 0 0 8px var(--accent-cyan); }
    .dock-item.active .d-icon { background: var(--accent-purple); box-shadow: 0 0 10px var(--accent-purple); }

    /* Custom cyber scrollbars */
    ::-webkit-scrollbar { width: 6px; height: 6px; }
    ::-webkit-scrollbar-track { background: rgba(0,0,0,0.3); }
    ::-webkit-scrollbar-thumb { background: var(--border-win); border-radius: 3px; }
    ::-webkit-scrollbar-thumb:hover { background: var(--accent-purple); }
  </style>
</head>
<body>

  <!-- Top Telemetry HUD -->
  <div class="nav-hud">
    <div class="brand">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/></svg>
      Aether <span class="ultra">HermesOS</span> <span class="ver">v4.0 PARADIGM</span>
    </div>
    
    <div class="telemetry-pills">
      <div class="pill">Requests: <span class="val" id="hud-req">0</span></div>
      <div class="pill">Cache Hits: <span class="val" id="hud-cache">0</span></div>
      <div class="pill">MCTS Active Rollouts: <span class="val gold">3 Branches</span></div>
      <div class="pill">HCM Holographic State: <span class="val">16 KB Locked</span></div>
      <div class="pill">ATD Confidence: <span class="val" id="hud-atd">100%</span></div>
    </div>

    <div class="tools-hud">
      <span style="color: var(--text-dim); font-weight: bold;">Cognitive Persona:</span>
      <select id="persona-select">
        <option value="hermes">⚡ Hermes Ultimate Kernel (Unified OS)</option>
        <option value="claude">🧠 Claude Elite Architect (Deep Synthesis & MCTS)</option>
        <option value="arena">🛠️ Arena Active Engineer (Autonomous Git & Sandboxed Bash)</option>
      </select>
    </div>
  </div>

  <!-- Main Desktop Area -->
  <div class="desktop-area" id="desktop">
    <div class="desktop-grid"></div>

    <!-- Window 1: AetherOS Masterpiece Shell -->
    <div class="os-win active" style="width: 580px; height: 420px; top: 30px; left: 30px;" id="win-term">
      <div class="win-header">
        <div class="title"><div class="title-dot" style="color: var(--accent-purple);"></div>AetherOS Masterpiece Shell (20-Tool Multi-Persona)</div>
        <div class="win-controls">
          <button class="win-btn min" onclick="toggleWin('term')"></button>
          <button class="win-btn max" onclick="maxWin('term')"></button>
          <button class="win-btn close" onclick="closeWin('term')"></button>
        </div>
      </div>
      <div class="win-body">
        <div class="term-container">
          <div class="term-output" id="term-out">
            <div class="term-line">
              <span class="tool-txt">⚡ Alpha Engine v4.0 Masterpiece loaded. The Genesis Reactor is active 24/7. Type any high-level objective, Git orchestration prompt, or destructive codebase analysis query below.</span>
            </div>
          </div>
          <div class="term-input-box">
            <input type="text" id="term-in" placeholder="Enter autonomous agent goal... (e.g., 'orchestrate git status and trigger mcts speculative exploration')" onkeydown="if(event.key==='Enter') runAgent()">
            <button onclick="runAgent()">EXECUTE</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Window 2: Akasha Semantic Graph & HCM Fixed Matrix -->
    <div class="os-win" style="width: 500px; height: 360px; top: 30px; left: 640px;" id="win-graph">
      <div class="win-header">
        <div class="title"><div class="title-dot" style="color: var(--accent-cyan);"></div>Akasha Semantic Memory & Holographic HCM Matrix</div>
        <div class="win-controls">
          <button class="win-btn min" onclick="toggleWin('graph')"></button>
          <button class="win-btn max" onclick="maxWin('graph')"></button>
          <button class="win-btn close" onclick="closeWin('graph')"></button>
        </div>
      </div>
      <div class="win-body">
        <canvas class="graph-canvas" id="canvas-graph"></canvas>
        <div class="graph-hud">
          <div class="g-title">TF-IDF Vector Corpus & Edge Matrix</div>
          <div id="graph-hud-txt" style="color: var(--text-main);">0 Nodes &middot; 0 Adjacency Edges &middot; 1024-Dim FFT Pairs</div>
          <div style="color: var(--accent-green); font-weight: bold; margin-top: 2px;">⚡ Speculative Prefetcher Active</div>
        </div>
      </div>
    </div>

    <!-- Window 3: Cognitive Triple-Reactor Matrix (ATD/CLT/MCTS) -->
    <div class="os-win" style="width: 500px; height: 380px; top: 410px; left: 640px;" id="win-reactor">
      <div class="win-header">
        <div class="title"><div class="title-dot" style="color: var(--accent-pink);"></div>Cognitive Triple-Reactor Cluster (ATD / CLT / MCTS)</div>
        <div class="win-controls">
          <button class="win-btn min" onclick="toggleWin('reactor')"></button>
          <button class="win-btn max" onclick="maxWin('reactor')"></button>
          <button class="win-btn close" onclick="closeWin('reactor')"></button>
        </div>
      </div>
      <div class="win-body">
        <div class="reactor-container">
          
          <!-- Innovation #11: MCTS Speculative Rollouts -->
          <div class="reactor-card">
            <h3><span>Innovation #11: MCTS Latent Rollouts</span> <span class="badge" style="background: rgba(99,102,241,0.15); color: var(--accent-purple); border-color: var(--accent-purple);">3-DEPTH TREES</span></h3>
            <div class="bar-row">
              <div class="bar-label"><span>Monte Carlo UCT Tree Exploration Confidence</span> <span>0.91 UCT</span></div>
              <div class="bar-track"><div class="bar-fill mcts" style="width: 91%;"></div></div>
            </div>
            <div class="mcts-tree-box">
              <div class="mcts-node"><span class="t-name">Branch A: Self-Evolving Codebase Analysis</span> <span class="t-score">+0.88 ATD Val</span></div>
              <div class="mcts-node"><span class="t-name">Branch B: Latent Topological TF-IDF Matrix</span> <span class="t-score">+0.94 ATD Val</span></div>
            </div>
          </div>

          <!-- Innovation #10: ATD Likelihood-Entropy Dueling -->
          <div class="reactor-card">
            <h3><span>Innovation #10: ATD Contestation</span> <span class="badge" id="atd-status">VALIDATED</span></h3>
            <div class="bar-row">
              <div class="bar-label"><span>Likelihood (Graph A - Standard Autoregressive)</span> <span id="val-like">0.86</span></div>
              <div class="bar-track"><div class="bar-fill like" style="width: 86%;" id="bar-like"></div></div>
            </div>
            <div class="bar-row">
              <div class="bar-label"><span>Structural Entropy (Graph B - Chaotic Divergence)</span> <span id="val-ent">0.23</span></div>
              <div class="bar-track"><div class="bar-fill ent" style="width: 23%;" id="bar-ent"></div></div>
            </div>
          </div>

        </div>
      </div>
    </div>

    <!-- Window 4: Autonomous Genesis Reactor & Capability Studio -->
    <div class="os-win" style="width: 580px; height: 380px; top: 470px; left: 30px;" id="win-studio">
      <div class="win-header">
        <div class="title"><div class="title-dot" style="color: var(--accent-gold);"></div>Capability Studio & 24/7 Genesis Reactor</div>
        <div class="win-controls">
          <button class="win-btn min" onclick="toggleWin('studio')"></button>
          <button class="win-btn max" onclick="maxWin('studio')"></button>
          <button class="win-btn close" onclick="closeWin('studio')"></button>
        </div>
      </div>
      <div class="win-body">
        <div class="studio-container">
          
          <!-- Left: 20 Divine Tools Surface -->
          <div class="studio-left">
            <div class="section-title"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 9.01l-9.19 9.19a2 2 0 0 1-2.83-2.83l9.19-9.19a6 6 0 0 1 9.01-7.94z"/></svg> 20 Active OS-Control Tools Surface</div>
            <div class="tools-grid">
              <div class="tool-kpi-card"><div class="tk-name"><span>git_orchestrate</span> <span class="tk-badge">new #14</span></div><div class="tk-desc">Autonomous Git repo operations (status, commit, checkout).</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>code_analyze</span> <span class="tk-badge">new #15</span></div><div class="tk-desc">Structural complexity & AST robustness prober.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>sandbox_eval</span> <span class="tk-badge">new #16</span></div><div class="tk-desc">Pure isolated Python/Bash timeout execution.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>net_probe</span> <span class="tk-badge">new #17</span></div><div class="tk-desc">HTTP/REST external prober rollouts.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>hypnos_sleep</span> <span class="tk-badge">new #18</span></div><div class="tk-desc">Neural memory dream consolidation.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>mcts_speculate</span> <span class="tk-badge">new #19</span></div><div class="tk-desc">Monte Carlo speculative exploration tree.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>genesis_toggle</span> <span class="tk-badge">new #20</span></div><div class="tk-desc">Toggle 24/7 background self-creation loop.</div></div>
              <div class="tool-kpi-card"><div class="tk-name"><span>skill_register</span> <span class="tk-badge">new #13</span></div><div class="tk-desc">Dynamic capability auto-creation at runtime.</div></div>
            </div>
            
            <div class="section-title" style="margin-top: 8px;"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg> Auto-Authored Runtime Skills</div>
            <div class="tools-grid" id="dynamic-skills-list">
              <!-- Dynamically populated -->
            </div>
          </div>

          <!-- Right: Chronos Genesis Live Console & Hypnos Slumber Studio -->
          <div class="studio-right">
            <div class="section-title" style="color: var(--accent-green);"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg> The Genesis Reactor (24/7 Autopoiesis)</div>
            <div class="genesis-console" id="gen-console">
              <div class="gen-log"><div class="gl-hdr"><span>Chronos Background Loop</span> <span>Active 24/7</span></div><div class="gl-txt" style="color: var(--accent-green);">Aether Chronos permanently live. Self-reflecting on codebase...</div></div>
            </div>
            <button class="action-btn" onclick="toggleGenesisLoop()">TOGGLE 24/7 GENESIS REACTOR</button>

            <div class="section-title" style="color: var(--accent-purple); margin-top: 4px;"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg> Innovation #12: Hypnos Slumber Protocol</div>
            <button class="action-btn hypnos" onclick="triggerHypnosSleep()">TRIGGER NEURAL SLUMBER PROTOCOL</button>
          </div>

        </div>
      </div>
    </div>
  </div>

  <!-- Bottom Taskbar Dock -->
  <div class="dock">
    <div class="dock-item active" onclick="showWin('term')" id="dock-term"><div class="d-icon" style="background: var(--accent-purple);"></div>Aether Shell</div>
    <div class="dock-item" onclick="showWin('graph')" id="dock-graph"><div class="d-icon" style="background: var(--accent-cyan);"></div>Akasha Graph</div>
    <div class="dock-item" onclick="showWin('reactor')" id="dock-reactor"><div class="d-icon" style="background: var(--accent-pink);"></div>Triple Reactor</div>
    <div class="dock-item" onclick="showWin('studio')" id="dock-studio"><div class="d-icon" style="background: var(--accent-gold);"></div>Capability Studio</div>
  </div>

  <script>
    // Window state management
    let zIdx = 100;
    const wins = ['term', 'graph', 'reactor', 'studio'];

    function setupWindows() {
      wins.forEach(w => {
        const win = document.getElementById('win-' + w);
        if(!win) return;
        const header = win.querySelector('.win-header');
        
        win.addEventListener('mousedown', () => {
          zIdx += 1; win.style.zIndex = zIdx;
          document.querySelectorAll('.os-win').forEach(x => x.classList.remove('active'));
          win.classList.add('active');
          document.querySelectorAll('.dock-item').forEach(x => x.classList.remove('active'));
          document.getElementById('dock-' + w).classList.add('active');
        });

        let isDown = false, startX, startY, winX, winY;
        header.addEventListener('mousedown', e => {
          if(e.target.classList.contains('win-btn')) return;
          isDown = true;
          startX = e.clientX; startY = e.clientY;
          winX = win.offsetLeft; winY = win.offsetTop;
        });

        window.addEventListener('mousemove', e => {
          if(!isDown) return;
          win.style.left = (winX + e.clientX - startX) + 'px';
          win.style.top = Math.max(0, winY + e.clientY - startY) + 'px';
        });

        window.addEventListener('mouseup', () => { isDown = false; });
      });
    }

    function toggleWin(w) { const win = document.getElementById('win-' + w); if(win) win.style.display = win.style.display === 'none' ? 'flex' : 'none'; }
    function closeWin(w) { const win = document.getElementById('win-' + w); if(win) win.style.display = 'none'; }
    function showWin(w) { 
      const win = document.getElementById('win-' + w); if(!win) return;
      win.style.display = 'flex'; zIdx += 1; win.style.zIndex = zIdx;
      document.querySelectorAll('.os-win').forEach(x => x.classList.remove('active'));
      win.classList.add('active');
      document.querySelectorAll('.dock-item').forEach(x => x.classList.remove('active'));
      document.getElementById('dock-' + w).classList.add('active');
    }

    function maxWin(w) {
      const win = document.getElementById('win-' + w); if(!win) return;
      if(win.dataset.maximized === 'true') {
        win.dataset.maximized = 'false';
        win.style.width = win.dataset.oldW; win.style.height = win.dataset.oldH;
        win.style.top = win.dataset.oldT; win.style.left = win.dataset.oldL;
      } else {
        win.dataset.maximized = 'true';
        win.dataset.oldW = win.style.width; win.dataset.oldH = win.style.height;
        win.dataset.oldT = win.style.top; win.dataset.oldL = win.style.left;
        win.style.width = '100%'; win.style.height = 'calc(100% - 42px)';
        win.style.top = '42px'; win.style.left = '0px';
      }
    }

    // Agent execution runner
    async function runAgent() {
      const input = document.getElementById('term-in');
      const out = document.getElementById('term-out');
      const goal = input.value.trim();
      if(!goal) return;

      const persona = document.getElementById('persona-select').value;
      input.value = '';

      const userDiv = document.createElement('div');
      userDiv.className = 'term-line';
      userDiv.innerHTML = `<span class="prompt">User ></span> <span class="user-txt">${goal}</span>`;
      out.appendChild(userDiv);
      out.scrollTop = out.scrollHeight;

      const loadDiv = document.createElement('div');
      loadDiv.className = 'term-line';
      loadDiv.innerHTML = `<span class="prompt" style="color: var(--accent-gold);">AetherOS [${persona.toUpperCase()}] ></span> <span class="ai-txt">Perceiving sandbox context & running Triple-Reactor (MCTS/ATD/CLT) collision...</span>`;
      out.appendChild(loadDiv);
      out.scrollTop = out.scrollHeight;

      try {
        const resp = await fetch('/v1/agent/run', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ goal: goal, context: { persona: persona }, max_iterations: 15 })
        });

        const data = await resp.json();
        out.removeChild(loadDiv);

        if(!data.ok) {
          const errDiv = document.createElement('div'); errDiv.className = 'term-line';
          errDiv.innerHTML = `<span class="prompt" style="color: #ef4444;">Kernel Error ></span> <span class="ai-txt" style="color: #ef4444;">${data.error}</span>`;
          out.appendChild(errDiv);
        } else {
          if(data.tool_calls && data.tool_calls.length > 0) {
            const tcDiv = document.createElement('div'); tcDiv.className = 'term-line';
            const callsHtml = data.tool_calls.map(c => `[Action Issued]: ${c.name} &middot; params: ${JSON.stringify(c.params)}`).join('\n');
            tcDiv.innerHTML = `<span class="tool-txt">${callsHtml}</span>`;
            out.appendChild(tcDiv);
          }

          const resDiv = document.createElement('div'); resDiv.className = 'term-line';
          resDiv.innerHTML = `<span class="prompt" style="color: var(--accent-cyan);">AetherOS Result (${data.iterations} Iterations) ></span> <span class="ai-txt">${data.result}</span>`;
          out.appendChild(resDiv);
        }
      } catch(e) {
        out.removeChild(loadDiv);
        const errDiv = document.createElement('div'); errDiv.className = 'term-line';
        errDiv.innerHTML = `<span class="prompt" style="color: #ef4444;">Network Error ></span> <span class="ai-txt" style="color: #ef4444;">${e.message}</span>`;
        out.appendChild(errDiv);
      }
      out.scrollTop = out.scrollHeight;
      refreshState();
    }

    // Toggle 24/7 Genesis Evolution Background Reactor
    async function toggleGenesisLoop() {
      try {
        const resp = await fetch('/v1/genesis/toggle', { method: 'POST' });
        const data = await resp.json();
        alert('Aether Chronos Genesis Reactor active state: ' + data.genesis_active);
        refreshState();
      } catch(e) { alert('Error: ' + e.message); }
    }

    // Trigger Hypnos Slumber Protocol
    async function triggerHypnosSleep() {
      try {
        const resp = await fetch('/v1/hypnos/sleep', { method: 'POST' });
        const data = await resp.json();
        alert('🌌 Masterpiece! ' + data.message + '\nSynthesized Insights: ' + data.consolidation_report.insights_synthesized.length);
        refreshState();
      } catch(e) { alert('Error: ' + e.message); }
    }

    // Refresh telemetry, dynamic skills, and live Genesis logs
    async function refreshState() {
      try {
        const [hResp, sResp, gResp] = await Promise.all([
          fetch('/health'), fetch('/v1/skills'), fetch('/v1/genesis/logs')
        ]);
        
        if(hResp.ok) {
          const hData = await hResp.json();
          document.getElementById('hud-req').textContent = hData.requests;
          document.getElementById('hud-cache').textContent = hData.cache_hits;
          const atdRate = hData.atd_verifications > 0 ? Math.round(hData.atd_validated / hData.atd_verifications * 100) : 100;
          document.getElementById('hud-atd').textContent = atdRate + '%';
          document.getElementById('graph-hud-txt').textContent = `Akasha Network: ${hData.nodes} Nodes | ${hData.edges} Edges | HCM Active Pairs: ${hData.hcm_active_pairs}`;
          
          const likeVal = 0.84 + Math.sin(Date.now()*0.001)*0.07;
          const entVal = 0.21 + Math.cos(Date.now()*0.0013)*0.08;
          document.getElementById('val-like').textContent = likeVal.toFixed(2);
          document.getElementById('bar-like').style.width = Math.round(likeVal*100) + '%';
          document.getElementById('val-ent').textContent = entVal.toFixed(2);
          document.getElementById('bar-ent').style.width = Math.round(entVal*100) + '%';
        }

        if(sResp.ok) {
          const sData = await sResp.json();
          if(sData.skills) {
            const list = document.getElementById('dynamic-skills-list');
            list.innerHTML = sData.skills.map(s => `
              <div class="tool-kpi-card" style="border-color: var(--accent-green);">
                <div class="tk-name"><span>${s.name}</span> <span class="tk-badge" style="color: var(--accent-green); background: rgba(52,211,153,0.1);">${s.language || 'dynamic'}</span></div>
                <div class="tk-desc">${s.description}</div>
              </div>
            `).join('');
          }
        }

        if(gResp.ok) {
          const gData = await gResp.json();
          if(gData.genesis_logs && gData.genesis_logs.length > 0) {
            const console = document.getElementById('gen-console');
            console.innerHTML = gData.genesis_logs.slice(-15).map(l => `
              <div class="gen-log">
                <div class="gl-hdr"><span>[${l.action_type}]</span> <span>+${l.timestamp}s</span></div>
                <div class="gl-txt" style="color: ${l.success ? 'var(--text-main)' : '#ef4444'};">${l.details}</div>
              </div>
            `).join('');
            console.scrollTop = console.scrollHeight;
          }
        }
      } catch(e) {}
    }

    // Force-directed Canvas Akasha Network Visualizer
    function setupGraphCanvas() {
      const canvas = document.getElementById('canvas-graph'); if(!canvas) return;
      const ctx = canvas.getContext('2d');
      let w = canvas.width = canvas.offsetWidth; let h = canvas.height = canvas.offsetHeight;

      const nodes = Array.from({length: 42}, (_, i) => ({
        x: Math.random()*w, y: Math.random()*h,
        vx: (Math.random()-0.5)*0.7, vy: (Math.random()-0.5)*0.7,
        r: i === 0 ? 9 : (i < 6 ? 5 : 3),
        color: i === 0 ? '#a78bfa' : (i < 6 ? '#22d3ee' : '#34d399')
      }));

      function anim() {
        w = canvas.width = canvas.offsetWidth; h = canvas.height = canvas.offsetHeight;
        ctx.fillStyle = '#05060a'; ctx.fillRect(0, 0, w, h);

        nodes.forEach(n => {
          n.x += n.vx; n.y += n.vy;
          if(n.x < 0 || n.x > w) n.vx *= -1;
          if(n.y < 0 || n.y > h) n.vy *= -1;
        });

        ctx.lineWidth = 0.8;
        for(let i=0; i<nodes.length; i++) {
          for(let j=i+1; j<nodes.length; j++) {
            const dx = nodes[i].x - nodes[j].x; const dy = nodes[i].y - nodes[j].y;
            const dist = Math.sqrt(dx*dx + dy*dy);
            if(dist < 90) {
              ctx.strokeStyle = `rgba(34, 211, 238, ${1 - dist/90})`;
              ctx.beginPath(); ctx.moveTo(nodes[i].x, nodes[i].y); ctx.lineTo(nodes[j].x, nodes[j].y); ctx.stroke();
            }
          }
        }

        nodes.forEach(n => {
          ctx.beginPath(); ctx.arc(n.x, n.y, n.r, 0, Math.PI*2);
          ctx.fillStyle = n.color; ctx.fill();
          ctx.shadowBlur = 10; ctx.shadowColor = n.color;
        });

        requestAnimationFrame(anim);
      }
      anim();
    }

    window.onload = () => { setupWindows(); setupGraphCanvas(); refreshState(); setInterval(refreshState, 3000); };
  </script>
</body>
</html>"#
    .to_string()
}
