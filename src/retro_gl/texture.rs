use super::{gl::gl, pixel::Pixel};
use crate::video::RawTextureData;
use gl::types::GLuint;
use retro_ab::{core::AvInfo, erro_handle::ErroHandle};
use std::{ptr::null, rc::Rc, sync::Arc};

pub type TexturePosition = [f32; 2];

pub struct Texture2D {
    id: GLuint,
    pixel: Pixel,
    gl: Rc<gl::Gl>,
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &self.id) }
    }
}

impl Texture2D {
    pub fn active(&self) {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn push(&self, raw_data: &RawTextureData) {
        let param = raw_data.pitch as i32 / self.pixel.bpm;

        unsafe {
            self.gl.BindTexture(gl::TEXTURE0, self.id);
            self.gl.PixelStorei(gl::UNPACK_ROW_LENGTH, param);
            self.gl.TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                raw_data.width as i32,
                raw_data.height as i32,
                self.pixel.typ,
                self.pixel.format,
                raw_data.data,
            );
            self.gl.BindTexture(gl::TEXTURE0, 0);
        }
    }

    pub fn get_id(&self) -> GLuint {
        return self.id.clone();
    }

    pub fn new(av_info: &Arc<AvInfo>, gl: Rc<gl::Gl>) -> Result<Texture2D, ErroHandle> {
        let mut id = 0;
        let geo = &av_info.video.geometry;
        let pixel = Pixel::new(&av_info.video.pixel_format.lock().unwrap())?;

        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                *geo.max_width.read().unwrap() as i32,
                *geo.max_height.read().unwrap() as i32,
                0,
                pixel.typ,
                pixel.format,
                null(),
            );

            gl.BindTexture(gl::TEXTURE_2D, 0);

            Ok(Texture2D { id, pixel, gl })
        }
    }
}
