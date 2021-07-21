use crate::camera::Camera;
use crate::hittable::HittableList;

pub fn gpu(width: u32, height: u32, scene: &HittableList, camera: &Camera) {
    let entry = maligog::Entry::new().unwrap();
    let required_extensions = maligog::Surface::required_extensions();
    let instance = entry.create_instance(&[], &required_extensions);
    let device = instance
        .enumerate_physical_device()
        .into_iter()
        .find(|p| p.device_type() == maligog::PhysicalDeviceType::DISCRETE_GPU)
        .unwrap()
        .create_device();
    let color_image = device.create_image(
        Some("color image"),
        maligog::Format::R32G32B32A32_SFLOAT,
        width,
        height,
        maligog::ImageUsageFlags::STORAGE
            | maligog::ImageUsageFlags::TRANSFER_DST
            | maligog::ImageUsageFlags::TRANSFER_SRC,
        maligog::MemoryLocation::GpuOnly,
    );
}
