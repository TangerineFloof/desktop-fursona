use crate::rendering::{Color, RendererRect};
use glium::index::{NoIndices, PrimitiveType};
use glium::{
    implement_vertex, uniform, Display, DrawParameters, Frame, Program, Surface, VertexBuffer,
};
use glutin::surface::WindowSurface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    internal_pos: [f32; 2],
}
implement_vertex!(Vertex, position, internal_pos);

pub struct SquareRenderer {
    index_buffer: NoIndices,
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
}

const VERTEX_SHADER_SOURCE: &str = r#"
#version 100
uniform lowp mat4 matrix;
attribute lowp vec2 position;
attribute lowp vec2 internal_pos;
varying lowp vec2 v_internal_pos;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    v_internal_pos = internal_pos;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 100
uniform lowp vec4 color;
uniform lowp vec2 thickness;
varying lowp vec2 v_internal_pos;

void main() {
    if (
        v_internal_pos.x <= thickness.x ||
        v_internal_pos.x >= 1.0 - thickness.x ||
        v_internal_pos.y <= thickness.y ||
        v_internal_pos.y >= 1.0 - thickness.y
    ) {
        gl_FragColor = color;
    } else {
        discard;
    }
}
"#;

impl SquareRenderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let left = 0.0f32;
        let top = 0.0f32;
        let right = 1.0f32;
        let bottom = -1.0f32;

        let vertex_buffer = VertexBuffer::new(
            display,
            &[
                Vertex {
                    // BL
                    position: [left, bottom],
                    internal_pos: [0.0, 0.0],
                },
                Vertex {
                    // BR
                    position: [right, bottom],
                    internal_pos: [1.0, 0.0],
                },
                Vertex {
                    // TR
                    position: [right, top],
                    internal_pos: [1.0, 1.0],
                },
                Vertex {
                    // TR
                    position: [right, top],
                    internal_pos: [1.0, 1.0],
                },
                Vertex {
                    // TL
                    position: [left, top],
                    internal_pos: [0.0, 1.0],
                },
                Vertex {
                    // BL
                    position: [left, bottom],
                    internal_pos: [0.0, 0.0],
                },
            ],
        )
        .unwrap();

        let index_buffer = NoIndices(PrimitiveType::TrianglesList);

        let program =
            Program::from_source(display, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None)
                .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    pub fn draw(
        &self,
        frame: &mut Frame,
        square: RendererRect,
        color: &Color,
        // Thickness measure from 0.0 to 1.0 and is a percentage of the
        // provided square in each dimension. A thickness of (0.2, 0.2)
        // means the square will be 20% of the provided square on each
        // side
        thickness: (f32, f32),
        base_draw_parameters: &DrawParameters,
    ) -> () {
        // Multiply size by 2.0 because the vertices are sized to take up
        // one quadrant of the renderer system, but the incoming rect size
        // goes from 0.0 -> 1.0 for the whole coordinate system.
        let scale_x = square.width * 2.0;
        let scale_y = square.height * 2.0;

        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniform! {
                    matrix: [
                        [   scale_x,       0.0, 0.0, 0.0],
                        [       0.0,   scale_y, 0.0, 0.0],
                        [       0.0,       0.0, 1.0, 0.0],
                        [  square.x,  square.y, 0.0, 1.0f32],
                    ],
                    color: [color.0, color.1, color.2, color.3],
                    thickness: [thickness.0, thickness.1]
                },
                base_draw_parameters,
            )
            .unwrap();
    }
}
