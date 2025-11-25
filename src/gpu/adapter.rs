use wgpu::{
    Adapter, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions,
};
#[derive(Debug)]
pub struct GpuAdapter {
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub instance: Instance,
}

impl GpuAdapter {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("concentrict-gpu-device"),
                ..Default::default()
            })
            .await?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
