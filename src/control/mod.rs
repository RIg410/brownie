use crate::control::controller::{Controller, Handle};
use crate::context::Context;

pub mod controller;
pub mod message;
pub mod io;

pub fn make_controller() -> (Handle, Controller) {
    Controller::new()
}
