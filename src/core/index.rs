#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("index error: {0}")]
    Error(String),
    #[error("error loading index: {0}")]
    LoadError(String),
}

pub enum Metric {
    Cosine,
    InnerProduct,
    Euclidean,
}

pub trait Index {
    fn new(dimension: usize, metric: Metric) -> Result<Self, IndexError> where Self: Sized;
    fn load(path: &str) -> Result<Self, IndexError> where Self: Sized;
    fn save(path: &str) -> Result<(), IndexError>;
    fn delete(&self) -> Result<(), IndexError>;
    fn insert(&mut self, vector: Vec<f32>, index: String) -> Result<(), IndexError>;
    fn remove(&mut self, index: String) -> Result<(), IndexError>;
    fn search(&self, query: Vec<f32>, k: usize) -> Result<Vec<String>, IndexError>;
}
