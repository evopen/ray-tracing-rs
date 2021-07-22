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

#[repr(C)]
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

        let xy = UVec2::new(pixel.x, pixel.y);

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
    // #[spirv(hit_attribute)] hit_attr: &mut Vec2,
    // #[spirv(instance_id)] instance_id: usize, // index of instance in tlas
    // #[spirv(ray_geometry_index)] geometry_index: usize, // index of geometry in instance
    // #[spirv(primitive_id)] primitive_id: usize, // index of triangle in geometry
    // #[spirv(instance_custom_index)] instance_custom_index: usize, // blas id
    // #[spirv(ray_tmax)] ray_tmax: f32,
    // #[spirv(world_ray_origin)] world_ray_origin: Vec3,
    // #[spirv(world_ray_direction)] world_ray_direction: Vec3,
    // #[spirv(object_ray_origin)] object_ray_origin: Vec3,
    // #[spirv(object_ray_direction)] object_ray_direction: Vec3,
    // #[spirv(shader_record_buffer)] shader_record_buffer: &mut ShaderRecordData,
    // // #[spirv(world_to_object)] world_to_object: glam::Affine3A,
    // #[spirv(descriptor_set = 0, binding = 0)] tlas: &spirv_std::ray_tracing::AccelerationStructure,
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] geometry_infos: &mut [GeometryInfo], // per-BLAS
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] geometry_info_offsets: &mut [u32], // per-BLAS
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] transform_buffer: &mut [Mat4], // per-instance
    // #[spirv(descriptor_set = 0, binding = 6)] samplers: &RuntimeArray<Sampler>,
    // #[spirv(descriptor_set = 0, binding = 7)] images: &RuntimeArray<Image2d>,
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] material_infos: &mut [MaterialInfo],
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 9)] color_buffer: &mut [Vec4],
    // #[spirv(storage_buffer, descriptor_set = 0, binding = 10)] tex_coord_buffer: &mut [Vec2],
    // #[spirv(push_constant)] camera_info: &CameraInfo,
) {
    payload.color = Vec3::new(0.0, 1.0, 0.0);
}

#[spirv(miss)]
pub fn miss(
    #[spirv(incoming_ray_payload)] payload: &mut Payload,
    #[spirv(world_ray_direction)] world_ray_direction: Vec3,
) {
    payload.color = Vec3::new(0.7, 0.8, 1.0);
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
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: f32,
}

// this method is documented in raytracing gems book
fn gems_intersections(orig: Vec3, dir: Vec3, center: Vec3, radius: f32) -> Vec2 {
    let f = orig - center;
    let a = dir.dot(dir);
    let bi = (-f).dot(dir);
    let c = f.dot(f) - radius * radius;
    let s = f + (bi / a) * dir;
    let discr = radius * radius - s.dot(s);

    let mut t = vec2(-1.0, -1.0);
    if (discr >= 0.0) {
        let q = bi + (bi).signum() * (a * discr).sqrt();
        let t1 = c / q;
        let t2 = q / a;
        t = vec2(t1, t2);
    }
    return t;
}

#[spirv(intersection)]
pub fn sphere_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 11)] spheres: &mut [Sphere],
    #[spirv(instance_id)] instance_id: usize, // index of instance in tlas
    #[spirv(ray_geometry_index)] geometry_index: usize, // index of geometry in instance
    #[spirv(primitive_id)] primitive_id: usize, // index of triangle in geometry
    #[spirv(instance_custom_index)] instance_custom_index: usize, // blas id
    #[spirv(world_ray_direction)] world_ray_direction: Vec3,
    #[spirv(world_ray_origin)] world_ray_origin: Vec3,
) {
    let sphere = &spheres[instance_id];

    let t = gems_intersections(
        world_ray_origin,
        world_ray_direction,
        Vec3::new(sphere.x, sphere.y, sphere.z),
        sphere.r,
    );

    unsafe {
        report_intersection(t.x, 0);
        report_intersection(t.y, 0);
    }
}

#[spirv(intersection)]
pub fn translate_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] translates: &mut [Sphere],
    #[spirv(instance_id)] instance_id: usize, // index of instance in tlas
    #[spirv(ray_geometry_index)] geometry_index: usize, // index of geometry in instance
    #[spirv(primitive_id)] primitive_id: usize, // index of triangle in geometry
    #[spirv(instance_custom_index)] instance_custom_index: usize, // blas id
) {
    unsafe {}
}

#[spirv(intersection)]
pub fn rotate_y_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] rotate_ys: &mut [Sphere],
) {
    unsafe {}
}

#[spirv(intersection)]
pub fn x_y_rect_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] x_y_rects: &mut [Sphere],
) {
    unsafe {}
}

#[spirv(intersection)]
pub fn box_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] boxes: &mut [Sphere],
) {
    unsafe {}
}

#[spirv(intersection)]
pub fn constant_medium_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] constant_mediums: &mut [Sphere],
) {
    unsafe {}
}

#[spirv(intersection)]
pub fn moving_sphere_intersection(
    #[spirv(storage_buffer, descriptor_set = 0, binding = 12)] moving_spheres: &mut [Sphere],
) {
    unsafe {}
}
