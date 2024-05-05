use std::{
    collections::HashMap,
    fmt,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }
    pub fn inc(&self, key: impl AsRef<str>) {
        let key = key.as_ref();
        if let Some(counter) = self.data.get(key) {
            counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: self.data.clone(),
        }
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
