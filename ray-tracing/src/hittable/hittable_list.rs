use std::sync::Arc;

use super::{BVHNode, AABB};
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
    aabb_buffer: Vec<maligog::Buffer>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            aabb_buffer: vec![],
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn build_bvh(&self, time_0: crate::Float, time_1: crate::Float) -> BVHNode {
        BVHNode::new(&self.objects, time_0, time_1)
    }

    pub fn build_tlas(&mut self, device: &maligog::Device) -> maligog::TopAccelerationStructure {
        let mut aabb_positions = Vec::new();
        for object in &self.objects {
            let bb = object.bounding_box(0.0, 1.0).unwrap();

            aabb_positions.push(bb);
        }
        let a: &[u8] = bytemuck::cast_slice(&aabb_positions);
        dbg!(&a.len());
        let buffer = device.create_buffer_init(
            Some("aabb positions"),
            a,
            maligog::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR,
            maligog::MemoryLocation::GpuOnly,
        );
        dbg!(&buffer.handle());
        self.aabb_buffer.push(buffer.clone());
        let mut blases = Vec::new();
        for (i, object) in self.objects.iter().enumerate() {
            let buffer_view = maligog::BufferView {
                buffer: buffer.clone(),
                offset: (std::mem::size_of::<AABB>() * i) as u64,
            };
            let blas = device.create_bottom_level_acceleration_structure(
                Some("blas"),
                &[maligog::AABBGeometry::new(buffer_view, 1)],
            );
            let mut blas_instance =
                maligog::BLASInstance::new(device, &blas, &glam::Mat4::IDENTITY, 0, 0);
            blas_instance.build();
            blases.push(blas_instance);
        }
        let instance_geometries = maligog::InstanceGeometry::new(device, &blases);
        device.create_top_level_acceleration_structure(Some("main"), &[instance_geometries])
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        return result;
    }

    fn bounding_box(&self, time_0: crate::Float, time_1: crate::Float) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        };
        let mut object_iter = self.objects.iter();
        let mut bounding_box =
            if let Some(bb) = object_iter.next().unwrap().bounding_box(time_0, time_1) {
                bb
            } else {
                return None;
            };
        while let Some(object) = object_iter.next() {
            if let Some(b) = object.bounding_box(time_0, time_1) {
                bounding_box = b.surrounding_box(&bounding_box);
            } else {
                return None;
            }
        }
        Some(bounding_box)
    }

    fn intersection_shader_entry_point(&self) -> Option<&str> {
        None
    }
}
