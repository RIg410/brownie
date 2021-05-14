use crate::control::message::Action;
use anyhow::Result;

pub struct State {}

impl State {
    pub fn load(config: String) -> State {
        todo!()
    }

    pub fn store(&self, config: String) {}

    pub fn perform(&mut self, action: Action) -> Result<()> {
        todo!()
    }
}
