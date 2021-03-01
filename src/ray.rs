use crate::vec3::{Point3,Vec3};

pub struct Ray {
	pub origin : Point3,
	pub dir : Vec3,
}

impl Ray {
	pub fn new(origin: &Point3, dir: &Vec3) -> Self {
		Ray{
			origin: *origin,
			dir: *dir,
		}
	}
}