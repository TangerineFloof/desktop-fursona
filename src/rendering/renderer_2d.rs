use std::ops::Deref;
use std::rc::Rc;

use crate::stage::Stage;

use super::{Renderer, RendererRect};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::CompressedTexture2d;
use glium::{implement_vertex, uniform, Frame, Program, Surface, VertexBuffer};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Renderer2D {
    index_buffer: NoIndices,
    program: Program,
    texture: Option<Rc<CompressedTexture2d>>,
    vertex_buffer: VertexBuffer<Vertex>,
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 100
uniform lowp mat4 matrix;
attribute lowp vec2 position;
attribute lowp vec2 tex_coords;
varying lowp vec2 v_tex_coords;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    v_tex_coords = tex_coords;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 100
uniform lowp sampler2D tex;
varying lowp vec2 v_tex_coords;

void main() {
    gl_FragColor = texture2D(tex, v_tex_coords);
}
"#;

impl Renderer2D {
    pub fn new(stage: &Stage) -> Self {
        let left = 0.0f32;
        let top = 0.0f32;
        let right = 1.0f32;
        let bottom = -1.0f32;

        let vertex_buffer = VertexBuffer::new(
            &stage.display,
            &[
                Vertex {
                    // BL
                    position: [left, bottom],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    // BR
                    position: [right, bottom],
                    tex_coords: [1.0, 0.0],
                },
                Vertex {
                    // TR
                    position: [right, top],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    // TR
                    position: [right, top],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    // TL
                    position: [left, top],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    // BL
                    position: [left, bottom],
                    tex_coords: [0.0, 0.0],
                },
            ],
        )
        .unwrap();

        let index_buffer = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(
            &stage.display,
            VERTEX_SHADER_SOURCE,
            FRAGMENT_SHADER_SOURCE,
            None,
        )
        .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            program,
            texture: None,
        }
    }

    pub fn set_texture(&mut self, texture: Rc<CompressedTexture2d>) {
        self.texture = Some(texture);
    }
}

impl Renderer for Renderer2D {
    fn draw(&self, frame: &mut Frame, rect: RendererRect) -> () {
        let texture = self.texture.as_ref().unwrap();

        // Multiply size by 2.0 because the vertices are sized to take up
        // one quadrant of the renderer system, but the incoming rect size
        // goes from 0.0 -> 1.0 for the whole coordinate system.
        let scale_x = rect.width * 2.0;
        let scale_y = rect.height * 2.0;

        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniform! {
                    matrix: [
                        [ scale_x,     0.0, 0.0, 0.0],
                        [     0.0, scale_y, 0.0, 0.0],
                        [     0.0,     0.0, 1.0, 0.0],
                        [  rect.x,  rect.y, 0.0, 1.0f32],
                    ],
                    tex: texture.deref(),
                },
                &Default::default(),
            )
            .unwrap();
    }
}
