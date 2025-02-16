mod colour;
mod hittable;
mod ray;
mod sphere;
mod vec;

use crate::{
    colour::Colour,
    ray::Ray,
    vec::{Point3, Vec3},
};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    let a = &r.direction().length_squared();
    let h = Vec3::dot(&r.direction(), &oc);
    let c = &oc.length_squared() - radius * radius;
    let discriminant = h * h - *a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-h - discriminant.sqrt()) / a
    }
}

fn ray_colour(r: &Ray) -> Colour {
    let t = hit_sphere(Point3::from(0.0, 0.0, -1.0), 0.5, r);
    if (t > 0.0) {
        let n_diff = r.at(t) - Vec3::from(0.0, 0.0, -1.0);
        let n = Vec3::unit_vector(&n_diff);
        return 0.5 * Colour::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - a) * Colour::from(1.0, 1.0, 1.0)) + (a * Colour::from(0.5, 0.7, 1.0))
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    // Set viewport to true image values
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new();

    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / ((image_width) as f64);
    let pixel_delta_v = viewport_v / ((image_height) as f64);

    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - y);
        for x in 0..image_width {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from(camera_center, ray_direction);

            let pixel_colour = ray_colour(&r);
            colour::write_colour(&mut std::io::stdout(), &pixel_colour).unwrap()
        }
    }

    eprint!("\rDone                 \n");
}
