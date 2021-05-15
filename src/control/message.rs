use anyhow::Error;
use crate::context::Context;
use crate::state::State;
use crate::control::io::IO;

pub type Call = dyn Fn(&mut State, &mut IO) -> Result<(), Error> + Send + Sync + 'static;

pub enum Message {
    Action(Action),
    System(System),
}

pub enum Action {
    Call(Box<Call>),
}

pub enum System {
    Shutdown,
    SetContext(Context),
    Reload(String),
    Store(String),
}
