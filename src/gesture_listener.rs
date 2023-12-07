use std::process::Command;

use crate::configuration::Direction;
use input::event::gesture::{
    GestureEventCoordinates, GestureEventTrait, GestureHoldEvent, GesturePinchEvent,
    GesturePinchEventTrait, GestureSwipeEvent,
};
use input::event::GestureEvent;
use input::event::GestureEvent::Hold;
use input::event::GestureEvent::Pinch;
use input::event::GestureEvent::Swipe;
use timer::{Guard, Timer};

pub trait GestureListener {
    fn fire(&mut self, event: &GestureEvent);
}

pub struct ThreeFingerSwipeListener {
    direction: Direction,
    action: String,

    accumulated_dx: f64,
    accumulated_dy: f64,
}

impl ThreeFingerSwipeListener {
    pub fn new(direction: Direction, action: String) -> Self {
        Self {
            direction,
            action,
            accumulated_dx: 0.0,
            accumulated_dy: 0.0,
        }
    }
}

impl GestureListener for ThreeFingerSwipeListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Swipe(swipe_event) = event {
            match swipe_event {
                GestureSwipeEvent::Begin(_begin_event) => {
                    self.accumulated_dx = 0.0;
                    self.accumulated_dy = 0.0;
                }
                GestureSwipeEvent::Update(update_event) => {
                    self.accumulated_dx += update_event.dx();
                    self.accumulated_dy += update_event.dy();
                }
                GestureSwipeEvent::End(end_event) => {
                    let direction: Direction;
                    let finger_count: i32 = end_event.finger_count();
                    if self.accumulated_dy.abs() >= self.accumulated_dx.abs() {
                        if self.accumulated_dy >= 0.0 {
                            direction = Direction::Down;
                        } else {
                            direction = Direction::Up;
                        }
                    } else {
                        if self.accumulated_dx >= 0.0 {
                            direction = Direction::Right;
                        } else {
                            direction = Direction::Left;
                        }
                    }

                    if self.direction == direction && finger_count == 3 {
                        let command = Command::new("sh")
                            .arg("-c")
                            .arg(self.action.as_str())
                            .output();

                        match command {
                            Ok(output) => {
                                tracing::info!(?output, "Command output")
                            }
                            Err(error) => tracing::error!(?error, "Bash command threw an error"),
                        }
                    }
                }
                _ => tracing::warn!(?event, "Got unsupported event"),
            }
        }
    }
}

pub struct ThreeFingerSwipeGradualListener {
    direction: Direction,
    action: String,

    count: u32,
}

impl ThreeFingerSwipeGradualListener {
    pub fn new(direction: Direction, action: String) -> Self {
        Self {
            direction,
            action,

            count: 0,
        }
    }
}

impl GestureListener for ThreeFingerSwipeGradualListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Swipe(swipe_event) = event {
            if let GestureSwipeEvent::Update(update_event) = swipe_event {
                let direction: Direction;
                let finger_count: i32 = update_event.finger_count();
                if update_event.dy().abs() >= update_event.dx().abs() {
                    if update_event.dy() >= 0.0 {
                        direction = Direction::Down;
                    } else {
                        direction = Direction::Up;
                    }
                } else {
                    if update_event.dx() >= 0.0 {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Left;
                    }
                }

                self.count += 1;

                if self.direction == direction && finger_count == 3 && self.count % 3 == 0 {
                    let command = Command::new("sh")
                        .arg("-c")
                        .arg(self.action.as_str())
                        .output();

                    match command {
                        Ok(output) => {
                            tracing::info!(?output, "Command output")
                        }
                        Err(error) => tracing::error!(?error, "Bash command threw an error"),
                    }
                }
            }
        }
    }
}

pub struct FourFingerSwipeListener {
    direction: Direction,
    action: String,

    accumulated_dx: f64,
    accumulated_dy: f64,
}

impl FourFingerSwipeListener {
    pub fn new(direction: Direction, action: String) -> Self {
        Self {
            direction,
            action,
            accumulated_dx: 0.0,
            accumulated_dy: 0.0,
        }
    }
}

impl GestureListener for FourFingerSwipeListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Swipe(swipe_event) = event {
            match swipe_event {
                GestureSwipeEvent::Begin(_begin_event) => {
                    self.accumulated_dx = 0.0;
                    self.accumulated_dy = 0.0;
                }
                GestureSwipeEvent::Update(update_event) => {
                    self.accumulated_dx += update_event.dx();
                    self.accumulated_dy += update_event.dy();
                }
                GestureSwipeEvent::End(end_event) => {
                    let direction: Direction;
                    let finger_count: i32 = end_event.finger_count();
                    if self.accumulated_dy.abs() >= self.accumulated_dx.abs() {
                        if self.accumulated_dy >= 0.0 {
                            direction = Direction::Down;
                        } else {
                            direction = Direction::Up;
                        }
                    } else {
                        if self.accumulated_dx >= 0.0 {
                            direction = Direction::Right;
                        } else {
                            direction = Direction::Left;
                        }
                    }

                    if self.direction == direction && finger_count == 4 {
                        let command = Command::new("sh")
                            .arg("-c")
                            .arg(self.action.as_str())
                            .output();
                        match command {
                            Ok(output) => {
                                tracing::info!(?output, "Command output")
                            }
                            Err(error) => tracing::error!(?error, "Bash command threw an error"),
                        }
                    }
                }
                _ => tracing::warn!(?event, "Got unsupported event"),
            }
        }
    }
}

pub struct PinchListener {
    action: String,

    scale: f64,
}

impl PinchListener {
    pub fn new(action: String) -> Self {
        Self { action, scale: 1.0 }
    }
}

impl GestureListener for PinchListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Pinch(pinch_event) = event {
            match pinch_event {
                GesturePinchEvent::Begin(_begin_event) => {
                    self.scale = 1.0;
                }
                GesturePinchEvent::Update(update_event) => {
                    self.scale = update_event.scale();
                }
                GesturePinchEvent::End(_end_event) => {
                    if self.scale < 1.0 {
                        let command = Command::new("sh")
                            .arg("-c")
                            .arg(self.action.as_str())
                            .output();
                        match command {
                            Ok(output) => {
                                tracing::info!(?output, "Command output")
                            }
                            Err(error) => tracing::error!(?error, "Bash command threw an error"),
                        }
                    }
                }
                _ => tracing::warn!(?event, "Got unsupported event"),
            }
        }
    }
}

pub struct SpreadListener {
    action: String,

    scale: f64,
}

impl SpreadListener {
    pub fn new(action: String) -> Self {
        Self { action, scale: 1.0 }
    }
}

impl GestureListener for SpreadListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Pinch(pinch_event) = event {
            match pinch_event {
                GesturePinchEvent::Begin(_begin_event) => {
                    self.scale = 1.0;
                }
                GesturePinchEvent::Update(update_event) => {
                    self.scale = update_event.scale();
                }
                GesturePinchEvent::End(_end_event) => {
                    if self.scale > 1.0 {
                        let command = Command::new("sh")
                            .arg("-c")
                            .arg(self.action.as_str())
                            .output();
                        match command {
                            Ok(output) => {
                                tracing::info!(?output, "Command output")
                            }
                            Err(error) => tracing::error!(?error, "Bash command threw an error"),
                        }
                    }
                }
                _ => tracing::warn!(?event, "Got unsupported event"),
            }
        }
    }
}

pub struct HoldListener {
    action: String,
    duration: i64,

    timer: Option<Timer>,
    guard: Option<Guard>,
}

impl HoldListener {
    pub fn new(action: String, duration: i64) -> Self {
        Self {
            action,
            duration,
            timer: Option::None,
            guard: Option::None,
        }
    }
}

impl GestureListener for HoldListener {
    fn fire(&mut self, event: &GestureEvent) {
        if let Hold(hold_event) = event {
            match hold_event {
                GestureHoldEvent::Begin(_begin_event) => {
                    let timer = Timer::new();
                    let command = self.action.clone();
                    let guard = timer.schedule_with_delay(
                        chrono::Duration::milliseconds(self.duration),
                        move || {
                            let command =
                                Command::new("sh").arg("-c").arg(command.as_str()).output();
                            match command {
                                Ok(output) => {
                                    tracing::info!(?output, "Command output")
                                }
                                Err(error) => {
                                    tracing::error!(?error, "Bash command threw an error")
                                }
                            }
                        },
                    );

                    self.timer = Option::from(timer);
                    self.guard = Option::from(guard);
                }

                GestureHoldEvent::End(_end_event) => {
                    self.timer = Option::None;
                    self.guard = Option::None;
                }

                _ => tracing::warn!(?event, "Got unsupported event"),
            }
        }
    }
}
