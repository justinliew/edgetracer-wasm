mod vec3;
mod ray;

use vec3::{Colour, Point3, Vec3};
use ray::Ray;
use std::time::{Instant};

fn write_colour(pixel_colour : Colour) {
	println!("{} {} {}\n",
		(255.999 * pixel_colour.x) as usize,
		(255.999 * pixel_colour.y) as usize,
		(255.999 * pixel_colour.z) as usize);
}

fn ray_colour(r : &ray::Ray) -> Colour {
	let t = sphere_intersection(&Point3::new(0.0,0.0,-1.0), 0.5, r);
	if t > 0.0 {
		let N = Vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
		return Colour::new(N.x+1.0,N.y+1.0,N.z+1.0) * 0.5;
	}
	let unit_direction = Vec3::unit_vector(r.dir);
	let t = 0.5 * (unit_direction.y + 1.0);
	Colour::new(1.0,1.0,1.0) * (1.0-t) + Colour::new(0.5,0.7,1.0) * t
}

fn sphere_intersection(centre: &Point3, radius: f64, r: &Ray) -> f64 {
	let oc = r.origin - *centre;
	let a = Vec3::dot(&r.dir, &r.dir);
	let b = 2.0 * Vec3::dot(&oc, &r.dir);
	let c = Vec3::dot(&oc,&oc) - radius * radius;
	let disc = b*b - 4.0*a*c;
	if disc < 0.0 {
		return -1.0;
	} else {
		return (-b - disc.sqrt()) / (2.0 * a);
	}
}

fn main() {

	let start = Instant::now();

	// Image
	const ASPECT_RATIO : f64 = (16/9) as f64;
	const WIDTH : usize = 400;
	const HEIGHT : usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

	// Camera
	const VIEWPORT_HEIGHT : f64 = 2.0;
	const VIEWPORT_WIDTH : f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
	const FOCAL_LENGTH : f64 = 1.0;

	let origin : Point3 = Point3::new(0.0,0.0,0.0);
	let horizontal : Point3 = Point3::new(VIEWPORT_WIDTH, 0.0,0.0);
	let vertical : Point3 = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
	let ll : Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,FOCAL_LENGTH);

	// Render
	println!("P3\n{} {}\n255\n", WIDTH, HEIGHT);
	for j in (0..HEIGHT).rev() {
		eprint!("\r{} scanlines remaining", j);
		for i in 0..WIDTH {
			let u = (i as f64) / ((WIDTH-1) as f64);
			let v = (j as f64) / ((HEIGHT-1) as f64);
			let r = Ray::new(&origin,
				&(ll + horizontal * u + vertical * v - origin));
			let c = ray_colour(&r);
			write_colour(c);
		}
	}
	eprint!("\ndone {}ms\n", start.elapsed().as_millis());
}
