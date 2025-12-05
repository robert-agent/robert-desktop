use crate::{Edge, GraphError, GraphStore, Node, VectorStore};
use async_trait::async_trait;
use serde::Deserialize;
use std::path::PathBuf;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

#[derive(Clone)]
pub struct SurrealStore {
    db: Surreal<Db>,
}

impl SurrealStore {
    pub async fn new(path: PathBuf) -> Result<Self, GraphError> {
        let db = Surreal::new::<RocksDb>(path)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        db.use_ns("robert")
            .use_db("core")
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        Ok(Self { db })
    }
}

#[async_trait]
impl GraphStore for SurrealStore {
    async fn add_node(&self, node: Node) -> Result<(), GraphError> {
        let _: Option<Node> = self
            .db
            .create(("node", &node.id))
            .content(node)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn add_edge(&self, edge: Edge) -> Result<(), GraphError> {
        // In SurrealDB, edges are relations: RELATE source->relation->target
        // We need to construct the query manually or use the query builder if available for relations
        // For now, let's use a raw query for flexibility with dynamic table names

        let sql = format!(
            "RELATE node:{}->{}->node:{} SET weight = {}",
            edge.source, edge.relation, edge.target, edge.weight
        );

        self.db
            .query(sql)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn get_node(&self, id: &str) -> Result<Node, GraphError> {
        let node: Option<Node> = self
            .db
            .select(("node", id))
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        node.ok_or(GraphError::NotFound(id.to_string()))
    }

    async fn update_node(&self, node: Node) -> Result<(), GraphError> {
        let _: Option<Node> = self
            .db
            .update(("node", &node.id))
            .content(node)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get_neighbors(&self, id: &str) -> Result<Vec<(Edge, Node)>, GraphError> {
        // Get all outgoing edges from this node
        // In SurrealDB, relations are stored as: node:source->relation_name->node:target
        // We need to query all relations from this node

        let sql = format!(
            "SELECT ->? as edges FROM node:{} FETCH edges, edges.out",
            id
        );

        let mut response = self
            .db
            .query(sql)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        #[derive(Deserialize, Debug)]
        struct RelationEdge {
            #[serde(rename = "in")]
            source: surrealdb::sql::Thing,
            #[serde(rename = "out")]
            target: surrealdb::sql::Thing,
        }

        #[derive(Deserialize, Debug)]
        struct NeighborResult {
            edges: Option<Vec<RelationEdge>>,
        }

        let result: Vec<NeighborResult> = response
            .take(0)
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        let mut neighbors = Vec::new();

        if let Some(first) = result.into_iter().next() {
            if let Some(edges) = first.edges {
                for rel_edge in edges {
                    // Fetch the target node
                    let target_node: Option<Node> = self
                        .db
                        .select((rel_edge.target.tb.clone(), rel_edge.target.id.to_string()))
                        .await
                        .map_err(|e| GraphError::Storage(e.to_string()))?;

                    if let Some(node) = target_node {
                        // Create Edge representation
                        let edge = Edge {
                            source: rel_edge.source.id.to_string(),
                            target: rel_edge.target.id.to_string(),
                            relation: rel_edge.target.tb.clone(), // Relation name is the table name
                            weight: 1.0, // Default weight, could be stored in relation properties
                        };

                        neighbors.push((edge, node));
                    }
                }
            }
        }

        Ok(neighbors)
    }

    async fn query_by_partition(&self, partition_id: &str) -> Result<Vec<Node>, GraphError> {
        let sql = "SELECT * FROM node WHERE partition_id = $partition";

        let mut response = self
            .db
            .query(sql)
            .bind(("partition", partition_id))
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        let nodes: Vec<Node> = response
            .take(0)
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        Ok(nodes)
    }

    async fn get_neighbors_in_partition(
        &self,
        id: &str,
        partition_id: &str,
    ) -> Result<Vec<(Edge, Node)>, GraphError> {
        // Get neighbors filtered by partition
        let all_neighbors = self.get_neighbors(id).await?;

        // Filter by partition
        let filtered: Vec<(Edge, Node)> = all_neighbors
            .into_iter()
            .filter(|(edge, node)| {
                edge.partition_id == partition_id && node.partition_id == partition_id
            })
            .collect();

        Ok(filtered)
    }
}

#[async_trait]
impl VectorStore for SurrealStore {
    async fn add_embedding(&self, id: &str, vector: Vec<f32>) -> Result<(), GraphError> {
        // Update the node with the embedding
        // Assuming 'embedding' field on the node
        let sql = format!("UPDATE node:{} SET embedding = $vector", id);
        self.db
            .query(sql)
            .bind(("vector", vector))
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn search(
        &self,
        vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<(String, f32)>, GraphError> {
        // Vector search query
        // SELECT id, vector::similarity::cosine(embedding, $query) as score FROM node ORDER BY score DESC LIMIT $limit
        let sql = "SELECT id, vector::similarity::cosine(embedding, $query) as score FROM node ORDER BY score DESC LIMIT $limit";

        let mut response = self
            .db
            .query(sql)
            .bind(("query", vector))
            .bind(("limit", limit))
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        // Parse results
        // This requires a struct to deserialize into
        #[derive(Deserialize)]
        struct SearchResult {
            id: surrealdb::sql::Thing,
            score: f32,
        }

        let results: Vec<SearchResult> = response
            .take(0)
            .map_err(|e| GraphError::Storage(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|r| (r.id.id.to_string(), r.score))
            .collect())
    }
}
