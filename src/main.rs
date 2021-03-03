mod vec3;
mod ray;
mod camera;
mod hittable;
mod sphere;
mod hittable_list;

const INFINITY : f64 = std::f64::INFINITY;

use std::time::{Instant};

use camera::{Camera};
use hittable::{Hittable};
use hittable_list::{HittableList};
use ray::Ray;
use sphere::Sphere;
use vec3::{Colour, Point3, Vec3};


fn write_colour(pixel_colour : Colour) {
	println!("{} {} {}\n",
		(255.999 * pixel_colour.x) as usize,
		(255.999 * pixel_colour.y) as usize,
		(255.999 * pixel_colour.z) as usize);
}

fn ray_colour(r : &ray::Ray, world: &dyn Hittable) -> Colour {
	match world.hit(r, 0.0, INFINITY) {
		Some(hr) => {
			return (hr.normal + Colour::new(1.0,1.0,1.0)) * 0.5;
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

	const ASPECT_RATIO : f64 = (16/9) as f64;
	const WIDTH : usize = 400;
	const HEIGHT : usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

	// World
	let mut world : HittableList = HittableList::new();
	world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
	world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.)));

	// Camera
	let camera = Camera::new(ASPECT_RATIO);


	// Render
	println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
	for j in (0..HEIGHT).rev() {
		eprint!("\r{} scanlines remaining", j);
		for i in 0..WIDTH {
			let u = (i as f64) / ((WIDTH-1) as f64);
			let v = (j as f64) / ((HEIGHT-1) as f64);
			let r = camera.ray(u,v);
			let c = ray_colour(&r, &world);
			write_colour(c);
		}
	}
	eprint!("\ndone {}ms\n", start.elapsed().as_millis());
}
