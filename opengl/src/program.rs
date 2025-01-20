use std::ffi::CString;

use crate::shader::{Shader, ShaderError};
use gl::types::*;

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            println!("Error: {:?}", String::from_utf8(error_log));
            Err(ShaderError::LinkingError)
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLint, ShaderError> {
        let attrib = CString::new(attrib).map_err(|_| ShaderError::NulError)?;
        let location = gl::GetAttribLocation(self.id, attrib.as_ptr());
        if location == -1 {
            println!(
                "Warning: Attribute '{}' not found in shader program",
                attrib.to_string_lossy()
            );
            return Err(ShaderError::AttributeNotFound);
        }
        Ok(location)
    }

    pub unsafe fn set_uniform_1i(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        let name = CString::new(name).map_err(|_| ShaderError::NulError)?;
        let location = gl::GetUniformLocation(self.id, name.as_ptr());
        if location == -1 {
            println!(
                "Warning: Uniform '{}' not found in shader program",
                name.to_string_lossy()
            );
            return Err(ShaderError::UniformNotFound);
        }
        gl::Uniform1i(location, value);
        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
