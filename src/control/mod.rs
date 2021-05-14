use crate::control::controller::Controller;

pub mod controller;
pub mod message;
pub mod state;

pub fn make_controller() -> Controller {
    Controller::new()
}
