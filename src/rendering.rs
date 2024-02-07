use glow::{Context, HasContext, NativeBuffer, NativeProgram, NativeShader, VertexArray};
use glutin::display::{Display, GlDisplay};

pub struct Renderer {
    gl: Context,
    program: Option<NativeProgram>,
    vao: Option<VertexArray>,
    vbo: Option<NativeBuffer>,
}

impl Renderer {
    pub fn new(gl_display: &Display) -> Self {
        unsafe {
            let gl = Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));

            println!("Running on {}", gl.get_parameter_string(glow::RENDERER));
            println!("OpenGL Version {}", gl.get_parameter_string(glow::VERSION));
            println!(
                "Shaders version on {}",
                gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION)
            );

            let vertex_shader =
                create_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE).unwrap();
            let fragment_shader =
                create_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE).unwrap();

            let program = gl.create_program().unwrap();

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);

            gl.link_program(program);

            gl.use_program(Some(program));

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().expect("Cannot create buffer array");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let vertex_bytes: Vec<u8> = VERTEX_DATA.iter().flat_map(|f| f.to_ne_bytes()).collect();
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertex_bytes, glow::STATIC_DRAW);

            let pos_attrib = gl.get_attrib_location(program, "position").unwrap();
            let color_attrib = gl.get_attrib_location(program, "color").unwrap();
            gl.vertex_attrib_pointer_f32(
                pos_attrib,
                2,
                glow::FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                0,
            );
            gl.vertex_attrib_pointer_f32(
                color_attrib,
                3,
                glow::FLOAT,
                false,
                5 * std::mem::size_of::<f32>() as i32,
                (2 * std::mem::size_of::<f32>()) as i32,
            );

            gl.enable_vertex_attrib_array(pos_attrib);
            gl.enable_vertex_attrib_array(color_attrib);

            Self {
                gl,
                program: Some(program),
                vao: Some(vao),
                vbo: Some(vbo),
            }
        }
    }

    pub fn draw(&self) -> () {
        unsafe {
            self.gl.use_program(self.program);

            self.gl.bind_vertex_array(self.vao);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

            self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}

unsafe fn create_shader(gl: &Context, shader: u32, source: &str) -> Result<NativeShader, String> {
    let shader = gl.create_shader(shader)?;
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    Ok(shader)
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
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
