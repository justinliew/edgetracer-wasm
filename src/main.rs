mod vec3;
mod ray;
mod camera;
mod dielectric;
mod hittable;
mod sphere;
mod hittable_list;
mod material;
mod lambertian;
mod metal;
mod utils;

const INFINITY : f64 = std::f64::INFINITY;

use std::time::{Instant};
use std::rc::Rc;

use camera::{Camera};
use hittable::{Hittable};
use hittable_list::{HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Colour, Point3, Vec3};
use lambertian::Lambertian;
use metal::Metal;
use dielectric::Dielectric;
use utils::{clamp,rand_unit_vector};


fn write_colour(pixel_colour : Colour, samples_per_pixel: usize) {
	let scale = 1.0 / (samples_per_pixel as f64);
	let cr = clamp(pixel_colour.x * scale, 0.0, 0.999);
	println!("{} {} {}\n",
		(256.0 * clamp(f64::sqrt(pixel_colour.x * scale), 0.0, 0.999)) as usize,
		(256.0 * clamp(f64::sqrt(pixel_colour.y * scale), 0.0, 0.999)) as usize,
		(256.0 * clamp(f64::sqrt(pixel_colour.z * scale), 0.0, 0.999)) as usize);
}

fn ray_colour(r : &ray::Ray, world: &dyn Hittable, depth: usize) -> Colour {

	if depth  <= 0 {
		return Colour::new(0.0,0.0,0.0);
	}
	match world.hit(r, 0.001, INFINITY) {
		Some(hr) => {
			match hr.material.scatter(r, &hr) {
				Some((scattered,attenuation)) => {
					ray_colour(&scattered, world, depth-1) * attenuation
				},
				None => Colour::new(0.0,0.0,0.0)
			}
		},
		None => {
			let unit_direction = Vec3::unit_vector(r.dir);
			let t = 0.5 * (unit_direction.y + 1.0);
			Colour::new(1.0,1.0,1.0) * (1.0-t) + Colour::new(0.5,0.7,1.0) * t
		}
	}
}



fn main() {
	// TODO - add a timing param so it won't write colours, just calculate them

	let start = Instant::now();

	const SAMPLES_PER_PIXEL : usize = 10;
	const MAX_DEPTH : usize = 10;

	const ASPECT_RATIO : f64 = 16.0/9.0;
	const WIDTH : usize = 400;
	const HEIGHT : usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

	// World
	let mut world : HittableList = HittableList::new();
	let material_ground = Rc::new(Lambertian::new(&Colour::new(0.8,0.8,0.0)));
	let material_centre = Rc::new(Lambertian::new(&Colour::new(0.7,0.3,0.3)));
	let material_left = Rc::new(Metal::new(&Colour::new(0.8,0.8,0.8)));
	let material_right = Rc::new(Dielectric::new(1.5));

	world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100., material_ground)));
	world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5, material_centre)));
	world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.5, material_left)));
	world.add(Box::new(Sphere::new(Point3::new(1.0,0.0,-1.0), -0.4, material_right)));

	// Camera
	let camera = Camera::new(ASPECT_RATIO, 135.0);


	// Render
	println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
	for j in (0..HEIGHT).rev() {
		eprint!("\r{:03} scanlines remaining", j);
		for i in 0..WIDTH {
			let mut pixel_colour = Colour::new(0.0,0.0,0.0);
			for _ in 0..SAMPLES_PER_PIXEL {
				let u = ((i as f64) + rand::random::<f64>()) / ((WIDTH-1) as f64);
				let v = ((j as f64) + rand::random::<f64>()) / ((HEIGHT-1) as f64);
				let r = camera.ray(u,v);
				let c = ray_colour(&r, &world, MAX_DEPTH);
				pixel_colour = pixel_colour + c;
			}
			write_colour(pixel_colour, SAMPLES_PER_PIXEL);
		}
	}
	eprint!("\ndone {}ms\n", start.elapsed().as_millis());
}
