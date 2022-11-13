extern crate glium;
extern crate nalgebra_glm as glm;

use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Vertex {
		position: [f32; 3],
		normal:  [f32; 3],
		color: [f32; 3],
		uv: [f32; 2],
}

glium::implement_vertex!(Vertex, position, normal, color, uv);


pub struct Mesh{
		pub id: u64,
		pub vertex: Vec<Vertex>,
		pub index: Vec<u64>
}

pub struct Resources {
		meshes: HashMap<String, Mesh>,
		textures: HashMap<String, i32>
}

fn load_mesh_from_file(_name: &str) -> Mesh {
		Mesh { id: 0,
						vertex:
						vec![
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},

								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 0.0]},

								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 0.0]},

								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 0.0]},

								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 1.0]},

								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 1.0]}
						],
					 index: vec![],
		}
}

impl Resources {
		pub fn new() -> Resources {
				let mut new_meshes = HashMap::new();

				let key = "plane".to_string();
				new_meshes.insert(key,
						Mesh{
								id: 0,
								vertex: vec![
										Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
										Vertex { position: [ 1.0, -1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 0.0]},
										Vertex { position: [ 1.0,  1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
										Vertex { position: [-1.0,  1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
										Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 1.0]},
										Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
								],
								index: vec![],
						});

				Resources { meshes: new_meshes, textures: HashMap::new() }
		}

		pub fn get_mesh(&mut self, name: &str) -> &Mesh {
				if self.meshes.contains_key(name) {
						return &self.meshes[name];
				}
				else {
						let loaded_mesh = load_mesh_from_file(name);
						self.meshes.insert(name.to_string(), loaded_mesh);
						return &self.meshes[name];
				}
		}
}

