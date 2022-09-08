use std::sync::Arc;

use crate::color::Color;
use crate::hittable::Box;
use crate::hittable::ConstantMedium;
use crate::hittable::HittableList;
use crate::hittable::MovingSphere;
use crate::hittable::Sphere;
use crate::hittable::{RotateY, Translate};
use crate::hittable::{XYRect, XZRect, YZRect};
use crate::material::Dielectric;
use crate::material::Metal;
use crate::material::{DiffuseLight, Lambertian};
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::types::Point3;
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
            let choose_mat = utils::gen_float();
            let center = Point3::new(
                a as crate::Float + 0.9 * utils::gen_float(),
                0.2,
                b as crate::Float + 0.9 * utils::gen_float(),
            );

            if choose_mat < 0.7 {
                // diffuse
                let albedo = utils::rand_vec3() * utils::rand_vec3();
                let sphere_material = Arc::new(material::Lambertian::new_with_color(albedo));
                let center_1 = center + Vec3::new(0.0, utils::gen_range(0.0..0.5), 0.0);
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
                let fuzz = utils::gen_range(0.0..0.5);
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

pub fn simple_light() -> HittableList {
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

    let diffuse_light = Arc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light,
    )));

    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_with_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_color(Color::splat(15.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let box2 = Arc::new(Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_with_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_color(Color::splat(7.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Arc::new(Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let box1 = Arc::new(ConstantMedium::new_with_color(
        box1,
        0.01,
        Color::splat(0.0),
    ));
    objects.add(box1);

    let box2 = Arc::new(Box::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    let box2 = Arc::new(ConstantMedium::new_with_color(
        box2,
        0.01,
        Color::splat(1.0),
    ));
    objects.add(box2);

    objects
}

pub fn final_scene() -> HittableList {
    let mut objects = HittableList::new();

    let ground = Arc::new(Lambertian::new_with_color(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    let mut ground_boxes = HittableList::new();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as crate::Float * w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as crate::Float * w;
            let x1 = x0 + w;
            let y1 = utils::gen_range(1.0..101.0);
            let z1 = z0 + w;

            ground_boxes.add(Arc::new(Box::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    objects.add(Arc::new(ground_boxes.build_bvh(0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_with_color(Color::splat(7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let moving_sphere_material = Arc::new(Lambertian::new_with_color(Color::new(0.7, 0.3, 0.1)));
    let center_0 = Point3::new(400.0, 400.0, 200.0);
    let center_1 = center_0 + Vec3::new(30.0, 0.0, 0.0);
    objects.add(Arc::new(MovingSphere::new(
        center_0,
        center_1,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // subsurface
    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // fog
    let boundary = Arc::new(Sphere::new(
        Point3::splat(0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        0.0001,
        Color::splat(1.0),
    )));

    // earth
    let earth_mat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "image/earthmap.jpg",
    ))));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));

    let perlin = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin)),
    )));

    let mut balls = HittableList::new();
    let white = Arc::new(Lambertian::new_with_color(Color::splat(0.73)));
    let num = 1000;
    for _ in 0..num {
        balls.add(Arc::new(Sphere::new(
            utils::rand_vec3_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(balls.build_bvh(0.0, 1.0)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
