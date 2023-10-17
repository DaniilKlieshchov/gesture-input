use crate::configuration::{Direction, Gesture};
use input::event::gesture::*;
use std::{
    io,
    process::{Command, Output},
};
use timer::{Timer, Guard};
enum GestureState {
    None,
    SwipeStarted,
    SwipeUpdated {
        accumulated_dx: f64,
        accumulated_dy: f64,
    },
    HoldStarted(Vec<(Guard, Timer)>),
    PinchStarted,
    PinchUpdated {
        scale: f64,
    },
}

pub struct GestureEventHandler {
    state: GestureState,
    gestures: Vec<Gesture>,

}

impl GestureEventHandler {
    pub fn new(gestures: Vec<Gesture>) -> Self {
        Self {
            state: GestureState::None,
            gestures,
        }
    }

    pub fn process_event(&mut self, event: GestureEvent) {
        match event {
            GestureEvent::Swipe(swipe_event) => self.handle_swipe_event(&swipe_event),
            GestureEvent::Pinch(pinch_event) => self.handle_pinch_event(&pinch_event),
            GestureEvent::Hold(hold_event) => self.handle_hold_event(&hold_event),
            _ => todo!(),
        }
    }

    fn handle_swipe_event(&mut self, event: &GestureSwipeEvent) {
        match event {
            GestureSwipeEvent::Begin(_begin_event) => {
                self.state = GestureState::SwipeStarted;
            }
            GestureSwipeEvent::Update(update_event) => {
                if let GestureState::SwipeUpdated {
                    accumulated_dx,
                    accumulated_dy,
                } = self.state
                {
                    self.state = GestureState::SwipeUpdated {
                        accumulated_dx: accumulated_dx + update_event.dx(),
                        accumulated_dy: accumulated_dy + update_event.dy(),
                    }
                } else if let GestureState::SwipeStarted = self.state {
                    self.state = GestureState::SwipeUpdated {
                        accumulated_dx: update_event.dx(),
                        accumulated_dy: update_event.dy(),
                    }
                }
            }
            GestureSwipeEvent::End(end_event) => {
                let read_direction: Direction;
                let finger_count: i32 = end_event.finger_count();
                if let GestureState::SwipeUpdated {
                    accumulated_dx,
                    accumulated_dy,
                } = self.state
                {
                    if accumulated_dy.abs() >= accumulated_dx.abs() {
                        if accumulated_dy >= 0.0 {
                            read_direction = Direction::Down;
                        } else {
                            read_direction = Direction::Up;
                        }
                    } else {
                        if accumulated_dx >= 0.0 {
                            read_direction = Direction::Right;
                        } else {
                            read_direction = Direction::Left;
                        }
                    }
                } else {
                    read_direction = Direction::Down;
                }
                for event_config in &self.gestures {
                    if let Gesture::FourFingerSwipe { direction, action } = event_config {
                        if &read_direction == direction && finger_count == 4 {
                            let command = Self::execute_command(action);
                            println!("{}", String::from_utf8(command.unwrap().stdout).unwrap())
                        }
                    } else if let Gesture::ThreeFingerSwipe { direction, action } = event_config {
                        if &read_direction == direction && finger_count == 3 {
                            let command = Self::execute_command(action);
                            println!("{}", String::from_utf8(command.unwrap().stdout).unwrap())
                        }
                    }
                }

                self.state = GestureState::None;
            }
            _ => todo!(),
        }
    }

    fn handle_pinch_event(&mut self, event: &GesturePinchEvent) {
        match event {
            GesturePinchEvent::Begin(_begin_event) => {
                self.state = GestureState::PinchStarted;
            }
            GesturePinchEvent::Update(update_event) => {
                self.state = GestureState::PinchUpdated {
                    scale: update_event.scale(),
                }
            }
            GesturePinchEvent::End(_end_event) => {
                let scale;
                if let GestureState::PinchUpdated { scale: s } = self.state {
                    scale = s;
                } else {
                    scale = 1.0;
                }

                for event_config in &self.gestures {
                    if scale < 1.0 {
                        if let Gesture::Pinch { action } = event_config {
                            let command = Self::execute_command(action);
                            println!("{}", String::from_utf8(command.unwrap().stdout).unwrap())
                        }
                    } else {
                        if let Gesture::Spread { action } = event_config {
                            let command = Self::execute_command(action);
                            println!("{}", String::from_utf8(command.unwrap().stdout).unwrap())
                        }
                    }
                }

                self.state = GestureState::None;
            }
            _ => todo!(),
        }
    }

    fn handle_hold_event(&mut self, event: &GestureHoldEvent) {
        match event {
            GestureHoldEvent::Begin(_begin_event) => {
                let mut hold_timers = vec![];
                for event_config in &self.gestures {
                    if let Gesture::Hold { duration, action } = event_config {

                        let timer = Timer::new();
                        let command = action.clone();
                        let guard = timer.schedule_with_delay(
                            chrono::Duration::milliseconds(*duration),
                            move || {
                                let command = Self::execute_command(&command);
                                println!("{}", String::from_utf8(command.unwrap().stdout).unwrap())
                            },
                        );

                        hold_timers.push((guard, timer)); 
                    }
                }
                self.state = GestureState::HoldStarted(hold_timers);
            }

            GestureHoldEvent::End(_end_event) => {
                self.state = GestureState::None;
            }

            _ => todo!(),
        }
    }

    fn execute_command(command: &str) -> io::Result<Output> {
        Command::new("sh").arg("-c").arg(command).output()
    }
}


