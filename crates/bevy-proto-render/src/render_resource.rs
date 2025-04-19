use bevy_derive::Deref;
use bevy_ecs::prelude::*;
use frame_graph::prelude::Device;

#[derive(Resource, Clone, Deref)]
pub struct RenderDevice(Device);

impl RenderDevice {
    pub fn new(device: Device) -> Self {
        Self(device)
    }
}
