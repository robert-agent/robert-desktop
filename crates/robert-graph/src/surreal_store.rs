use crate::{GraphError, GraphStore, Node, Edge, VectorStore};
use async_trait::async_trait;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use std::path::PathBuf;
use serde::Deserialize;

pub struct SurrealStore {
    db: Surreal<Db>,
}

impl SurrealStore {
    pub async fn new(path: PathBuf) -> Result<Self, GraphError> {
        let db = Surreal::new::<RocksDb>(path).await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        
        db.use_ns("robert").use_db("core").await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
            
        Ok(Self { db })
    }
}

#[async_trait]
impl GraphStore for SurrealStore {
    async fn add_node(&self, node: Node) -> Result<(), GraphError> {
        let _: Option<Node> = self.db
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
        
        self.db.query(sql).await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
            
        Ok(())
    }

    async fn get_node(&self, id: &str) -> Result<Node, GraphError> {
        let node: Option<Node> = self.db.select(("node", id)).await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
            
        node.ok_or(GraphError::NotFound(id.to_string()))
    }

    async fn update_node(&self, node: Node) -> Result<(), GraphError> {
        let _: Option<Node> = self.db
            .update(("node", &node.id))
            .content(node)
            .await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get_neighbors(&self, id: &str) -> Result<Vec<(Edge, Node)>, GraphError> {
        // Query: SELECT ->? as edge, ->?.out as target FROM node:id
        // This is a bit complex to map directly to our Edge struct which expects strings
        // Let's simplify: fetch edges where out = target
        
        // SurrealDB graph traversal: SELECT ->? as relation FROM node:id
        // We want the edge details and the target node
        
        // Let's use a custom query
        let sql = format!("SELECT * FROM node:{}->?", id);
        let _response = self.db.query(sql).await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
            
        // Parsing this dynamic result is tricky without a specific struct
        // For Alpha, let's just return empty or implement a simpler version
        // TODO: Implement proper neighbor parsing
        Ok(vec![])
    }
}

#[async_trait]
impl VectorStore for SurrealStore {
    async fn add_embedding(&self, id: &str, vector: Vec<f32>) -> Result<(), GraphError> {
        // Update the node with the embedding
        // Assuming 'embedding' field on the node
        let sql = format!("UPDATE node:{} SET embedding = $vector", id);
        self.db.query(sql).bind(("vector", vector)).await
            .map_err(|e| GraphError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn search(&self, vector: Vec<f32>, limit: usize) -> Result<Vec<(String, f32)>, GraphError> {
        // Vector search query
        // SELECT id, vector::similarity::cosine(embedding, $query) as score FROM node ORDER BY score DESC LIMIT $limit
        let sql = "SELECT id, vector::similarity::cosine(embedding, $query) as score FROM node ORDER BY score DESC LIMIT $limit";
        
        let mut response = self.db.query(sql)
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
        
        let results: Vec<SearchResult> = response.take(0)
            .map_err(|e| GraphError::Storage(e.to_string()))?;
            
        Ok(results.into_iter().map(|r| (r.id.id.to_string(), r.score)).collect())
    }
}
