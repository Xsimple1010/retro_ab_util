/* automatically generated by rust-bindgen 0.69.4 */

pub type RustVideoRefreshT = ::std::option::Option<
    unsafe extern "C" fn(
        extra_data: *mut ::std::os::raw::c_void,
        data: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
        pitch: usize,
    ),
>;
pub type RustAudioSampleT = ::std::option::Option<
    unsafe extern "C" fn(extra_data: *mut ::std::os::raw::c_void, left: i16, right: i16),
>;
pub type RustAudioSampleTBatchT = ::std::option::Option<
    unsafe extern "C" fn(
        extra_data: *mut ::std::os::raw::c_void,
        data: *const i16,
        frames: usize,
    ) -> usize,
>;
extern "C" {
    #[doc = "  essa função deve ser enviada para o núcleo"]
    pub fn core_video_refresh(
        data: *const ::std::os::raw::c_void,
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
        pitch: usize,
    );
}
extern "C" {
    pub fn core_audio_sample(left: i16, right: i16);
}
extern "C" {
    pub fn core_audio_sample_batch(data: *const i16, frames: usize) -> usize;
}
extern "C" {
    #[doc = " tem que ser chamada antes que qualquer callback ser chamada"]
    pub fn set_video_extra_data(extra_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn set_audio_extra_data(extra_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn set_rust_video_refresh(cb: RustVideoRefreshT);
}
extern "C" {
    pub fn set_rust_audio_sample(cb: RustAudioSampleT);
}
extern "C" {
    pub fn set_rust_audio_sample_batch(cb: RustAudioSampleTBatchT);
}
extern "C" {
    pub fn de_init_all_callbacks();
}
