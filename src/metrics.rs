// metrics data structure
// 基本功能：inc/dec/snapshot

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}