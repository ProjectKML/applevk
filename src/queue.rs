use std::mem;

use metal::IOCommandQueueDescriptor;

use crate::{Device, Error};

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum IOPriority {
    #[default]
    Normal = 1,
    Low = 2,
    High = 0,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum IOCommandQueueType {
    #[default]
    Concurrent = 0,
    Serial = 1,
}

pub struct IOQueue {
    queue: metal::IOCommandQueue,
    pub(crate) device: Device,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct IOQueueDesc {
    priority: IOPriority,
    ty: IOCommandQueueType,
    max_command_buffers: Option<u64>,
    max_commands_in_flight: Option<u64>,
}

impl IOQueue {
    pub fn new(device: &Device, desc: &IOQueueDesc) -> Result<Self, Error> {
        let descriptor = IOCommandQueueDescriptor::new();
        descriptor.set_priority(unsafe { mem::transmute(desc.priority) });
        descriptor.set_type(unsafe { mem::transmute(desc.ty) });

        if let Some(max_command_buffers) = desc.max_command_buffers {
            descriptor.set_max_command_buffers(max_command_buffers as _);
        }

        if let Some(max_commands_in_flight) = desc.max_commands_in_flight {
            descriptor.set_max_commands_in_flight(max_commands_in_flight as _);
        }

        let queue = device.mtl_device().new_io_command_queue(&descriptor)?;

        Ok(Self {
            queue,
            device: device.clone(),
        })
    }

    #[inline]
    pub fn mtl_io_command_queue(&self) -> &metal::IOCommandQueue {
        &self.queue
    }
}
