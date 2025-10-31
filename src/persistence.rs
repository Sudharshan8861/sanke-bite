//! High score persistence using JSON files
//!
//! This module provides functionality to save and load high scores to/from JSON files.
//! Uses serde for serialization to ensure round-trip compatibility.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// A single high score entry with score and optional metadata
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighScore {
    pub score: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}

/// Collection of high scores, keyed by grid size for separate leaderboards
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HighScores {
    #[serde(default)]
    pub scores: BTreeMap<String, Vec<HighScore>>,
}

/// Errors that can occur during persistence operations
#[derive(Debug, PartialEq, Eq)]
pub enum PersistenceError {
    IoError(String),
    SerializationError(String),
    DeserializationError(String),
}

impl From<std::io::Error> for PersistenceError {
    fn from(err: std::io::Error) -> Self {
        PersistenceError::IoError(err.to_string())
    }
}

/// Store for managing high scores with file persistence
pub struct HighScoreStore {
    path: std::path::PathBuf,
    scores: HighScores,
}

impl HighScoreStore {
    /// Create a new store with the given file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, PersistenceError> {
        let path = path.as_ref().to_path_buf();
        let scores = if path.exists() {
            Self::load_from_path(&path)?
        } else {
            HighScores::default()
        };

        Ok(Self { path, scores })
    }

    /// Load high scores from a file path
    fn load_from_path<P: AsRef<Path>>(path: P) -> Result<HighScores, PersistenceError> {
        let contents = fs::read_to_string(path.as_ref())
            .map_err(|e| PersistenceError::IoError(format!("Failed to read file: {}", e)))?;

        serde_json::from_str(&contents)
            .map_err(|e| PersistenceError::DeserializationError(format!("Invalid JSON: {}", e)))
    }

    /// Save high scores to the configured file path
    pub fn save(&self) -> Result<(), PersistenceError> {
        let json = serde_json::to_string_pretty(&self.scores)
            .map_err(|e| PersistenceError::SerializationError(format!("Failed to serialize: {}", e)))?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| PersistenceError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        fs::write(&self.path, json)
            .map_err(|e| PersistenceError::IoError(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    /// Get all high scores for a given grid size key
    pub fn get_scores(&self, grid_key: &str) -> &[HighScore] {
        self.scores.scores.get(grid_key).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get the top N high scores for a given grid size key
    pub fn get_top_scores(&self, grid_key: &str, limit: usize) -> Vec<HighScore> {
        self.get_scores(grid_key)
            .iter()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Add a high score entry for a given grid size key
    pub fn add_score(&mut self, grid_key: String, score: HighScore) {
        let entry = self.scores.scores.entry(grid_key).or_insert_with(Vec::new);
        entry.push(score);
        // Sort in descending order (highest first) and keep top 10
        entry.sort_by(|a, b| b.score.cmp(&a.score));
        entry.truncate(10);
    }

    /// Get the highest score for a given grid size key
    pub fn get_highest_score(&self, grid_key: &str) -> Option<u32> {
        self.get_scores(grid_key).first().map(|hs| hs.score)
    }

    /// Check if a score qualifies as a high score for the given grid size
    pub fn is_high_score(&self, grid_key: &str, score: u32) -> bool {
        match self.get_highest_score(grid_key) {
            None => true, // No existing scores, so any score qualifies
            Some(highest) => score > highest,
        }
    }
}

/// Helper function to create a grid key from grid dimensions
pub fn grid_key(width: i32, height: i32) -> String {
    format!("{}x{}", width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_temp_store() -> (HighScoreStore, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("high_scores.json");
        let store = HighScoreStore::new(&path).unwrap();
        (store, temp_dir)
    }

    #[test]
    fn test_high_score_creation() {
        let score = HighScore {
            score: 100,
            player_name: Some("Player1".to_string()),
            timestamp: Some(1234567890),
        };
        assert_eq!(score.score, 100);
    }

    #[test]
    fn test_store_initialization_new_file() {
        let (store, _temp_dir) = create_temp_store();
        assert_eq!(store.get_scores("10x10").len(), 0);
    }

    #[test]
    fn test_add_and_get_scores() {
        let (mut store, _temp_dir) = create_temp_store();
        let key = "10x10".to_string();
        
        store.add_score(key.clone(), HighScore {
            score: 50,
            player_name: None,
            timestamp: None,
        });
        
        store.add_score(key.clone(), HighScore {
            score: 100,
            player_name: None,
            timestamp: None,
        });

        let scores = store.get_scores(&key);
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].score, 100); // Should be sorted descending
        assert_eq!(scores[1].score, 50);
    }

    #[test]
    fn test_top_scores_limit() {
        let (mut store, _temp_dir) = create_temp_store();
        let key = "10x10".to_string();
        
        for i in 1..=15 {
            store.add_score(key.clone(), HighScore {
                score: i * 10,
                player_name: None,
                timestamp: None,
            });
        }

        let top = store.get_top_scores(&key, 5);
        assert_eq!(top.len(), 5);
        assert_eq!(top[0].score, 150);
    }

    #[test]
    fn test_is_high_score() {
        let (mut store, _temp_dir) = create_temp_store();
        let key = "10x10".to_string();
        
        assert!(store.is_high_score(&key, 10)); // No scores yet
        
        store.add_score(key.clone(), HighScore {
            score: 50,
            player_name: None,
            timestamp: None,
        });
        
        assert!(store.is_high_score(&key, 100));
        assert!(!store.is_high_score(&key, 30));
    }

    #[test]
    fn test_grid_key_helper() {
        assert_eq!(grid_key(10, 10), "10x10");
        assert_eq!(grid_key(20, 15), "20x15");
    }

    #[test]
    fn test_save_and_load_round_trip() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test_scores.json");
        
        // Create store and add scores
        let mut store1 = HighScoreStore::new(&path).unwrap();
        store1.add_score("10x10".to_string(), HighScore {
            score: 100,
            player_name: Some("Alice".to_string()),
            timestamp: Some(1234567890),
        });
        store1.add_score("10x10".to_string(), HighScore {
            score: 75,
            player_name: Some("Bob".to_string()),
            timestamp: Some(1234567891),
        });
        store1.save().unwrap();

        // Load in a new store
        let store2 = HighScoreStore::new(&path).unwrap();
        let scores = store2.get_scores("10x10");
        
        assert_eq!(scores.len(), 2);
        assert_eq!(scores[0].score, 100);
        assert_eq!(scores[0].player_name, Some("Alice".to_string()));
        assert_eq!(scores[1].score, 75);
        assert_eq!(scores[1].player_name, Some("Bob".to_string()));
    }

    #[test]
    fn test_multiple_grid_sizes() {
        let (mut store, _temp_dir) = create_temp_store();
        
        store.add_score("10x10".to_string(), HighScore {
            score: 50,
            player_name: None,
            timestamp: None,
        });
        
        store.add_score("20x20".to_string(), HighScore {
            score: 100,
            player_name: None,
            timestamp: None,
        });

        assert_eq!(store.get_scores("10x10")[0].score, 50);
        assert_eq!(store.get_scores("20x20")[0].score, 100);
        assert_eq!(store.get_scores("15x15").len(), 0);
    }

    #[test]
    fn test_max_ten_scores_per_grid() {
        let (mut store, _temp_dir) = create_temp_store();
        let key = "10x10".to_string();
        
        // Add 15 scores
        for i in 1..=15 {
            store.add_score(key.clone(), HighScore {
                score: i,
                player_name: None,
                timestamp: None,
            });
        }

        let scores = store.get_scores(&key);
        assert_eq!(scores.len(), 10); // Should be truncated to 10
        assert_eq!(scores[0].score, 15); // Highest should be first
        assert_eq!(scores[9].score, 6); // Lowest kept should be 6
    }
}

