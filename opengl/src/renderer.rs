use crate::program::ShaderProgram;
use crate::set_attribute;
use crate::shader::{Shader, ShaderError};
use crate::shapes::vertex::{Buffer, Vertex, VertexArray};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec3 color;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
uniform float time;
out vec4 FragColor;
in vec3 vertexColor;

void main() {
    // Erzeuge Farbverläufe mit Sinuswellen
    float r = 0.5 + 0.5 * sin(time + vertexColor.x * 10.0);
    float g = 0.5 + 0.5 * sin(time + vertexColor.y * 10.0);
    float b = 0.5 + 0.5 * sin(time + vertexColor.z * 10.0);

    // Kombiniere die Farben für ein pulsierendes Ergebnis
    FragColor = vec4(r, g, b, 1.0);
}
"#;

const VERTICES: [Vertex; 3] = [
    Vertex([0.5, 0.5], [1.0, 0.0, 0.0]),
    Vertex([1.0, -1.0], [0.0, 1.0, 0.0]),
    Vertex([-1.0, -1.0], [0.0, 0.0, 1.0]),
];

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    vertex_array: VertexArray,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
            let vertex_array = VertexArray::new();
            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, Vertex::1);

            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                vertex_array,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            self.vertex_array.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
