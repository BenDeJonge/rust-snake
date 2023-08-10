// External imports.
use crate::dateformat;
use crate::game::Game;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

// Constants.
pub const NUMBER_HIGH_SCORES: usize = 10;
pub const MAX_NAME_LENGTH: usize = 10;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Score {
    player: String,
    score: i32,
    #[serde(with = "dateformat")]
    timestamp: DateTime<Utc>,
}

impl Score {
    pub fn builder() -> ScoreBuilder {
        ScoreBuilder::default()
    }

    pub fn player(&self) -> &str {
        &self.player
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}

#[derive(Default)]
pub struct ScoreBuilder {
    player: String,
    score: i32,
    timestamp: DateTime<Utc>,
}

impl ScoreBuilder {
    pub fn default() -> Self {
        Self {
            player: String::from("default"),
            score: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn player(mut self, player: &str) -> Self {
        self.player = String::from(player);
        self
    }

    pub fn score(mut self, score: i32) -> Self {
        self.score = score;
        self
    }

    pub fn build(self) -> Score {
        Score {
            player: self.player,
            score: self.score,
            timestamp: self.timestamp,
        }
    }
}

/// Parse a vector of scores from the score file in an infallible way.
/// # Arguments
/// * `json: P` - A reference to path-like object, pointing to a score file.
pub fn parse_scores<P: AsRef<Path>>(json: P) -> Vec<Score> {
    let mut data = String::new();
    // Open the file in read-only mode with buffer.
    if let Ok(f) = File::open(json) {
        let mut reader = BufReader::new(f);
        reader.read_to_string(&mut data).unwrap_or_default();
    };
    let mut scores: Vec<Score> = serde_json::from_str(&data).unwrap_or_else(|_| {
        // Generating default map.
        let map: Vec<Score> = Vec::new();
        map
    });
    // Reserve enough space for all the high scores and populate the map with defaults if not enough are read.
    scores
        .try_reserve_exact(NUMBER_HIGH_SCORES)
        .expect("Cannot hold a score database of that size.");
    scores.truncate(NUMBER_HIGH_SCORES);
    if scores.len() < NUMBER_HIGH_SCORES {
        let mut append = vec![ScoreBuilder::default().build(); NUMBER_HIGH_SCORES - scores.len()];
        scores.append(&mut append);
    }
    scores
}

/// Binary search for the first score in the reverse sorted arrays of scores that is lower than the new score.
/// # Arguments
/// * `score: i32` - The score to search for.
/// * `scores: &Vec<Score>` - The reverse sorted vector of Score structs.
/// # Returns
/// * `Option<i32>` - The rank of the score as a i32 or None.
pub fn check_score(score: i32, scores: &Vec<Score>) -> Option<usize> {
    if scores.is_empty() {
        return None;
    }

    let mut low: i32 = 0;
    let mut high: i32 = scores.len() as i32 - 1;

    while low <= high {
        let middle = low + (high - low) / 2;
        if let Some(current) = scores.get(middle as usize) {
            if current.score >= score {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }
    }
    // Clip low to valid indices
    if low >= 0 && low < scores.len() as i32 {
        return Some(low as usize);
    }
    None
}

/// Remove the lowest score and insert the new highscore at the correct rank.
/// # Arguments
/// * `rank: usize` - The rank of the new score.
/// * `score: Score` - The score to be inserted.
/// * `scores: &mut Vec<Score>` - A mutable reference to the current list of highscores.
pub fn update_scores(rank: usize, score: Score, scores: &mut Vec<Score>) {
    if rank <= NUMBER_HIGH_SCORES {
        scores.pop();
        scores.insert(rank, score);
    }
}

pub fn write_scores_to_json<P: AsRef<Path>>(json: P, scores: &Vec<Score>) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string_pretty(scores).unwrap();
    let mut buffer = File::create(json)?;
    buffer.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn write_score(scores: &mut Vec<Score>, name: &str, game: &Game, scores_file: &PathBuf) {
    if let Some(rank) = check_score(game.score(), scores) {
        update_scores(
            rank,
            ScoreBuilder::default()
                .player(name)
                .score(game.score())
                .build(),
            scores,
        );
        match write_scores_to_json(scores_file, scores) {
            Ok(_) => (),
            Err(e) => panic!("Could not write scores: {e:?}"),
        };
    }
}

pub fn create_empty_name() -> String {
    let mut s = String::new();
    s.reserve_exact(MAX_NAME_LENGTH);
    s
}
