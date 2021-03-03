use crate::vec3::{Point3,Vec3};
use crate::ray::{Ray};

pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front: bool,
}

impl HitRecord {
	pub fn new(r: &Ray, root: f64, outward_normal: &Vec3) -> Self {
		let p = r.at(root);
		let front = Vec3::dot(&r.dir, &outward_normal) < 0.0;
		let normal = match front {
			true => *outward_normal,
			false => -*outward_normal,
		};
		HitRecord{
			p: p,
			normal: normal,
			t: root,
			front: front,
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
