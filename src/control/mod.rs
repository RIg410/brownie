use crate::control::controller::Controller;

pub mod controller;
pub mod message;

pub fn make_controller() -> Controller {
    Controller::new()
}
