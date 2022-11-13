
pub struct Material {
		pub color: glm::Vec3,
		pub specular: f32,
}

impl Material {
		pub fn new(color: glm::Vec3, specular: f32) -> Material {
				Material {
						color,
						specular,
				}
		}
}
