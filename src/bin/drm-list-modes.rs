extern crate drm_rs;

use std::fs::File;
use std::os::unix::io::{
    RawFd,
    AsRawFd,
};
use std::slice;
use std::ffi::CStr;

use drm_rs::xf86drm_mode::{
    drmModeRes,
    drmModeConnector,
    drmModeConnection,
    drmModeModeInfo,
    drmModeGetResources,
    drmModeFreeResources,
    drmModeGetConnector,
    drmModeFreeConnector,
};

pub struct Resources<'a> {
    raw: &'a mut drmModeRes,
}

impl<'a> Resources<'a> {
    pub fn try_from_raw_fd(fd: RawFd) -> Option<Self> {
        match unsafe { drmModeGetResources(fd).as_mut() } {
            Some(resources) => Some(Resources{raw: resources}),
            None => None,
        }
    }

    pub fn try_from_file(file: &File) -> Option<Self> {
        Resources::try_from_raw_fd(file.as_raw_fd())
    }

    pub fn connector_ids(&self) -> &[u32] {
        unsafe { slice::from_raw_parts(self.raw.connectors,
            self.raw.count_connectors as usize) }
    }
}

impl<'a> Drop for Resources<'a> {
    fn drop(&mut self) {
        unsafe { drmModeFreeResources(self.raw) };
    }
}

pub struct Connector<'a> {
    raw: &'a mut drmModeConnector,
}

impl<'a> Connector<'a> {
    pub fn try_from_raw_fd_and_id(fd: RawFd, id: u32) -> Option<Self> {
        match unsafe { drmModeGetConnector(fd, id).as_mut() } {
            Some(connector) => Some(Connector{raw: connector}),
            None => None,
        }
    }

    pub fn try_from_file_and_id(file: &File, id: u32) -> Option<Self> {
        Connector::try_from_raw_fd_and_id(file.as_raw_fd(), id)
    }

    pub fn connected(&self) -> bool {
        match &self.raw.connection {
            &drmModeConnection::DRM_MODE_CONNECTED => true,
            _ => false,
        }
    }

    pub fn modes(&self) -> &[drmModeModeInfo] {
        unsafe { slice::from_raw_parts(self.raw.modes,
            self.raw.count_modes as usize) }
    }
}

impl<'a> Drop for Connector<'a> {
    fn drop(&mut self) {
        unsafe { drmModeFreeConnector(self.raw) };
    }
}

fn main() {
    let path = "/dev/dri/card0";
    let file = File::open(path).unwrap();

    let resources = Resources::try_from_file(&file).unwrap();

    for connector_id in resources.connector_ids() {
        let connector = Connector::try_from_file_and_id(&file, *connector_id)
            .unwrap();

        if ! connector.connected() {
            println!("Connector {} is disconnected", connector_id);
            continue;
        }

        println!("Connector {} has modes...", connector_id);

        let modes = connector.modes();

        for mode in modes {
            println!("    {}x{} @ {}", mode.hdisplay, mode.vdisplay, mode.vrefresh);
        }
    }
}
