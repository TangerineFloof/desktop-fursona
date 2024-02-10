use glium::index::{NoIndices, PrimitiveType};
use glium::{implement_vertex, Display, Frame, Program, Surface, VertexBuffer};
use glutin::surface::WindowSurface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub struct Renderer {
    indices: NoIndices,
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
}

impl Renderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        Self {
            indices: NoIndices(PrimitiveType::TrianglesList),
            program: Program::from_source(
                display,
                VERTEX_SHADER_SOURCE,
                FRAGMENT_SHADER_SOURCE,
                None,
            )
            .unwrap(),
            vertex_buffer: VertexBuffer::new(display, &VERTEX_DATA).unwrap(),
        }
    }

    pub fn draw(&self, frame: &mut Frame) -> () {
        frame
            .draw(
                &self.vertex_buffer,
                self.indices,
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

static VERTEX_DATA: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];

const VERTEX_SHADER_SOURCE: &str = r#"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}"#;
