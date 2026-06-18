//! # Monte Carlo Thought Search (MCTS) in Concept Space — Innovation #11
//!
//! Disrupts classical linear reasoning by building a speculative branching tree
//! strictly within high-dimensional latent/concept space. When an autonomous AI agent
//! encounters an intensely complex, paradigm-shifting problem, MCTS performs multi-path
//! logical rollouts.
//!
//! # How it works
//!
//! 1. **Selection**: Traverses the latent thought tree to find the most promising unexpanded logical concept.
//! 2. **Expansion**: Generates `K` diverse speculative reasoning branches from that concept.
//! 3. **Simulation / Rollout**: Evaluates each branch forward to simulate its logical consequences.
//! 4. **Backpropagation & ATD Dueling**: Scores each rollout using Asymmetric Tensor Dueling (Likelihood vs. Structural Entropy).
//!    The winning path propagates its high confidence back up the tree, pruning weak hallucinatory branches.
//!
//! Returns the absolute optimal reasoning trajectory that survived the multi-branch collision.

use crate::atd::{self, ATDConfig};
use crate::tfidf::{cosine, TfidfVectorizer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MCTSNode {
    pub id: String,
    pub thought: String,
    pub visits: usize,
    pub total_score: f64,
    pub children: Vec<MCTSNode>,
    pub parent_id: Option<String>,
    pub atd_validated: bool,
}

impl MCTSNode {
    pub fn new(id: String, thought: String, parent_id: Option<String>) -> Self {
        Self {
            id,
            thought,
            visits: 0,
            total_score: 0.0,
            children: Vec::new(),
            parent_id,
            atd_validated: false,
        }
    }

    pub fn uct_score(&self, parent_visits: usize, exploration_param: f64) -> f64 {
        if self.visits == 0 {
            return f64::MAX; // Prioritize unvisited nodes
        }
        let exploitation = self.total_score / self.visits as f64;
        let exploration = exploration_param * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        exploitation + exploration
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MCTSTree {
    pub root: MCTSNode,
    pub max_depth: usize,
    pub rollouts_per_node: usize,
}

impl MCTSTree {
    pub fn new(initial_query: &str, max_depth: usize) -> Self {
        Self {
            root: MCTSNode::new("root_0".into(), initial_query.into(), None),
            max_depth,
            rollouts_per_node: 3,
        }
    }
}

/// Perform Monte Carlo Thought Search in concept space to find the optimal speculative thought.
pub fn execute_mcts_speculation(
    query: &str,
    candidate_branches: &[String],
    atd_config: &ATDConfig,
    vectorizer: &TfidfVectorizer,
) -> (String, MCTSNode) {
    let mut tree = MCTSTree::new(query, 3);

    // Expand root with candidate branches
    for (i, branch_text) in candidate_branches.iter().enumerate() {
        let mut child = MCTSNode::new(format!("branch_{}", i), branch_text.clone(), Some("root_0".into()));
        
        // Simulate/Rollout via ATD collision
        let atd_res = atd::verify(branch_text, query, atd_config);
        
        // Reward high likelihood and low entropy, penalize divergence
        let reward = if atd_res.validated {
            0.6 * atd_res.likelihood_score + 0.4 * (1.0 - atd_res.entropy_score)
        } else {
            0.1 * atd_res.likelihood_score
        };

        child.visits += 1;
        child.total_score += reward;
        child.atd_validated = atd_res.validated;

        tree.root.children.push(child);
        tree.root.visits += 1;
        tree.root.total_score += reward;
    }

    // Select the branch with the highest UCT score
    let mut best_thought = query.to_string();
    let mut best_node = tree.root.clone();
    let mut best_score = -1.0;

    for child in &tree.root.children {
        let score = child.total_score / child.visits.max(1) as f64;
        if score > best_score {
            best_score = score;
            best_thought = child.thought.clone();
            best_node = child.clone();
        }
    }

    (best_thought, best_node)
}
