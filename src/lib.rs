extern crate libc;
extern crate drm_rs;

use std::fs::File;
use std::os::unix::io::{RawFd, AsRawFd};
use std::slice;

use drm_rs::xf86drm_mode::{
    drmModeConnection,
    drmModeConnector,
    drmModeFreeConnector,
    drmModeFreeProperty,
    drmModeFreeResources,
    drmModeGetConnector,
    drmModeGetProperty,
    drmModeGetResources,
    drmModeModeInfo,
    drmModePropertyRes,
    drmModeRes,
};


pub struct Resources<'a> {
    pub raw: &'a mut drmModeRes,
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
    pub raw: &'a mut drmModeConnector,
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

    pub fn property_ids(&self) -> &[u32] {
        unsafe { slice::from_raw_parts(self.raw.props,
            self.raw.count_props as usize) }
    }

    pub fn property_id_value_pairs(&'a self) -> std::iter::Zip<std::slice::Iter<'a, u32>, std::slice::Iter<'a, u64>> {
        let ids = unsafe { slice::from_raw_parts(self.raw.props,
            self.raw.count_props as usize) };
        let values = unsafe { slice::from_raw_parts(self.raw.prop_values,
            self.raw.count_props as usize) };

        ids.iter().zip(values)
    }
}

impl<'a> Drop for Connector<'a> {
    fn drop(&mut self) {
        unsafe { drmModeFreeConnector(self.raw) };
    }
}

pub struct Property<'a> {
    pub raw: &'a mut drmModePropertyRes,
}

impl<'a> Property<'a> {
    pub fn try_from_raw_fd_and_id(fd: RawFd, id: u32) -> Option<Self> {
        match unsafe { drmModeGetProperty(fd, id).as_mut() } {
            Some(property) => Some(Property{raw: property}),
            None => None,
        }
    }

    pub fn try_from_file_and_id(file: &File, id: u32) -> Option<Self> {
        Property::try_from_raw_fd_and_id(file.as_raw_fd(), id)
    }
}

impl<'a> Drop for Property<'a> {
    fn drop(&mut self) {
        unsafe { drmModeFreeProperty(self.raw) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
