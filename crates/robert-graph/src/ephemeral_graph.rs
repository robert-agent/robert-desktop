use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::kosaraju_scc;
use crate::{Node, Edge};
use std::collections::HashMap;

pub struct EphemeralGraph {
    graph: DiGraph<Node, f32>,
    node_indices: HashMap<String, NodeIndex>,
}

impl EphemeralGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn from_nodes_and_edges(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        let mut ephem = Self::new();
        
        for node in nodes {
            ephem.add_node(node);
        }

        for edge in edges {
            ephem.add_edge(edge);
        }

        ephem
    }

    pub fn add_node(&mut self, node: Node) {
        if !self.node_indices.contains_key(&node.id) {
            let id = node.id.clone();
            let idx = self.graph.add_node(node);
            self.node_indices.insert(id, idx);
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        if let (Some(&source), Some(&target)) = (
            self.node_indices.get(&edge.source),
            self.node_indices.get(&edge.target),
        ) {
            self.graph.add_edge(source, target, edge.weight);
        }
    }

    pub fn get_strongly_connected_components(&self) -> Vec<Vec<Node>> {
        let scc = kosaraju_scc(&self.graph);
        scc.into_iter()
            .map(|indices| {
                indices.into_iter()
                    .map(|idx| self.graph[idx].clone())
                    .collect()
            })
            .collect()
    }

    // Add more algorithms as needed (PageRank, Community Detection, etc.)
}
