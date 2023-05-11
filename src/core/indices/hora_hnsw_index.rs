use crate::core::index::{Index, IndexError, Metric};

enum IndexType {

}

// DO NOT USE: This is more so a proof of concept. Hora is a bit deprecated so I won't bother finishing it.
pub struct HoraHNSWIndex {
    index: hora::index::hnsw_idx::HNSWIndex<f32, String>,
    dimension: usize,
}

impl HoraHNSWIndex {
    fn get_metric(metric: Metric) -> hora::core::metrics::Metric  {
        match metric {
            Metric::Cosine => hora::core::metrics::Metric::Cosine,
            Metric::InnerProduct => hora::core::metrics::Metric::InnerProduct,
            Metric::Euclidean => hora::core::metrics::Metric::Euclidean,
        }
    }
}

impl Index for HoraHNSWIndex {
    fn new(dimension: usize, metric: Metric) -> Result<Self, IndexError> {
        let metric = HoraHNSWIndex::get_metric(metric);

        let params = hora::index::hnsw_params::HNSWParams::default()
            .e_type(metric)
            .has_deletion(true);
        
        let index = hora::index::hnsw_idx::HNSWIndex::new(dimension, &params);
        
        Ok(HoraHNSWIndex {
            index,
            dimension,
        })
    }

    fn load(path: &str) -> Result<Self, IndexError> where Self: Sized {
        let index = hora::index::hnsw_idx::HNSWIndex::load(path)
            .map_err(|e| IndexError::LoadError(e.to_string()))?;

        Ok(HoraHNSWIndex {
            index,
            dimension: index.dimension(),
        })
    }

    fn save(&self, path: &str) -> Result<(), IndexError> {
        todo!()
    }

    fn insert(&mut self, vector: Vec<f32>, index: String) -> Result<(), IndexError> {
        self.index
            .add(&vector, index)
            .map_err(|e| IndexError::Error(e.to_string()))
    }

    fn remove(&mut self, index: String) -> Result<(), IndexError> {
        todo!() // Hora is a bit crap - realised in retrospect.
    }

    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<String>, IndexError> {
        Ok(self.index
            .search(&query, k))
    }
}
