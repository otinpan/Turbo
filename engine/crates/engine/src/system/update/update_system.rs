use crate::{Input, Registry};
use anyhow::Result;

pub struct UpdateContext<'a> {
    pub registry: &'a mut Registry,
    pub input: &'a Input,
    pub delta_time: f32,
}

pub struct ScheduledUpdateSystem{
    pub name: String,
    pub system: Box<dyn UpdateSystem>,
    pub enabled: bool,
}
pub trait UpdateSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>;
}
