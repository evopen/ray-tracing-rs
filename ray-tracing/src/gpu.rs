use std::io::{stdin, Read};

use maligog::vk;
use maplit::btreemap;

use crate::camera::Camera;
use crate::hittable::HittableList;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct Sphere {
    pub center: glam::Vec3,
    pub radius: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
pub struct CameraInfo {
    view_inv: glam::Mat4,
    proj_inv: glam::Mat4,
}

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

pub fn gpu(width: u32, height: u32, scene: &mut HittableList, camera: &Camera) {
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    let entry = maligog::Entry::new().unwrap();
    let required_extensions = maligog::Surface::required_extensions();
    let instance = entry.create_instance(&[], &required_extensions);
    let surface = instance.create_surface(&window);

    let device = instance
        .enumerate_physical_device()
        .into_iter()
        .find(|p| p.device_type() == maligog::PhysicalDeviceType::DISCRETE_GPU)
        .unwrap()
        .create_device();
    let swapchain = device.create_swapchain(surface, maligog::PresentModeKHR::FIFO);

    dbg!(&width);
    dbg!(&height);
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
            maligog::DescriptorSetLayoutBinding {
                binding: 11,
                descriptor_type: maligog::DescriptorType::StorageBuffer,
                stage_flags: maligog::ShaderStageFlags::ALL,
                descriptor_count: 1,
                variable_count: false,
            },
        ],
    );
    let image_descriptor_set_layout = device.create_descriptor_set_layout(
        Some("ray tracing image"),
        &[maligog::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: maligog::DescriptorType::StorageImage,
            stage_flags: maligog::ShaderStageFlags::ALL,
            descriptor_count: 1,
            variable_count: false,
        }],
    );
    let pipeline_layout = device.create_pipeline_layout(
        Some("main"),
        &[&as_descriptor_set_layout, &image_descriptor_set_layout],
        &[maligog::PushConstantRange::builder()
            .offset(0)
            .size(std::mem::size_of::<CameraInfo>() as u32)
            .stage_flags(
                maligog::ShaderStageFlags::RAYGEN_KHR | maligog::ShaderStageFlags::CLOSEST_HIT_KHR,
            )
            .build()],
    );
    let spirv = std::fs::read(
        spirv_builder("./shaders/shader")
            .build()
            .unwrap()
            .module
            .unwrap_single(),
    )
    .unwrap();
    let module = device.create_shader_module(spirv);
    let procedural_hit_group = maligog::ProceduralHitGroup::new(
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
    );
    let pipeline = device.create_ray_tracing_pipeline(
        Some("main"),
        &pipeline_layout,
        &maligog::ShaderStage::new(&module, maligog::ShaderStageFlags::RAYGEN_KHR, "main"),
        &[&maligog::ShaderStage::new(
            &module,
            maligog::ShaderStageFlags::MISS_KHR,
            "miss",
        )],
        &[&procedural_hit_group],
        31,
    );
    let top = scene.build_tlas(&device);

    let image_descriptor_set = device.create_descriptor_set(
        Some("image descriptor set"),
        &descriptor_pool,
        &image_descriptor_set_layout,
        btreemap! {
            0 => maligog::DescriptorUpdate::Image(vec![color_image.create_view()]),
        },
    );

    let mut cmd_buf =
        device.create_command_buffer(Some("main"), device.graphics_queue_family_index());
    let mut hit_groups: Vec<u32> = Vec::new();
    for i in 0..12345 {
        hit_groups.push(0);
    }
    dbg!(&color_image.width());
    dbg!(&color_image.height());
    let shader_binding_tables = pipeline.create_shader_binding_tables(&hit_groups);
    dbg!(&color_image.linear_size());
    let color_image_buffer = device.create_buffer(
        Some("depth image buffer"),
        color_image.linear_size(),
        maligog::BufferUsageFlags::empty(),
        maligog::MemoryLocation::GpuToCpu,
    );

    let sphere1 = Sphere {
        center: glam::Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
    };
    let sphere2 = Sphere {
        center: glam::Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
    };
    let spheres = vec![sphere1, sphere2];
    let spheres_buffer = device.create_buffer_init(
        None,
        bytemuck::cast_slice(&spheres),
        maligog::BufferUsageFlags::STORAGE_BUFFER,
        maligog::MemoryLocation::GpuOnly,
    );

    let as_descriptor_set = device.create_descriptor_set(
        Some("as descriptor set"),
        &descriptor_pool,
        &as_descriptor_set_layout,
        btreemap! {
            0 => maligog::DescriptorUpdate::AccelerationStructure(vec![top]),
            11 => maligog::DescriptorUpdate::Buffer(vec![maligog::BufferView{buffer:spheres_buffer,offset:0}]),
        },
    );

    let mut camera_info = CameraInfo {
        view_inv: glam::Mat4::look_at_lh(camera.origin, camera.origin + camera.front, camera.up)
            .inverse(),
        proj_inv: glam::Mat4::perspective_lh(20.0, 1.0, 0.001, 10000.0).inverse(),
    };
    cmd_buf.encode(|rec| {
        rec.bind_ray_tracing_pipeline(&pipeline, |rec| {
            rec.bind_descriptor_sets(vec![&as_descriptor_set, &image_descriptor_set], 0);
            rec.push_constants(
                maligog::ShaderStageFlags::RAYGEN_KHR | maligog::ShaderStageFlags::CLOSEST_HIT_KHR,
                &bytemuck::cast_slice(&[camera_info]),
            );
            rec.trace_ray(
                &shader_binding_tables.ray_gen_table(),
                &shader_binding_tables.miss_table(),
                &shader_binding_tables.hit_table(),
                &shader_binding_tables.callable_table(),
                color_image.width(),
                color_image.height(),
                31,
            );
            rec.copy_image_to_buffer(
                &color_image,
                maligog::ImageLayout::TRANSFER_SRC_OPTIMAL,
                &color_image_buffer,
            );
        });
    });
    device.graphics_queue().submit_blocking(&[cmd_buf]);

    let data = color_image_buffer
        .lock_memory()
        .unwrap()
        .mapped_slice()
        .unwrap()
        .to_owned();

    let pixels: &[f32] = bytemuck::cast_slice(&data);

    let width = color_image.width() as usize;
    let height = color_image.height() as usize;
    exr::prelude::write_rgb_file("result.exr", width, height, |x, y| {
        (
            // generate (or lookup in your own image) an f32 rgb color for each of the 2048x2048 pixels
            pixels[y * width * 4 + x * 4],     // red
            pixels[y * width * 4 + x * 4 + 1], // green
            pixels[y * width * 4 + x * 4 + 2], // blue
        )
    })
    .unwrap();

    loop {
        if let Ok(index) = swapchain.acquire_next_image() {
            let frame = swapchain.get_image(index);

            let mut cmd_buf =
                device.create_command_buffer(Some("main"), device.graphics_queue_family_index());
            let mut hit_groups: Vec<u32> = Vec::new();
            for i in 0..12345 {
                hit_groups.push(0);
            }
            let shader_binding_tables = pipeline.create_shader_binding_tables(&hit_groups);
            let color_image_buffer = device.create_buffer(
                Some("depth image buffer"),
                color_image.linear_size(),
                maligog::BufferUsageFlags::empty(),
                maligog::MemoryLocation::GpuToCpu,
            );

            let mut camera_info = CameraInfo {
                view_inv: glam::Mat4::look_at_lh(
                    camera.origin,
                    camera.origin + camera.front,
                    camera.up,
                )
                .inverse(),
                proj_inv: glam::Mat4::perspective_lh(40.0, 1.0, 0.001, 10000.0).inverse(),
            };
            cmd_buf.encode(|rec| {
                rec.bind_ray_tracing_pipeline(&pipeline, |rec| {
                    rec.bind_descriptor_sets(vec![&as_descriptor_set, &image_descriptor_set], 0);
                    rec.push_constants(
                        maligog::ShaderStageFlags::RAYGEN_KHR
                            | maligog::ShaderStageFlags::CLOSEST_HIT_KHR,
                        &bytemuck::cast_slice(&[camera_info]),
                    );
                    rec.trace_ray(
                        &shader_binding_tables.ray_gen_table(),
                        &shader_binding_tables.miss_table(),
                        &shader_binding_tables.hit_table(),
                        &shader_binding_tables.callable_table(),
                        color_image.width(),
                        color_image.height(),
                        31,
                    );
                    rec.copy_image_to_buffer(
                        &color_image,
                        maligog::ImageLayout::TRANSFER_SRC_OPTIMAL,
                        &color_image_buffer,
                    );
                });
            });
            device.graphics_queue().submit_blocking(&[cmd_buf]);

            swapchain.present(index, &[&swapchain.image_available_semaphore()]);
        }
    }
}
