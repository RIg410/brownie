use crate::control::state::State;
use anyhow::Error;

pub type Call = dyn Fn(&mut State) -> Result<(), Error> + Send + Sync + 'static;

pub enum Message {
    Action(Action),
    System(System),
}

pub enum Action {
    Call(Box<Call>),
}

pub enum System {
    Shutdown,
    Reload(String),
    Store(String),
}
