#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]
#![allow(dead_code, unused_imports, unused)]

mod util;

use spirv_std::arch::report_intersection;
use spirv_std::glam::uvec3;
use spirv_std::image::{Image2d, Image2dArray, Image2dI, Image2dU};
use spirv_std::num_traits::float::Float;
use spirv_std::RuntimeArray;

use spirv_std::glam;
use spirv_std::glam::{
    vec2, vec3, vec4, Mat4, UVec2, UVec3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles,
};
use spirv_std::Image;
use spirv_std::{image, Sampler};

#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

pub struct CameraInfo {
    view_inv: Mat4,
    projection_inv: Mat4,
}

pub struct Payload {
    color: Vec3,
    depth: u32,
    rng_state: u32,
}

#[spirv(ray_generation)]
pub fn main(
    #[spirv(push_constant)] camera_info: &CameraInfo,
    #[spirv(launch_id)] pixel: UVec3,
    #[spirv(launch_size)] launch_size: UVec3,
    #[spirv(ray_payload)] payload: &mut Payload,
    #[spirv(descriptor_set = 0, binding = 0)] tlas: &spirv_std::ray_tracing::AccelerationStructure,
    // #[spirv(descriptor_set = 0, binding = 1)] img: &Image!(2D, type=f32, sampled=false),
    #[spirv(descriptor_set = 1, binding = 0)] color_image: &mut image::Image<
        f32,
        { image::Dimensionality::TwoD },
        { image::ImageDepth::False },
        { image::Arrayed::False },
        { image::Multisampled::False },
        { image::Sampled::No },
        { image::ImageFormat::Rgba32f },
        { None },
    >,
    #[spirv(descriptor_set = 1, binding = 1)] ao_image: &mut image::Image<
        f32,
        { image::Dimensionality::TwoD },
        { image::ImageDepth::False },
        { image::Arrayed::False },
        { image::Multisampled::False },
        { image::Sampled::No },
        { image::ImageFormat::R32f },
        { None },
    >,
    // #[spirv(uniform, descriptor_set = 0, binding = 2)] camera_pos: &mut Vec2,
) {
    unsafe {
        let tmin = 0.001;
        let tmax = 10000.0;
        let origin = camera_info.view_inv * Vec3::splat(0.0).extend(1.0);

        let pixel_center = Vec2::new(pixel.x as f32, pixel.y as f32) + Vec2::splat(0.5);

        // map to (0, 1)
        let uv = pixel_center / Vec2::new(launch_size.x as f32, launch_size.y as f32);

        // map to (-1, 1) square
        let d = uv * 2.0 - Vec2::splat(1.0);

        let target = camera_info.projection_inv * d.extend(1.0).extend(1.0);
        let target_norm = (target.xyz() / target.w).normalize();
        let direction = (camera_info.view_inv * target_norm.extend(0.0)).normalize();

        payload.depth = 10;
        payload.rng_state = 10086;
        tlas.trace_ray(
            spirv_std::ray_tracing::RayFlags::OPAQUE,
            0xFF,
            0,
            0,
            0,
            origin.xyz(),
            tmin,
            direction.xyz(),
            tmax,
            payload,
        );

        let xy = UVec2::new(pixel.x, launch_size.y - pixel.y);

        color_image.write(xy, payload.color.extend(1.0));
    }
}

pub struct ShaderRecordData {
    index_offset: u32,
    vertex_offset: u32,
}

#[repr(C)]
pub struct GeometryInfo {
    pub index_offset: u64,
    pub vertex_offset: u64,
    pub index_count: u64,
    pub vertex_count: u64,
    pub material_index: u64,
    pub color_offset: u64,
    pub tex_coord_offset: u64,
    pub has_color: u32,
    pub has_tex_coord: u32,
}

#[repr(C)]
pub struct MaterialInfo {
    base_color_factor: glam::Vec4,
    has_base_color_texture: u32,
    base_color_sampler_index: u32,
    base_color_image_index: u32,
    has_metallic_roughness_texture: u32,
    metallic_roughness_sampler_index: u32,
    metallic_roughness_image_index: u32,
    padding: u64,
}

#[spirv(closest_hit)]
pub fn closest_hit(
    #[spirv(incoming_ray_payload)] payload: &mut Payload,
    #[spirv(hit_attribute)] hit_attr: &mut Vec2,
    #[spirv(instance_id)] instance_id: usize, // index of instance in tlas
    #[spirv(ray_geometry_index)] geometry_index: usize, // index of geometry in instance
    #[spirv(primitive_id)] primitive_id: usize, // index of triangle in geometry
    #[spirv(instance_custom_index)] instance_custom_index: usize, // blas id
    #[spirv(ray_tmax)] ray_tmax: f32,
    #[spirv(world_ray_origin)] world_ray_origin: Vec3,
    #[spirv(world_ray_direction)] world_ray_direction: Vec3,
    #[spirv(object_ray_origin)] object_ray_origin: Vec3,
    #[spirv(object_ray_direction)] object_ray_direction: Vec3,
    #[spirv(shader_record_buffer)] shader_record_buffer: &mut ShaderRecordData,
    // #[spirv(world_to_object)] world_to_object: glam::Affine3A,
    #[spirv(descriptor_set = 0, binding = 0)] tlas: &spirv_std::ray_tracing::AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] index_buffer: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] vertex_buffer: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] geometry_infos: &mut [GeometryInfo], // per-BLAS
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] geometry_info_offsets: &mut [u32], // per-BLAS
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] transform_buffer: &mut [Mat4], // per-instance
    #[spirv(descriptor_set = 0, binding = 6)] samplers: &RuntimeArray<Sampler>,
    #[spirv(descriptor_set = 0, binding = 7)] images: &RuntimeArray<Image2d>,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] material_infos: &mut [MaterialInfo],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 9)] color_buffer: &mut [Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 10)] tex_coord_buffer: &mut [Vec2],
    #[spirv(push_constant)] camera_info: &CameraInfo,
) {
    payload.depth -= 1;
    if payload.depth == 0 {
        payload.color = Vec3::splat(0.0);
        return;
    }
    let tmin = 0.001;
    let tmax = 10000.0;

    let barycentrics = vec3(1.0 - hit_attr.x - hit_attr.y, hit_attr.x, hit_attr.y);

    let geometry_info =
        &geometry_infos[geometry_info_offsets[instance_custom_index] as usize + geometry_index];
    let index_offset = (geometry_info.index_offset / 4) as usize; // by index
    let vertex_offset = (geometry_info.vertex_offset / 4) as usize; // by index
    let color_offset = (geometry_info.color_offset / 16) as usize; // by index
    let tex_coord_offset = (geometry_info.tex_coord_offset / 8) as usize; // by index

    let material_info = &material_infos[geometry_info.material_index as usize];

    let v0_index = index_buffer[index_offset + primitive_id * 3] as usize;
    let v0 = vec3(
        vertex_buffer[vertex_offset + v0_index * 3],
        vertex_buffer[vertex_offset + v0_index * 3 + 1],
        vertex_buffer[vertex_offset + v0_index * 3 + 2],
    );
    let v1_index = index_buffer[index_offset + primitive_id * 3 + 1] as usize;
    let v1 = vec3(
        vertex_buffer[vertex_offset + v1_index * 3],
        vertex_buffer[vertex_offset + v1_index * 3 + 1],
        vertex_buffer[vertex_offset + v1_index * 3 + 2],
    );
    let v2_index = index_buffer[index_offset + primitive_id * 3 + 2] as usize;
    let v2 = vec3(
        vertex_buffer[vertex_offset + v2_index * 3],
        vertex_buffer[vertex_offset + v2_index * 3 + 1],
        vertex_buffer[vertex_offset + v2_index * 3 + 2],
    );

    let object_to_world = transform_buffer[instance_id];

    let object_position = v0 * barycentrics.x + v1 * barycentrics.y + v2 * barycentrics.z;
    // let object_normal = (v1 - v0).cross(v2 - v0).normalize();
    let world_position = object_to_world.transform_point3(object_position);
    let world_v0 = object_to_world.transform_point3(v0);
    let world_v1 = object_to_world.transform_point3(v1);
    let world_v2 = object_to_world.transform_point3(v2);
    let mut world_normal = (world_v1 - world_v0).cross(world_v2 - world_v0).normalize();
    world_normal = util::facefoward(&world_normal, &world_ray_direction);

    if geometry_info.has_tex_coord == 1 {
        let v0_tex_coord = tex_coord_buffer[tex_coord_offset + v0_index];
        let v1_tex_coord = tex_coord_buffer[tex_coord_offset + v1_index];
        let v2_tex_coord = tex_coord_buffer[tex_coord_offset + v2_index];
        let tex_coord = v0_tex_coord * barycentrics.x
            + v1_tex_coord * barycentrics.y
            + v2_tex_coord * barycentrics.z;
        if material_info.has_base_color_texture == 1 {
            let sampler =
                unsafe { samplers.index(material_info.base_color_sampler_index as usize) };
            let image = unsafe { images.index(material_info.base_color_image_index as usize) };
            let texel: Vec4 = image.sample_by_lod(*sampler, tex_coord, 0.0);
            unsafe {
                let mut scatter_direction =
                    world_normal + util::rand_vec3_unit(&mut payload.rng_state);
                if util::is_near_zero(scatter_direction) {
                    scatter_direction = world_normal;
                }
                tlas.trace_ray(
                    spirv_std::ray_tracing::RayFlags::OPAQUE,
                    0xFF,
                    0,
                    0,
                    0,
                    world_position,
                    tmin,
                    scatter_direction,
                    tmax,
                    payload,
                );
            }
            payload.color = (texel * material_info.base_color_factor * 0.5).xyz() * payload.color;
        } else if geometry_info.has_color == 1 {
            let v0_color = color_buffer[color_offset + v0_index];
            let v1_color = color_buffer[color_offset + v1_index];
            let v2_color = color_buffer[color_offset + v2_index];
            let color =
                v0_color * barycentrics.x + v1_color * barycentrics.y + v2_color * barycentrics.z;
            payload.color = (color * material_info.base_color_factor).xyz();
        } else {
            payload.color = material_info.base_color_factor.xyz();
        }
    }
}

#[spirv(miss)]
pub fn miss(
    #[spirv(incoming_ray_payload)] payload: &mut Payload,
    #[spirv(world_ray_direction)] world_ray_direction: Vec3,
    #[spirv(descriptor_set = 1, binding = 2)] sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 0)] sky_texture: &image::Image<
        f32,
        { image::Dimensionality::TwoD },
        { image::ImageDepth::False },
        { image::Arrayed::False },
        { image::Multisampled::False },
        { image::Sampled::Yes },
        { image::ImageFormat::Rgba8 },
        { None },
    >,
) {
    // *payload = vec3(1.0, 0.5, 0.23);
    let coord = sample_sphereical_map(&world_ray_direction);
    let color: Vec4 = sky_texture.sample_by_lod(*sampler, coord, 0.0);
    payload.color = color.xyz() * 2.0;
}

pub fn sample_sphereical_map(direction: &Vec3) -> Vec2 {
    let inv_atan = vec2(0.1591, 0.3183);
    let mut uv = vec2(direction.z.atan2(direction.x), -direction.y.asin());
    uv *= inv_atan;
    uv += Vec2::splat(0.5);
    return uv;
}

#[repr(C)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    material_id: u32,
}

#[spirv(intersection)]
pub fn sphere_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 11)] spheres: &mut [Sphere],
) {
    unsafe {}
}
