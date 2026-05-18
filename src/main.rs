#![warn(clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

mod gl;
mod math;
mod window;

use crate::{
    gl::{GL, Triangle},
    math::{Vec2, Vec2I},
    window::Window,
};

static VERTICES: [Vec2; 4] = [
    Vec2 { x: 1.0, y: 1.0 },
    Vec2 { x: 1.0, y: -1.0 },
    Vec2 { x: -1.0, y: -1.0 },
    Vec2 { x: -1.0, y: 1.0 },
];
static ELEMENTS: [Triangle<u8>; 2] = [Triangle(0, 1, 3), Triangle(1, 2, 3)];
const VERTEX_SHADER_SOURCE: &str = include_str!("shader/vertex.glsl");
const MANDELBROT_SHADER_SOURCE: &str = include_str!("shader/mandelbrot.glsl");
const JULIA_SHADER_SOURCE: &str = include_str!("shader/julia.glsl");

struct GenericProgram {
    program: u32,

    viewport_uniform: i32,
    zoom_uniform: i32,
    center_uniform: i32,
    quality_uniform: i32,
}

impl GenericProgram {
    fn new(gl: &GL, vertex_shader: u32, source: &str) -> Self {
        let shader = gl.create_shader(GL::FRAGMENT_SHADER);
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        let program = gl.create_program();
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, shader);
        gl.link_program(program);
        gl.delete_shader(shader);

        let viewport_uniform = gl.get_uniform_location(program, c"viewport");
        let zoom_uniform = gl.get_uniform_location(program, c"zoom");
        let center_uniform = gl.get_uniform_location(program, c"center");
        let quality_uniform = gl.get_uniform_location(program, c"quality");

        Self {
            program,
            viewport_uniform,
            zoom_uniform,
            center_uniform,
            quality_uniform,
        }
    }

    fn r#use(&self, gl: &GL) {
        gl.use_program(self.program);
    }

    fn set_viewport(&self, gl: &GL, value: Vec2I) {
        gl.uniform_2iv(self.viewport_uniform, value);
    }

    fn set_zoom(&self, gl: &GL, value: f32) {
        gl.uniform_1f(self.zoom_uniform, value);
    }

    fn set_center(&self, gl: &GL, value: Vec2) {
        gl.uniform_2fv(self.center_uniform, value);
    }

    fn set_quality(&self, gl: &GL, value: i32) {
        gl.uniform_1i(self.quality_uniform, value);
    }
}

struct JuliaProgram {
    generic_program: GenericProgram,

    parameter_uniform: i32,
}

impl JuliaProgram {
    fn new(gl: &GL, vertex_shader: u32, source: &str) -> Self {
        let generic_program = GenericProgram::new(gl, vertex_shader, source);

        let parameter_uniform = gl.get_uniform_location(generic_program.program, c"parameter");

        Self {
            generic_program,
            parameter_uniform,
        }
    }

    fn set_parameter(&self, gl: &GL, value: Vec2) {
        gl.uniform_2fv(self.parameter_uniform, value);
    }
}

fn main() {
    let window = Window::create().unwrap();
    let gl = window.load_gl();

    gl.enable(GL::DEBUG_OUTPUT);
    gl.debug_message_callback(|_, _, _, _, message| eprintln!("OpenGL Debug: {message}"));

    let vbo = gl.create_buffer();
    gl.named_buffer_data(vbo, &VERTICES, GL::STATIC_DRAW);
    let vao = gl.create_vertex_array();
    unsafe {
        gl.vertex_array_vertex_buffer(vao, 0, vbo, 0, std::mem::size_of::<Vec2>() as u32);
    }
    gl.enable_vertex_array_attrib(vao, 0);
    gl.vertex_array_attrib_format(vao, 0, 2, GL::FLOAT, false, 0);
    gl.vertex_array_attrib_binding(vao, 0, 0);

    let ebo = gl.create_buffer();
    gl.named_buffer_data(ebo, &ELEMENTS, GL::STATIC_DRAW);
    gl.vertex_array_element_buffer(vao, ebo);

    let vertex_shader = gl.create_shader(GL::VERTEX_SHADER);
    gl.shader_source(vertex_shader, VERTEX_SHADER_SOURCE);
    gl.compile_shader(vertex_shader);
    let mandelbrot_program = GenericProgram::new(&gl, vertex_shader, MANDELBROT_SHADER_SOURCE);
    let julia_program = JuliaProgram::new(&gl, vertex_shader, JULIA_SHADER_SOURCE);
    gl.delete_shader(vertex_shader);

    let mut current_program = &mandelbrot_program;

    gl.bind_vertex_array(vao);

    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut enter_pressed = false;

    let mut julia = false;
    let mut old_pointer = window.pointer_position();
    let mut center = Vec2::default();
    let mut quality: u8 = 1;
    loop {
        let viewport = gl.get_viewport();
        let pointer = window.pointer_position();
        let movement = if !window.left_button() || old_pointer == Vec2::default() {
            Vec2::default()
        } else {
            pointer - old_pointer
        };
        let zoom = 10.0f32.powf(window.total_scroll() as f32 / 100.0);
        center.x -= movement.x / viewport.x as f32 / zoom * 2.0;
        center.y += movement.y / viewport.y as f32 / zoom * 2.0;

        if window.up_key() && !up_pressed {
            up_pressed = true;
            quality += 1;
        } else if !window.up_key() && up_pressed {
            up_pressed = false;
        }
        if window.down_key() && !down_pressed {
            down_pressed = true;
            quality = u8::max(quality - 1, 1);
        } else if !window.down_key() && down_pressed {
            down_pressed = false;
        }
        if window.enter_key() && !enter_pressed {
            enter_pressed = true;
            if julia {
                julia = false;
                current_program = &mandelbrot_program;
            } else {
                julia = true;
                current_program = &julia_program.generic_program;
                current_program.r#use(&gl);
                julia_program.set_parameter(&gl, center);
            }
        } else if !window.enter_key() && enter_pressed {
            enter_pressed = false;
        }

        current_program.r#use(&gl);
        current_program.set_viewport(&gl, viewport);
        current_program.set_zoom(&gl, zoom);
        current_program.set_center(&gl, center);
        current_program.set_quality(&gl, i32::from(quality));
        gl.draw_elements(&ELEMENTS);

        window.swap_buffers().unwrap();
        window.check_events().unwrap();
        old_pointer = pointer;
    }
}
