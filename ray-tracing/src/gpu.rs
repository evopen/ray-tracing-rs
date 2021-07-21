use maligog::vk;

use crate::camera::Camera;
use crate::hittable::HittableList;

pub fn spirv_builder(path_to_crate: &str) -> spirv_builder::SpirvBuilder {
    spirv_builder::SpirvBuilder::new(path_to_crate, "spirv-unknown-vulkan1.2")
        .capability(spirv_builder::Capability::RayTracingKHR)
        .capability(spirv_builder::Capability::ImageQuery)
        .capability(spirv_builder::Capability::Int8)
        .capability(spirv_builder::Capability::Int16)
        .capability(spirv_builder::Capability::Int64)
        .capability(spirv_builder::Capability::RuntimeDescriptorArray)
        .capability(spirv_builder::Capability::Linkage)
        .extension("SPV_KHR_ray_tracing")
        .extension("SPV_EXT_descriptor_indexing")
        .name_variables(false)
        .scalar_block_layout(true)
        .print_metadata(spirv_builder::MetadataPrintout::None)
}

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
    let descriptor_pool = device.create_descriptor_pool(
        &[
            maligog::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(2)
                .build(),
            maligog::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(2)
                .build(),
            maligog::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::SAMPLER)
                .descriptor_count(2)
                .build(),
            maligog::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .descriptor_count(1)
                .build(),
        ],
        10,
    );
    let as_descriptor_set_layout = device.create_descriptor_set_layout(
        Some("ray tracing as"),
        &[
            maligog::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: maligog::DescriptorType::AccelerationStructure,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 2,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 3,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 4,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 5,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 6,
                descriptor_type: maligog::DescriptorType::Sampler(None),
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 500,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 7,
                descriptor_type: maligog::DescriptorType::SampledImage,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 500,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 8,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 9,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
            maligog::DescriptorSetLayoutBinding {
                binding: 10,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
        ],
    );
    let pipeline_layout =
        device.create_pipeline_layout(Some("main"), &[&as_descriptor_set_layout], &[]);
    let spirv = std::fs::read(
        spirv_builder("./shaders/shader")
            .build()
            .unwrap()
            .module
            .unwrap_single(),
    )
    .unwrap();
    let module = device.create_shader_module(spirv);
    let pipeline = device.create_ray_tracing_pipeline(
        Some("main"),
        &pipeline_layout,
        &maligog::ShaderStage::new(&module, maligog::ShaderStageFlags::RAYGEN_KHR, "main"),
        &[&maligog::ShaderStage::new(
            &module,
            maligog::ShaderStageFlags::MISS_KHR,
            "miss",
        )],
        &[&maligog::ProceduralHitGroup::new(
            &maligog::ShaderStage::new(
                &module,
                maligog::ShaderStageFlags::CLOSEST_HIT_KHR,
                "closest_hit",
            ),
            &maligog::ShaderStage::new(
                &module,
                maligog::ShaderStageFlags::INTERSECTION_KHR,
                "sphere_intersection",
            ),
            None,
        )],
        31,
    );
}
