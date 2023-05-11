use std::{collections::VecDeque, path::Path};

use crate::core::index::Index;

use super::collection::Collection;

enum CollectionManagerError {
    Error(String),
    InvalidPath(String),
    CollectionNotFound(String),
    CollectionLoaded(String),
}

pub struct CollectionManager {
    collections: VecDeque<Collection>,
    collection_names: Vec<String>,
    max_size: usize,
    path: String,
}

impl CollectionManager {
    pub fn new(max_size: usize, path: String) -> CollectionManager {
        CollectionManager {
            collections: VecDeque::new(),
            collection_names: Vec::new(),
            max_size,
            path,
        }
    }

    fn load_collection_config(&mut self, collection_name: &str) -> Result<(), CollectionManagerError> {
        todo!()
    }

    fn load_collection(&mut self, name: &str) -> Result<&Collection, CollectionManagerError> {
        if !self.has_collection(name) {
            return Err(CollectionManagerError::CollectionNotFound(name.to_string()));
        }

        if self.has_loaded_collection(name) {
            return Err(CollectionManagerError::CollectionLoaded(name.to_string()));
        }

        let index = Index::load(
            Path::new(&self.path)
                .join(name.to_string())
                .to_str()
                .ok_or(CollectionManagerError::InvalidPath(name.to_string()))?
        ).map_err(|e| CollectionManagerError::Error(e.to_string()))?;


        todo!()
    }

    fn has_collection(&self, name: &str) -> bool {
        self.collection_names.contains(&name.to_string())
    }

    fn has_loaded_collection(&self, name: &str) -> bool {
        self.collections
            .iter()
            .any(|c| c.name == name)
    }

    pub fn get_collection(&mut self, name: &str) -> Result<&Collection, CollectionManagerError> {
        if !self.has_loaded_collection(name) {
            return self.load_collection(name)
        }
    
        let index = self.collections
            .iter()
            .position(|c| c.name == name)
            .ok_or(CollectionManagerError::CollectionNotFound(name.to_string()))?;

        let collection = self.collections
            .remove(index)
            .ok_or(CollectionManagerError::CollectionNotFound(name.to_string()))?
            .index
            .save();

        self.collections.push_front(collection);
        self.collection_names.push(name.to_string());

        self.collections
            .front()
            .ok_or(CollectionManagerError::CollectionNotFound(name.to_string()))
    }

    pub fn new_collection(&mut self, name: &str, index: Box<dyn Index>) -> Result<&Collection, CollectionManagerError> {
        todo!()
    }

    pub fn save_collection(&mut self, name: &str) -> Result<(), CollectionManagerError> {
        todo!()
    }

    pub fn delete_collection(&mut self, name: &str) -> Result<(), CollectionManagerError> {
        todo!()      
    }
}