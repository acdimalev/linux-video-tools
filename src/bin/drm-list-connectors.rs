extern crate libc;
extern crate drm_rs;

use std::fs::File;
use std::os::unix::io::{RawFd, AsRawFd};
use std::slice;

use drm_rs::xf86drm_mode::{
    drmModeConnection,
    drmModeConnector,
    drmModeFreeConnector,
    drmModeFreeResources,
    drmModeGetConnector,
    drmModeGetResources,
    drmModeRes,
    drmModeSubPixel,
    DRM_MODE_CONNECTOR_UNKNOWN,
    DRM_MODE_CONNECTOR_VGA,
    DRM_MODE_CONNECTOR_DVII,
    DRM_MODE_CONNECTOR_DVID,
    DRM_MODE_CONNECTOR_DVIA,
    DRM_MODE_CONNECTOR_COMPOSITE,
    DRM_MODE_CONNECTOR_SVIDEO,
    DRM_MODE_CONNECTOR_LVDS,
    DRM_MODE_CONNECTOR_COMPONENT,
    DRM_MODE_CONNECTOR_9PINDIN,
    DRM_MODE_CONNECTOR_DISPLAYPORT,
    DRM_MODE_CONNECTOR_HDMIA,
    DRM_MODE_CONNECTOR_HDMIB,
    DRM_MODE_CONNECTOR_TV,
    DRM_MODE_CONNECTOR_EDP,
    DRM_MODE_CONNECTOR_VIRTUAL,
    DRM_MODE_CONNECTOR_DSI,
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
}

impl<'a> Drop for Connector<'a> {
    fn drop(&mut self) {
        unsafe { drmModeFreeConnector(self.raw) };
    }
}

/// List connector information related to a DRI device.
///
/// FIXME -- should accept device file as parameter.

fn main() {
    let file = File::open("/dev/dri/card0").unwrap();

    let resources = Resources::try_from_file(&file).unwrap();

    for connector_id in resources.connector_ids() {
        println!("connector: {}", connector_id);

        let connector = Connector::try_from_file_and_id(&file, *connector_id)
            .unwrap();

        println!("  type: {}", match connector.raw.connector_type as i32 {
            DRM_MODE_CONNECTOR_UNKNOWN     => "unknown",
            DRM_MODE_CONNECTOR_VGA         => "vga",
            DRM_MODE_CONNECTOR_DVII        => "dvi-i",
            DRM_MODE_CONNECTOR_DVID        => "dvi-d",
            DRM_MODE_CONNECTOR_DVIA        => "dvi-a",
            DRM_MODE_CONNECTOR_COMPOSITE   => "composite",
            DRM_MODE_CONNECTOR_SVIDEO      => "s-video",
            DRM_MODE_CONNECTOR_LVDS        => "lvds",
            DRM_MODE_CONNECTOR_COMPONENT   => "component",
            DRM_MODE_CONNECTOR_9PINDIN     => "9 pin din",
            DRM_MODE_CONNECTOR_DISPLAYPORT => "displayport",
            DRM_MODE_CONNECTOR_HDMIA       => "hdmi a",
            DRM_MODE_CONNECTOR_HDMIB       => "hdmi b",
            DRM_MODE_CONNECTOR_TV          => "tv",
            DRM_MODE_CONNECTOR_EDP         => "edp",
            DRM_MODE_CONNECTOR_VIRTUAL     => "virtual",
            DRM_MODE_CONNECTOR_DSI         => "dsi",
            _                              => "-- invalid --",
        });
        println!("  ({} modes, {} props, {} encoders)",
            connector.raw.count_modes,
            connector.raw.count_props,
            connector.raw.count_encoders,
        );
        println!("  status: {}", match connector.raw.connection {
            drmModeConnection::DRM_MODE_CONNECTED         => "connected",
            drmModeConnection::DRM_MODE_DISCONNECTED      => "disconnected",
            drmModeConnection::DRM_MODE_UNKNOWNCONNECTION => "unknown",
        });
        if connector.connected() {
            println!("    {}x{}mm",
                connector.raw.mmWidth,
                connector.raw.mmHeight,
            );
            println!("    current encoder: {}", connector.raw.encoder_id);
            println!("    subpixel: {}", match connector.raw.subpixel {
                drmModeSubPixel::DRM_MODE_SUBPIXEL_UNKNOWN
                    => "unknown",
                drmModeSubPixel::DRM_MODE_SUBPIXEL_HORIZONTAL_RGB
                    => "horizontal rgb",
                drmModeSubPixel::DRM_MODE_SUBPIXEL_HORIZONTAL_BGR
                    => "horizontal bgr",
                drmModeSubPixel::DRM_MODE_SUBPIXEL_VERTICAL_RGB
                    => "vertical rgb",
                drmModeSubPixel::DRM_MODE_SUBPIXEL_VERTICAL_BGR
                    => "vertical bgr",
                drmModeSubPixel::DRM_MODE_SUBPIXEL_NONE
                    => "none",
            });
        }
    }
}
