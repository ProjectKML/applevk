use ash::vk;
use metal::{
    foreign_types::ForeignTypeRef, BufferRef, MTLOrigin, MTLSize, SharedEventRef, TextureRef,
};

use crate::{Device, IOFileHandle, IOQueue};

pub struct IOCommandBuffer {
    command_buffer: metal::IOCommandBuffer,
    device: Device,
}

impl IOCommandBuffer {
    pub fn new(queue: &IOQueue) -> Self {
        let command_buffer = queue.mtl_io_command_queue().new_command_buffer();
        Self {
            command_buffer,
            device: queue.device.clone(),
        }
    }

    pub unsafe fn load_buffer(
        &self,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        source_handle: &IOFileHandle,
        source_handle_offset: usize,
    ) {
        let mut buffer_info = vk::ExportMetalBufferInfoEXT::default().memory(memory);
        let mut export_info = vk::ExportMetalObjectsInfoEXT::default().push_next(&mut buffer_info);

        self.device.export_metal_objects(&mut export_info);

        let buffer = BufferRef::from_ptr(buffer_info.mtl_buffer.cast());

        self.command_buffer.load_buffer(
            buffer,
            offset as _,
            size as _,
            source_handle.mtl_io_file_handle(),
            source_handle_offset as _,
        );
    }

    pub unsafe fn load_image(
        &self,
        image: vk::Image,
        slice: usize,
        level: usize,
        size: vk::Extent3D,
        source_bytes_per_row: usize,
        source_bytes_per_image: usize,
        destination_origin: vk::Extent3D,
        source_handle: &IOFileHandle,
        source_handle_offset: usize,
    ) {
        let mut texture_info = vk::ExportMetalTextureInfoEXT::default().image(image);
        let mut export_info = vk::ExportMetalObjectsInfoEXT::default().push_next(&mut texture_info);

        self.device.export_metal_objects(&mut export_info);

        let texture = TextureRef::from_ptr(texture_info.mtl_texture.cast());

        let size = MTLSize {
            width: size.width as _,
            height: size.height as _,
            depth: size.depth as _,
        };

        let destination_origin = MTLOrigin {
            x: destination_origin.width as _,
            y: destination_origin.height as _,
            z: destination_origin.depth as _,
        };

        self.command_buffer.load_texture(
            texture,
            slice as _,
            level as _,
            size,
            source_bytes_per_row as _,
            source_bytes_per_image as _,
            destination_origin,
            source_handle.mtl_io_file_handle(),
            source_handle_offset as _,
        );
    }

    #[inline]
    pub fn commit(&self) {
        self.command_buffer.commit();
    }

    #[inline]
    pub fn wait_until_completed(&self) {
        self.command_buffer.wait_until_completed();
    }

    pub unsafe fn signal_semaphore(&self, semaphore: vk::Semaphore, value: u64) {
        let mut shared_event_info =
            vk::ExportMetalSharedEventInfoEXT::default().semaphore(semaphore);

        let mut export_info =
            vk::ExportMetalObjectsInfoEXT::default().push_next(&mut shared_event_info);

        self.device.export_metal_objects(&mut export_info);

        let shared_event = SharedEventRef::from_ptr(shared_event_info.mtl_shared_event.cast());

        self.command_buffer.signal_event(shared_event, value);
    }

    pub unsafe fn signal_event(&self, event: vk::Event, value: u64) {
        let mut shared_event_info = vk::ExportMetalSharedEventInfoEXT::default().event(event);

        let mut export_info =
            vk::ExportMetalObjectsInfoEXT::default().push_next(&mut shared_event_info);

        self.device.export_metal_objects(&mut export_info);

        let shared_event = SharedEventRef::from_ptr(shared_event_info.mtl_shared_event.cast());

        self.command_buffer.signal_event(shared_event, value);
    }

    #[inline]
    pub fn mtl_io_command_buffer(&self) -> &metal::IOCommandBuffer {
        &self.command_buffer
    }
}
