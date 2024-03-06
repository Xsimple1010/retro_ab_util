use retro_ab::core::AvInfo;
use sdl2::{
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};
use std::{ffi::c_uint, os::raw::c_void, ptr::null, sync::Arc};

use crate::retro_gl::render::{NextFrame, Render};

//
static mut RAW_TEX_POINTER: NextFrame = NextFrame {
    _data: null(),
    _pitch: 0,
    _height: 0,
    _width: 0,
};

pub fn video_refresh_callback(
    _data: *const c_void,
    _width: c_uint,
    _height: c_uint,
    _pitch: usize,
) {
    unsafe {
        RAW_TEX_POINTER._data = _data;
        RAW_TEX_POINTER._height = _height;
        RAW_TEX_POINTER._width = _width;
        RAW_TEX_POINTER._pitch = _pitch;
    }
}

pub struct RetroVideo {
    _video: VideoSubsystem,
    _window: Window,
    _gl_ctx: GLContext,
    _render: Render,
}

impl RetroVideo {
    pub fn draw_new_frame(&mut self) {
        unsafe {
            self._render.draw_new_frame(&RAW_TEX_POINTER);
        }

        self._window.gl_swap_window();
        // for _ in 0..45_900_00 {}
    }

    pub fn resize(&mut self, _new_size: (u32, u32)) {}
}

pub fn init(sdl: &Sdl, av_info: &Arc<AvInfo>) -> Result<RetroVideo, String> {
    let _video = sdl.video()?;

    let gl_attr = _video.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let geo = &av_info.video.geometry;

    let win_result = _video
        .window(
            "retro_ab_av",
            *geo.base_width.lock().unwrap(),
            *geo.base_height.lock().unwrap(),
        )
        .opengl()
        .position_centered()
        .build();

    match win_result {
        Ok(_window) => {
            let _gl_ctx = _window.gl_create_context().unwrap();
            gl::load_with(|name| _video.gl_get_proc_address(name) as *const _);
            _video.gl_set_swap_interval(1)?;

            let mut _render = Render::new(av_info).expect("erro ao tentar inicar o opengl");

            unsafe {
                gl::ClearColor(0., 0., 0., 0.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            _window.gl_swap_window();

            Ok(RetroVideo {
                _video,
                _window,
                _gl_ctx,
                _render,
            })
        }
        Err(e) => Err(e.to_string()),
    }
}
