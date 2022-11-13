extern crate nalgebra_glm as glm;

pub struct Camera{
		pub position: glm::Vec3,
		pub yaw: f32,
		pub pitch: f32,
		pub row: f32,
		pub front: glm::Vec3,
		pub up: glm::Vec3,
		pub view_mat: glm::Mat4,
}

impl Camera {
		pub fn new() -> Camera {
				Camera{
						position: glm::vec3(0.0, 0.0, 0.0),
						yaw: 0.0,
						pitch: 0.0,
						row: 0.0,
						front: glm::vec3(0.0, 0.0, -1.0),
						up: glm::vec3(0.0, 1.0, 0.0),
						view_mat: glm::identity::<f32, 4>(),
				}
		}

		pub fn look_at(&mut self) {
				self.front = glm::vec3(self.pitch.cos()*self.yaw.sin(),
																self.pitch.sin(),
																-self.pitch.cos()*self.yaw.cos());
				self.view_mat = glm::look_at(&-self.position, &-(self.position + self.front), &self.up);
		}


}
