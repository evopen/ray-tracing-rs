use std::sync::Arc;

use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::Point3;
use crate::Vec3;
use crate::{material, utils};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::splat(0.9),
    ));
    let ground_material = Arc::new(material::Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::rand_f64();
            let center = Point3::new(
                a as f64 + 0.9 * utils::rand_f64(),
                0.2,
                b as f64 + 0.9 * utils::rand_f64(),
            );

            if choose_mat < 0.7 {
                // diffuse
                let albedo = utils::rand_vec3() * utils::rand_vec3();
                let sphere_material = Arc::new(material::Lambertian::new_with_color(albedo));
                let center_1 = center + Vec3::new(0.0, utils::rand_f64_range(0.0, 0.5), 0.0);
                world.add(Arc::new(MovingSphere::new(
                    center,
                    center_1,
                    0.0,
                    1.0,
                    0.2,
                    sphere_material,
                )));
            } else if choose_mat < 0.85 {
                // metal
                let albedo = utils::rand_vec3_range(0.5, 1.0);
                let fuzz = utils::rand_f64_range(0.0, 0.5);
                let sphere_material = Arc::new(material::Metal::new(albedo, fuzz));
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            } else {
                let sphere_material = Arc::new(material::Dielectric::new(1.5));
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(material::Lambertian::new_with_color(Color::new(
        0.4, 0.2, 0.1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(material::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::splat(0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(material::Lambertian::new(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(material::Lambertian::new(checker.clone())),
    )));
    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(material::Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(material::Lambertian::new(perlin_texture.clone())),
    )));
    objects
}

pub fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("image/earthmap.jpg"));
    let earth_surface = Arc::new(material::Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::default(), 2.0, earth_surface));

    let mut list = HittableList::new();
    list.add(globe);
    list
}
