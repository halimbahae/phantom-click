use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreEntry {
    pub cps: f64,
    pub clicks: u64,
    pub duration_secs: u64,
    pub animal: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub best: HashMap<u64, Vec<ScoreEntry>>,
}

impl Scores {
    pub fn new() -> Self {
        Self {
            best: HashMap::new(),
        }
    }

    pub fn load() -> Self {
        let path = Self::path();
        fs::read_to_string(&path)
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_else(Self::new)
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, content);
        }
    }

    pub fn add_score(&mut self, entry: ScoreEntry) {
        let duration = entry.duration_secs;
        let entries = self.best.entry(duration).or_default();
        entries.push(entry);
        entries.sort_by(|a, b| b.cps.partial_cmp(&a.cps).unwrap_or(std::cmp::Ordering::Equal));
        entries.truncate(5);
        self.save();
    }

    pub fn best_for(&self, duration_secs: u64) -> Vec<&ScoreEntry> {
        self.best
            .get(&duration_secs)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    fn path() -> PathBuf {
        let home =
            std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(&home);
        path.push(".config");
        path.push("phantom-click");
        path.push("scores.json");
        path
    }
}
