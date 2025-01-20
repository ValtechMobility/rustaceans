use std::path::Path;
use std::ptr;

use crate::debug::check_gl_error;
use crate::game::Game;
use crate::program::ShaderProgram;
use crate::set_attribute;
use crate::shader::{Shader, ShaderError};
use crate::texture::Texture;
use crate::vertex::{Buffer, Vertex, VertexArray};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec2 vertexTexCoord;

out vec2 texCoord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    texCoord = vertexTexCoord;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec2 texCoord;
uniform sampler2D texture0;
void main() {
    FragColor = texture(texture0, texCoord);
}
"#;

const VERTICES: [Vertex; 4] = [
    Vertex([-0.5, -0.5], [0.0, 1.0]),
    Vertex([0.5, -0.5], [1.0, 1.0]),
    Vertex([0.5, 0.5], [1.0, 0.0]),
    Vertex([-0.5, 0.5], [0.0, 0.0]),
];

const INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

#[derive(Debug)]
pub enum RendererError {
    ResizeError,
}

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    _index_buffer: Buffer,
    vertex_array: VertexArray,
    texture: Texture,
    viewport_width: i32,
    viewport_height: i32,
    position: [f32; 2],
}

impl Renderer {
    pub fn new(initial_width: i32, initial_height: i32) -> Result<Self, ShaderError> {
        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            program.apply();
            program.set_uniform_1i("texture0", 0)?;

            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&INDICES, gl::STATIC_DRAW);

            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);

            let tex_coord_attrib = program.get_attrib_location("vertexTexCoord")?;
            set_attribute!(vertex_array, tex_coord_attrib, Vertex::1);

            let mut texture = Texture::new();
            texture.load(&Path::new("assets/ferris.png")).map_err(|e| {
                println!("Error: {e:?}");
                ShaderError::TextureLoadError
            })?;

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            let mut renderer = Self {
                program,
                _vertex_buffer: vertex_buffer,
                _index_buffer: index_buffer,
                vertex_array,
                texture,
                viewport_width: initial_width,
                viewport_height: initial_height,
                position: [0.0, 0.0],
            };

            println!(
                "Initial renderer setup with dimensions: {}x{}",
                initial_width, initial_height
            );
            renderer
                .resize(initial_width, initial_height)
                .expect("Hope this is also alwaya valid.");

            Ok(renderer)
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), RendererError> {
        if width <= 0 || height <= 0 {
            println!("Invalid resize dimensions, querying actual window size...");
            return Err(RendererError::ResizeError);
        };

        unsafe {
            println!(
                "Resizing: window {}x{}, texture {}x{}",
                width, height, self.texture.width, self.texture.height
            );

            self.viewport_width = width;
            self.viewport_height = height;

            gl::Viewport(0, 0, width, height);
            check_gl_error("Viewport");

            let texture_aspect = self.texture.width as f32 / self.texture.height as f32;
            let window_aspect = width as f32 / height as f32;

            println!(
                "Aspects: texture {}, window {}",
                texture_aspect, window_aspect
            );

            let (scale_x, scale_y) = if window_aspect > texture_aspect {
                (texture_aspect / window_aspect, 1.0)
            } else {
                (1.0, window_aspect / texture_aspect)
            };

            println!("Scale factors: x={}, y={}", scale_x, scale_y);

            let vertices = [
                Vertex([-0.5 * scale_x, -0.5 * scale_y], [0.0, 1.0]),
                Vertex([0.5 * scale_x, -0.5 * scale_y], [1.0, 1.0]),
                Vertex([0.5 * scale_x, 0.5 * scale_y], [1.0, 0.0]),
                Vertex([-0.5 * scale_x, 0.5 * scale_y], [0.0, 0.0]),
            ];

            self._vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
            check_gl_error("Buffer update");
        }
        Ok(())
    }

    pub fn update(&mut self, game: &Game) {
        self.position[0] = game.player.x_position;
        self.position[1] = game.player.y_position;

        self.update_vertices();
    }

    fn update_vertices(&self) {
        unsafe {
            let vertices = [
                Vertex([self.position[0] - 0.5, self.position[1] - 0.5], [0.0, 1.0]),
                Vertex([self.position[0] + 0.5, self.position[1] - 0.5], [1.0, 1.0]),
                Vertex([self.position[0] + 0.5, self.position[1] + 0.5], [1.0, 0.0]),
                Vertex([self.position[0] - 0.5, self.position[1] + 0.5], [0.0, 0.0]),
            ];

            self._vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind();

            self.program.apply();
            self.vertex_array.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }
        }
    }
}
