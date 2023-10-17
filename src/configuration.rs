use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub gestures: Vec<Gesture>,
}

#[derive(Deserialize)]
pub enum Gesture {
    ThreeFingerSwipe {
        direction: Direction,
        action: String,
    },
    FourFingerSwipe {
        direction: Direction,
        action: String,
    },
    Pinch {
        action: String,
    },
    Spread {
        action: String,
    },
    Hold {
        duration: i64,
        action: String,
    },
}

#[derive(Deserialize, PartialEq)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

pub fn get_configuration() -> Result<Config, serde_yaml::Error> {
    let data = fs::read_to_string("config.yaml").unwrap();
    serde_yaml::from_str(&data)
}
