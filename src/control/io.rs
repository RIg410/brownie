use crate::control::controller::Controller;
use crate::context::Context;
use crate::state::device::gateway::Gateway;
use crate::control::message::Message;
use anyhow::Error;

pub struct IO {
    pub context: Context,
    pub controller: Controller,
}

impl IO {
    pub fn send<M: ToMessage>(gateway: &Gateway, msg: M) -> Result<(), Error> {
        let msg = msg.to_message()?;
        match gateway {
            Gateway::Web { .. } => {

            }
            Gateway::Serial { .. } => {

            }
            Gateway::Usb { .. } => {

            }
        }
        todo!()
    }
}

pub trait ToMessage {
    fn to_message(&self) -> Result<Vec<Parameter>, Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: &'static str,
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    U8(u8),
}