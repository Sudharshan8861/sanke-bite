//! Round-trip tests for high score persistence
//!
//! These tests verify that high scores can be saved to JSON files and loaded back
//! with full fidelity, ensuring serialization/deserialization correctness.

use snake_game::persistence::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_round_trip_empty_scores() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("empty_scores.json");

    // Create and save empty store
    let store1 = HighScoreStore::new(&path).unwrap();
    store1.save().unwrap();

    // Load back
    let store2 = HighScoreStore::new(&path).unwrap();
    assert_eq!(store2.get_scores("10x10").len(), 0);
    // Verify empty state by checking multiple grid keys
    assert_eq!(store2.get_scores("20x20").len(), 0);
}

#[test]
fn test_round_trip_single_score() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("single_score.json");

    // Create, add score, and save
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 42,
            player_name: Some("TestPlayer".to_string()),
            timestamp: Some(1234567890),
        },
    );
    store1.save().unwrap();

    // Load back and verify
    let store2 = HighScoreStore::new(&path).unwrap();
    let scores = store2.get_scores("10x10");
    assert_eq!(scores.len(), 1);
    assert_eq!(scores[0].score, 42);
    assert_eq!(scores[0].player_name, Some("TestPlayer".to_string()));
    assert_eq!(scores[0].timestamp, Some(1234567890));
}

#[test]
fn test_round_trip_multiple_scores() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("multiple_scores.json");

    // Create, add multiple scores, and save
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 100,
            player_name: Some("Alice".to_string()),
            timestamp: Some(1000),
        },
    );
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 50,
            player_name: Some("Bob".to_string()),
            timestamp: Some(2000),
        },
    );
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 75,
            player_name: Some("Charlie".to_string()),
            timestamp: Some(3000),
        },
    );
    store1.save().unwrap();

    // Load back and verify ordering (should be sorted descending)
    let store2 = HighScoreStore::new(&path).unwrap();
    let scores = store2.get_scores("10x10");
    assert_eq!(scores.len(), 3);
    assert_eq!(scores[0].score, 100);
    assert_eq!(scores[0].player_name, Some("Alice".to_string()));
    assert_eq!(scores[1].score, 75);
    assert_eq!(scores[1].player_name, Some("Charlie".to_string()));
    assert_eq!(scores[2].score, 50);
    assert_eq!(scores[2].player_name, Some("Bob".to_string()));
}

#[test]
fn test_round_trip_multiple_grid_sizes() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("multi_grid_scores.json");

    // Create, add scores for different grid sizes, and save
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 50,
            player_name: None,
            timestamp: None,
        },
    );
    store1.add_score(
        "20x20".to_string(),
        HighScore {
            score: 100,
            player_name: None,
            timestamp: None,
        },
    );
    store1.add_score(
        "15x15".to_string(),
        HighScore {
            score: 75,
            player_name: None,
            timestamp: None,
        },
    );
    store1.save().unwrap();

    // Load back and verify all grid sizes are preserved
    let store2 = HighScoreStore::new(&path).unwrap();
    assert_eq!(store2.get_scores("10x10")[0].score, 50);
    assert_eq!(store2.get_scores("20x20")[0].score, 100);
    assert_eq!(store2.get_scores("15x15")[0].score, 75);
}

#[test]
fn test_round_trip_optional_fields() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("optional_fields.json");

    // Create scores with and without optional fields
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 100,
            player_name: None,
            timestamp: None,
        },
    );
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 50,
            player_name: Some("Player".to_string()),
            timestamp: None,
        },
    );
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 25,
            player_name: None,
            timestamp: Some(9999),
        },
    );
    store1.save().unwrap();

    // Load back and verify optional fields are preserved correctly
    let store2 = HighScoreStore::new(&path).unwrap();
    let scores = store2.get_scores("10x10");
    assert_eq!(scores.len(), 3);
    assert_eq!(scores[0].score, 100);
    assert_eq!(scores[0].player_name, None);
    assert_eq!(scores[1].score, 50);
    assert_eq!(scores[1].player_name, Some("Player".to_string()));
    assert_eq!(scores[2].score, 25);
    assert_eq!(scores[2].timestamp, Some(9999));
}

#[test]
fn test_round_trip_incremental_updates() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("incremental.json");

    // First save
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 50,
            player_name: None,
            timestamp: None,
        },
    );
    store1.save().unwrap();

    // Load, add more, save again
    let mut store2 = HighScoreStore::new(&path).unwrap();
    assert_eq!(store2.get_scores("10x10").len(), 1);
    store2.add_score(
        "10x10".to_string(),
        HighScore {
            score: 100,
            player_name: None,
            timestamp: None,
        },
    );
    store2.save().unwrap();

    // Final load and verify both scores exist
    let store3 = HighScoreStore::new(&path).unwrap();
    let scores = store3.get_scores("10x10");
    assert_eq!(scores.len(), 2);
    assert_eq!(scores[0].score, 100);
    assert_eq!(scores[1].score, 50);
}

#[test]
fn test_round_trip_max_scores_truncation() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("truncation.json");

    // Add more than 10 scores
    let mut store1 = HighScoreStore::new(&path).unwrap();
    for i in 1..=15 {
        store1.add_score(
            "10x10".to_string(),
            HighScore {
                score: i * 10,
                player_name: None,
                timestamp: None,
            },
        );
    }
    store1.save().unwrap();

    // Load back and verify only top 10 are kept
    let store2 = HighScoreStore::new(&path).unwrap();
    let scores = store2.get_scores("10x10");
    assert_eq!(scores.len(), 10);
    assert_eq!(scores[0].score, 150); // Highest
    assert_eq!(scores[9].score, 60); // Lowest kept
}

#[test]
fn test_json_file_format_verification() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("format_check.json");

    // Create and save
    let mut store = HighScoreStore::new(&path).unwrap();
    store.add_score(
        "10x10".to_string(),
        HighScore {
            score: 42,
            player_name: Some("Test".to_string()),
            timestamp: Some(1234567890),
        },
    );
    store.save().unwrap();

    // Read raw file and verify it's valid JSON
    let contents = fs::read_to_string(&path).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&contents).unwrap();
    
    // Verify structure
    assert!(parsed.is_object());
    assert!(parsed["scores"].is_object());
    assert!(parsed["scores"]["10x10"].is_array());
    assert_eq!(parsed["scores"]["10x10"][0]["score"], 42);
    assert_eq!(parsed["scores"]["10x10"][0]["player_name"], "Test");
}

#[test]
fn test_load_nonexistent_file_creates_empty_store() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("nonexistent.json");

    // Should not panic when file doesn't exist
    let store = HighScoreStore::new(&path).unwrap();
    assert_eq!(store.get_scores("10x10").len(), 0);
}

#[test]
fn test_round_trip_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("special_chars.json");

    // Test with special characters in player name
    let mut store1 = HighScoreStore::new(&path).unwrap();
    store1.add_score(
        "10x10".to_string(),
        HighScore {
            score: 100,
            player_name: Some("Player & Co. <test>".to_string()),
            timestamp: None,
        },
    );
    store1.save().unwrap();

    // Load back and verify special characters are preserved
    let store2 = HighScoreStore::new(&path).unwrap();
    let scores = store2.get_scores("10x10");
    assert_eq!(scores.len(), 1);
    assert_eq!(
        scores[0].player_name,
        Some("Player & Co. <test>".to_string())
    );
}

