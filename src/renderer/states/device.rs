use super::backend::BackendState;

use gfx_hal::{Adapter, Backend, Graphics, Instance, QueueGroup, Surface};

pub struct DeviceState<B: Backend> {
  pub device: B::Device,
  pub physical_device: B::PhysicalDevice,
  pub queues: QueueGroup<B, Graphics>,
}

impl<B: Backend> DeviceState<B> {
  pub fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self {
    let (device, queues) = adapter
      .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
      .unwrap();

    DeviceState {
      device,
      queues,
      physical_device: adapter.physical_device,
    }
  }
}