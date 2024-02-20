use crate::stage::Stage;

use super::{Renderer, RendererCoord};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::CompressedTexture2d;
use glium::{implement_vertex, uniform, Frame, Program, Surface, VertexBuffer};
use std::path::Path;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Renderer2D {
    index_buffer: NoIndices,
    program: Program,
    texture: CompressedTexture2d,
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
    pub fn new(stage: &Stage, filename: &str) -> Self {
        let image = image::open(Path::new(filename)).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        // Vertices will be anchored to top-left and spaced out with a 1x scale of the texture's dimensions
        let RendererCoord { x: left, y: top } = stage
            .viewport
            .convert_point_to_renderer_coord(&stage.viewport.top_left());
        let RendererCoord {
            x: right,
            y: bottom,
        } = stage
            .viewport
            .convert_point_to_renderer_coord(&(stage.viewport.top_left() + image_dimensions));

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
            texture: CompressedTexture2d::new(&stage.display, image).unwrap(),
        }
    }
}

impl Renderer for Renderer2D {
    fn draw(&self, frame: &mut Frame) -> () {
        let x = 0.0;

        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [ x , 0.0, 0.0, 1.0f32],
                    ],
                    tex: &self.texture,
                },
                &Default::default(),
            )
            .unwrap();
    }
}
