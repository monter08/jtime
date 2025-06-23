use anyhow::{Context, Result};
pub struct Cache {
    cache_file: String,
}

const CACHE_DURATION: u64 = 60 * 60 * 6; // 6 hours

impl Cache {
    pub fn new(name: String) -> Cache {
        Cache {
            cache_file: std::env::temp_dir()
                .join(format!("jira-cli-cache-{}.json", name))
                .to_string_lossy()
                .to_string(),
        }
    }
    pub fn save(&self, data: &str) -> Result<()> {
        std::fs::write(&self.cache_file, data)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Option<String>> {
        if !self.exists() || !self.is_valid()? {
            self.clear();
            return Ok(None);
        }

        Ok(std::fs::read_to_string(&self.cache_file).ok())
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.cache_file).exists()
    }

    pub fn is_valid(&self) -> Result<bool> {
        let metadata =
            std::fs::metadata(&self.cache_file).context("Failed to read cache metadata")?;
        let created = metadata
            .created()
            .context("Failed to read cache creation time")?;
        let now = std::time::SystemTime::now();
        let duration = now
            .duration_since(created)
            .context("Failed to calculate duration")?;
        Ok(duration.as_secs() < CACHE_DURATION)
    }

    pub fn clear(&self) {
        if self.exists() {
            std::fs::remove_file(&self.cache_file).expect("Failed to remove cache file");
        }
    }
}
