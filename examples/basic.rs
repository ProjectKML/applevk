use std::{fs, slice};

use applevk::{IOCommandBuffer, IOFileHandle, IOQueue, IOQueueDesc};
use ash::{ext, vk, Entry};

unsafe fn memory_type_index(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    mut type_bits: u32,
    property_flags: vk::MemoryPropertyFlags,
) -> u32 {
    let memory_properties = instance.get_physical_device_memory_properties(physical_device);
    for i in 0..memory_properties.memory_type_count {
        if (type_bits & 1) == 1 {
            if ((memory_properties.memory_types[i as usize].property_flags & property_flags)
                == property_flags)
            {
                return i;
            }
        }
        type_bits >>= 1;
    }

    panic!("Failed to find suitable memory type")
}

fn main() {
    //Initialize vulkan
    let entry = unsafe { Entry::load_from("./libMoltenVK.dylib") }.unwrap();
    let instance = unsafe {
        entry.create_instance(
            &vk::InstanceCreateInfo::default()
                .application_info(&vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_2)),
            None,
        )
    }
    .unwrap();

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];

    let device = unsafe {
        instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(slice::from_ref(
                    &vk::DeviceQueueCreateInfo::default().queue_priorities(slice::from_ref(&1.0)),
                ))
                .enabled_extension_names(slice::from_ref(&ext::metal_objects::NAME.as_ptr())),
            None,
        )
    }
    .unwrap();

    //Create applevk device
    let apple_vk_device = unsafe { applevk::Device::new(&instance, &device) };
    let handle = IOFileHandle::new(&apple_vk_device, "examples/test.txt").unwrap();

    let size = fs::metadata("examples/test.txt").unwrap().len();

    let queue = IOQueue::new(&apple_vk_device, &IOQueueDesc::default()).unwrap();
    let command_buffer = IOCommandBuffer::new(&queue);

    unsafe {
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size as _)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();

        let requirements = device.get_buffer_memory_requirements(buffer);
        let memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type_index(
                        &instance,
                        physical_device,
                        requirements.memory_type_bits,
                        vk::MemoryPropertyFlags::HOST_VISIBLE
                            | vk::MemoryPropertyFlags::HOST_COHERENT,
                    )),
                None,
            )
            .unwrap();

        device.bind_buffer_memory(buffer, memory, 0).unwrap();

        command_buffer.load_buffer(memory, 0, size as _, &handle, 0);

        command_buffer.commit();
        command_buffer.wait_until_completed();

        let mapped = device
            .map_memory(memory, 0, size, vk::MemoryMapFlags::empty())
            .unwrap();

        let content =
            std::str::from_utf8_unchecked(slice::from_raw_parts(mapped.cast::<u8>(), size as _));

        println!("{}", content);
    }
}
