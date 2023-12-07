use std::{fs, path::Path};

use serde::Deserialize;

use crate::gesture_listener::{
    FourFingerSwipeListener, GestureListener, HoldListener, PinchListener, SpreadListener,
    ThreeFingerSwipeGradualListener, ThreeFingerSwipeListener,
};

#[derive(Deserialize)]
pub struct Config {
    pub gestures: Vec<Gesture>,
}

#[derive(Deserialize)]
pub enum Gesture {
    ThreeFingerSwipeGradual {
        direction: Direction,
        action: String,
    },
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

impl Gesture {
    pub fn listen(self) -> Box<dyn GestureListener> {
        match self {
            Gesture::ThreeFingerSwipeGradual { direction, action } => {
                Box::new(ThreeFingerSwipeGradualListener::new(direction, action))
            }
            Gesture::ThreeFingerSwipe { direction, action } => {
                Box::new(ThreeFingerSwipeListener::new(direction, action))
            }
            Gesture::FourFingerSwipe { direction, action } => {
                Box::new(FourFingerSwipeListener::new(direction, action))
            }
            Gesture::Pinch { action } => Box::new(PinchListener::new(action)),
            Gesture::Spread { action } => Box::new(SpreadListener::new(action)),
            Gesture::Hold { duration, action } => Box::new(HoldListener::new(action, duration)),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

pub fn get_configuration(path: &Path) -> Result<Config, serde_yaml::Error> {
    let data = fs::read_to_string(path).expect("Failed to read configuration");
    serde_yaml::from_str(&data)
}
