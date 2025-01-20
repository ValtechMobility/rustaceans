use std::path::Path;

use crate::debug::check_gl_error;
use gl::types::GLuint;
use image::EncodableLayout;

#[derive(Debug)]
pub enum TextureError {
    LoadError,
}

pub struct Texture {
    pub id: GLuint,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenTextures(1, &mut id);
        Self {
            id,
            width: 0,
            height: 0,
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn load(&mut self, path: &Path) -> Result<(), TextureError> {
        self.bind();
        check_gl_error("Texture bind");

        let img = image::open(path)
            .map_err(|e| {
                println!("Error: {e:?}");
                TextureError::LoadError
            })?
            .into_rgba8();

        println!("Image loaded: {}x{}", img.width(), img.height(),);
        self.width = img.width() as i32;
        self.height = img.height() as i32;
        if self.width <= 0 || self.height <= 0 {
            println!("Warning: Invalid texture dimensions!");
            return Err(TextureError::LoadError);
        }

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr() as *const _,
        );
        check_gl_error("TexImage2D");

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        check_gl_error("Texture parameters");

        gl::GenerateMipmap(gl::TEXTURE_2D);
        check_gl_error("Generate mipmap");

        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
