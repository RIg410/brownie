use crate::state::device::gateway::Gateway;
use anyhow::Error;
use crate::control::io::{IO, ToMessage, Parameter};

const MIN: u8 = 0;
const MAX: u8 = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    is_on: bool,
    brightness: u8,
}

impl Light {
    pub fn new(is_on: bool, brightness: u8) -> Light {
        Light { is_on, brightness }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        if brightness >= 100 {
            self.brightness = 100;
        } else {
            self.brightness = brightness;
        }
    }

    pub fn set_is_on(&mut self, is_on: bool) {
        self.is_on = is_on;
    }
}

impl ToMessage for Light {
    fn to_message(&self) -> Result<Vec<Parameter>, Error> {

        todo!()
    }
}