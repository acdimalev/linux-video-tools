extern crate libc;
extern crate drm_rs;

use std::fs::File;
use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};

use drm_rs::xf86drm_mode::{drmModeGetResources,
    drmModeFreeResources};


/// Rust struct for drmModeRes
/// 
/// I can't figure out a sane way to manage
/// the C-allocated data structures.
///
/// So I'm just copying data out.

pub struct ModeRes {
    pub fb_ids: Vec<u32>,
    pub crtc_ids: Vec<u32>,
    pub connector_ids: Vec<u32>,
    pub encoder_ids: Vec<u32>,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}


impl ModeRes {

    /// Loads DRM Mode Resources from device file.
    ///
    /// FIXME -- Should return a Result (or Option).
    ///
    /// FIXME -- Clean up after `into_raw_fd`.
    ///
    /// I'm also not too sure that this is an appropriate use
    /// of the `From` construct.

    fn from_file(file: File) -> Self {
        return unsafe { ModeRes::from_raw_fd(file.into_raw_fd()) };
    }
}


/// Create a Vec<T> from a C-style array

unsafe fn vec_from_array<T>(array: *const T, count: isize)
    -> Vec<T> {

    let mut vec = Vec::new();

    for i in 0..count {
        vec.push(std::ptr::read(array.offset(i)));
    }

    vec
}


impl FromRawFd for ModeRes {
    /// Loads DRM Mode Resources from a raw file descriptor.
    ///
    /// FIXME -- Should return a Result (or Option).

    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        let resources_ptr = drmModeGetResources(fd);
        let r = resources_ptr.as_ref().unwrap();

        let mode_res = ModeRes {
            fb_ids: vec_from_array(r.fbs,
                r.count_fbs as isize),
            crtc_ids: vec_from_array(r.crtcs,
                r.count_crtcs as isize),
            connector_ids: vec_from_array(r.connectors,
                r.count_connectors as isize),
            encoder_ids: vec_from_array(r.encoders,
                r.count_encoders as isize),
            min_width: r.min_width,
            max_width: r.max_width,
            min_height: r.min_height,
            max_height: r.max_height,
        };

        drmModeFreeResources(resources_ptr);

        mode_res
    }
}


/// List connector information related to a DRI device.
///
/// FIXME -- should accept device file as parameter.

fn main() {
    let file = File::open("/dev/dri/card0").unwrap();

    let resources = ModeRes::from_file(file);

    for connector_id in resources.connector_ids {
        println!("connector: {}", connector_id);
    }
}
