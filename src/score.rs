use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::dateformat;
use crate::game::Game;

pub const NUMBER_HIGH_SCORES: i32 = 10;
const MAX_NAME_LENGTH: i32 = 10;

#[derive(Debug, Deserialize, Serialize)]
pub struct HighScore {
    #[serde(flatten)]
    score: HashMap<i32, Score>,
}

#[derive(Debug, Deserialize, Serialize)]
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

pub fn parse_scores<P: AsRef<Path>>(json: P) -> Result<HashMap<i32, Score>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(json)?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap_or_default();
    let mut scores: HashMap<i32, Score> = serde_json::from_str(&data).unwrap_or_else(|_| {
        // Generating default map.
        let map: HashMap<i32, Score> = HashMap::new();
        map
    });
    // Reserve enough space for all the high scores and populate the map with defaults if not enough are read.
    scores
        .try_reserve(NUMBER_HIGH_SCORES as usize)
        .expect("Cannot hold a score database of that size.");
    for i in 1..NUMBER_HIGH_SCORES + 1 {
        if scores.get(&i).is_none() {
            scores.insert(i, ScoreBuilder::default().build());
        }
    }
    Ok(scores)
}

pub fn check_score(score: i32, scores: &HashMap<i32, Score>) -> Option<i32> {
    (1..NUMBER_HIGH_SCORES + 1).find(|&rank| score > scores.get(&rank).unwrap().score)
}

pub fn update_scores(rank: i32, score: Score, scores: &mut HashMap<i32, Score>) {
    scores.insert(rank, score);
}

pub fn write_scores_to_json<P: AsRef<Path>>(
    json: P,
    scores: &HashMap<i32, Score>,
) -> std::io::Result<()> {
    let serialized: String = serde_json::to_string_pretty(scores).unwrap();
    let mut buffer = File::create(json)?;
    buffer.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn write_score(
    scores: &mut HashMap<i32, Score>,
    name: &str,
    game: &mut Game,
    scores_file: &PathBuf,
) {
    if let Some(rank) = check_score(game.score(), scores) {
        game.score_written = true;
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

pub fn ask_name() -> String {
    String::from("nice")
}
