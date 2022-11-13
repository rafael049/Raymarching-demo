
pub struct Transform {
		pub position: glm::Vec3,
		pub rotation: glm::Vec3,
		pub scale: glm::Vec3,

		local_mat: glm::Mat4,
		model_mat: glm::Mat4,
}

impl Transform {
		pub fn new() -> Transform {
				Transform {
						position: glm::vec3(0.0, 0.0, 0.0),
						rotation: glm::vec3(0.0, 0.0, 0.0),
						scale: glm::vec3(0.0, 0.0, 0.0),

						local_mat: glm::identity::<f32, 4>(),
						model_mat: glm::identity::<f32, 4>(),
				}
		}
}
