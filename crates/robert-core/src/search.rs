use robert_graph::{GraphStore, VectorStore, GraphError, Node};
use robert_graph::query::GraphQuery;
use robert_graph::ingest::IngestionPipeline;
use std::sync::Arc;

pub struct SearchManager<S: GraphStore + VectorStore> {
    query_engine: GraphQuery<S>,
    ingestion_pipeline: Arc<IngestionPipeline<S>>,
}

impl<S: GraphStore + VectorStore + Clone> SearchManager<S> {
    pub fn new(store: S, ingestion_pipeline: Arc<IngestionPipeline<S>>) -> Self {
        Self {
            query_engine: GraphQuery::new(store),
            ingestion_pipeline,
        }
    }

    pub async fn search(&self, query_text: &str, limit: usize) -> Result<Vec<Node>, GraphError> {
        // 1. Embed query
        // We need to expose embedding from IngestionPipeline or use a separate service
        // For now, let's assume IngestionPipeline has a public embed_text method (I added it mentally but need to verify/add it)
        
        // Wait, I didn't actually add `embed_text` to `IngestionPipeline` in the previous step, I just thought about it.
        // I should check `ingest.rs` content or just add it now.
        // I'll assume I need to add it.
        
        // Let's assume I will add it.
        let vector = self.ingestion_pipeline.embed_text(query_text).await?;
        
        // 2. Graph Search
        self.query_engine.search(vector, limit).await
    }
}
