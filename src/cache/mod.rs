pub struct Cache {
    cache_file: String,
}

impl Cache {
    pub fn new(name: String) -> Cache {
        Cache {
            cache_file: std::env::temp_dir()
                .join(format!("jira-cli-cache-{}.json", name))
                .to_string_lossy()
                .to_string(),
        }
    }
    pub fn save(&self, data: &str) {
        std::fs::write(&self.cache_file, data).expect("Failed to write cache file");
    }

    pub fn load(&self) -> Option<String> {
        if self.exists() && !self.is_valid() {
            return None;
        }

        match std::fs::read_to_string(&self.cache_file) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.cache_file).exists() && self.is_valid()
    }

    pub fn is_valid(&self) -> bool {
        let metadata = std::fs::metadata(&self.cache_file).expect("Failed to read cache metadata");
        let created = metadata
            .created()
            .expect("Failed to read cache creation time");
        let now = std::time::SystemTime::now();
        let duration = now
            .duration_since(created)
            .expect("Failed to calculate duration");
        duration.as_secs() < 60 * 60 * 12
    }
}
