extern crate nalgebra_glm as glm;

pub mod transform;
pub mod material;


pub struct Object {
		pub transform: transform::Transform,
		pub material: material::Material,
}

impl Object {
		pub fn new() -> Object {
				Object {
						transform: transform::Transform::new(),
								material: material::Material::new(glm::vec3(0.0, 0.0, 0.0), 1.0),
				}
		}
}
