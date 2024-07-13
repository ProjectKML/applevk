mod command_buffer;
mod handle;
mod queue;

use std::{path::Path, sync::Arc};

use ash::vk;
pub use command_buffer::*;
pub use handle::*;
use metal::{foreign_types::ForeignType, CaptureDescriptor, CaptureManager, MTLCaptureDestination};
pub use queue::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed: {0}")]
    Message(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

struct Inner {
    metal_objects: ash::ext::metal_objects::Device,
    device: metal::Device,
}

#[derive(Clone)]
pub struct Device(Arc<Inner>);

impl Device {
    pub unsafe fn new(instance: &ash::Instance, device: &ash::Device) -> Self {
        let metal_objects = ash::ext::metal_objects::Device::new(instance, device);

        let mut device_export_info = vk::ExportMetalDeviceInfoEXT::default();

        let mut metal_objects_info =
            vk::ExportMetalObjectsInfoEXT::default().push_next(&mut device_export_info);

        unsafe {
            (metal_objects.fp().export_metal_objects_ext)(device.handle(), &mut metal_objects_info);

            Self(Arc::new(Inner {
                metal_objects,
                device: metal::Device::from_ptr(device_export_info.mtl_device.cast()),
            }))
        }
    }

    #[inline]
    pub fn mtl_device(&self) -> &metal::Device {
        &self.0.device
    }

    #[inline]
    pub(crate) unsafe fn export_metal_objects(
        &self,
        export_info: &mut vk::ExportMetalObjectsInfoEXT,
    ) {
        let metal_objects = &self.0.metal_objects;
        (metal_objects.fp().export_metal_objects_ext)(metal_objects.device(), export_info);
    }

    pub fn capture<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let manager = CaptureManager::shared();
        let device = self.mtl_device();

        let mut descriptor = CaptureDescriptor::new();
        descriptor.set_capture_device(device);
        descriptor.set_destination(MTLCaptureDestination::GpuTraceDocument);
        descriptor.set_output_url(path);

        manager.start_capture(&descriptor)
    }
}
