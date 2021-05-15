use crate::state::device::light::Light;

pub mod light;
pub mod gateway;

#[derive(Debug, Serialize, Deserialize)]
pub enum Device {
    Light(Light),
}