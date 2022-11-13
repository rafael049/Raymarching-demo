#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;

use std::time::{SystemTime, Instant};

use glium::{glutin::event::KeyboardInput, backend::Facade};
#[allow(unused_imports)]
use glium::{glutin, Surface};

use crate::input::{ActionInput, KeyState};
use crate::glutin::dpi::LogicalPosition;

mod resources;
mod camera;
mod input;
mod object;
mod shader;

#[derive(Copy, Clone)]
struct Object {
		size: f32,        // 4
		kind: u32,        // 8
		material_id: u32, // 12
		pos: [f32;4],     // 16
}


implement_uniform_block!(Object, size, kind, material_id, pos);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_hardware_acceleration(Some(false)).with_vsync(false);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

		let mut window_size: (u32, u32) = (display.gl_window().window().inner_size().width,
																			 display.gl_window().window().inner_size().height);
		let mut mouse_pos: (u32, u32) = (0, 0);


    // building the vertex buffer, which contains all the vertices that we will draw

		let mut rsrc = resources::Resources::new();
		let mut camera = camera::Camera::new();
		let mut input = input::Input::new();
		let mut object = object::Object::new();
		let mut shader = shader::Shader::new(&display, "shaders/vertex_raymarching.glsl", "shaders/fragment_raymarching.glsl").unwrap();

		let mut cubo = object::Object::new();

		camera.position[2] = 5.0;

    let vertex_buffer = {
				let mesh = rsrc.get_mesh("plane");
        glium::VertexBuffer::new(&display,
																 &mesh.vertex
        ).unwrap()
    };

    // building the index buffer
		let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.

		
		let mut trsf = glm::identity::<f32, 4>();
		let proj = glm::perspective::<f32>(16.0/9.0, 60.0f32.to_radians(), 0.1, 1000.0);


		let params = glium::DrawParameters {
				depth: glium::Depth {
						test: glium::draw_parameters::DepthTest::IfLess,
						write: true,
						.. Default::default()
				},
				.. Default::default()
		};

		use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
		use glutin::event_loop::{ControlFlow, EventLoop};

		let mut last_instant = Instant::now();
		let mut total_time = 0.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => {
										*control_flow = glutin::event_loop::ControlFlow::Exit;
										return;
								},
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(physical_size) => {
										window_size.0 = physical_size.width;
										window_size.1 = physical_size.height;
                },
								glutin::event::WindowEvent::KeyboardInput {
										input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
										..
								} => match (virtual_code, state) {
										(VirtualKeyCode::Escape, _) => {
												*control_flow = ControlFlow::Exit;
												return;
										},
										_ => input.process_event(state, virtual_code)
								}
								glutin::event::WindowEvent::CursorMoved { position: pos, .. } => {
										mouse_pos = (pos.x as u32, pos.y as u32);
										//display.gl_window().window() .set_cursor_position(LogicalPosition::new(window_size.0 as f32 /2.0, window_size.1 as f32 /2.0));
								},
								glutin::event::WindowEvent::MouseInput { state: s, button: but, ..} =>
										input.process_mouse_buttons(s, but),
										
                _ => {}, //*control_flow = glutin::event_loop::ControlFlow::Poll,
            },

						// Rendering
						glutin::event::Event::MainEventsCleared => {


								let current_instant = Instant::now();
								let delta_time = current_instant.duration_since(last_instant).as_secs_f32();
								last_instant = current_instant;
								total_time += delta_time;

								//println!("fps: {:?}" , 1.0/delta_time);

								input.process_mouse_move(mouse_pos.0, mouse_pos.1, window_size);
								let _ = shader.reload(&display);

								let move_sentivity = 10.0;
								let look_sentivity = 50.0;

								if input.get_mouse_button(input::MouseButtonKeyCode::Right) == KeyState::KeyDown {
										match input.get_actions(input::ActionID::LOOK) {
												ActionInput::DIRECTIONAL(dir) => {
														camera.yaw   += delta_time * look_sentivity * dir.x;
														camera.pitch += delta_time * look_sentivity * dir.y;
												}
												_ => (),
										}
								}

								match input.get_actions(input::ActionID::MOVE) {
										ActionInput::DIRECTIONAL(dir) => {
												let front = camera.front * dir.y;
												let right = camera.front.cross(&camera.up) * dir.x;
												camera.position += delta_time*move_sentivity * (front + right);
										},
										_ => (),
								}




								let teste = 
										Object {
												pos: [0.5, 1.0, 1.0, 0.0],
												kind: 1,
												size: 1.0,
												material_id: 1,
										};

								camera.look_at();
								let teste_buffer = glium::uniforms::UniformBuffer::new(&display, teste).unwrap();
								let resolution = glm::vec2(window_size.0 as f32, window_size.1 as f32);
								let uniforms = uniform!{
										trsf_mat: *trsf.as_ref(),
										view_mat: *camera.view_mat.as_ref(),
										proj_mat: *proj.as_ref(),
										iTime: total_time,
										iResolution: *resolution.as_ref(),
										Objects: &teste_buffer,
								};


								//trsf = glm::rotate(&trsf, 0.01, &glm::vec3(0.0, 1.0, 1.0));

								
								let mut target = display.draw();
								target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
								target.draw(&vertex_buffer, &index_buffer, &shader.get_program(), &uniforms, &params).unwrap();
								target.finish().unwrap();
						}

            _ => {}, // *control_flow = glutin::event_loop::ControlFlow::Poll,
        };
    });
}
