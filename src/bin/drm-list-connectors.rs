extern crate libc;
extern crate drm_rs;
extern crate linux_video_tools;

use std::env;
use std::fs::File;

use drm_rs::xf86drm_mode::{
    drmModeConnection,
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

use linux_video_tools::{
    Resources,
    Connector,
};


fn connector_type_str(connector: &Connector) -> &'static str {
    match connector.raw.connector_type as i32 {
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
    }
}

fn connector_connection_str(connector: &Connector) -> &'static str {
    match connector.raw.connection {
        drmModeConnection::DRM_MODE_CONNECTED         => "connected",
        drmModeConnection::DRM_MODE_DISCONNECTED      => "disconnected",
        drmModeConnection::DRM_MODE_UNKNOWNCONNECTION => "unknown",
    }
}

fn connector_subpixel_str(connector: &Connector) -> &'static str {
    match connector.raw.subpixel {
        drmModeSubPixel::DRM_MODE_SUBPIXEL_UNKNOWN        => "unknown",
        drmModeSubPixel::DRM_MODE_SUBPIXEL_HORIZONTAL_RGB => "horizontal rgb",
        drmModeSubPixel::DRM_MODE_SUBPIXEL_HORIZONTAL_BGR => "horizontal bgr",
        drmModeSubPixel::DRM_MODE_SUBPIXEL_VERTICAL_RGB   => "vertical rgb",
        drmModeSubPixel::DRM_MODE_SUBPIXEL_VERTICAL_BGR   => "vertical bgr",
        drmModeSubPixel::DRM_MODE_SUBPIXEL_NONE           => "none",
    }
}

fn main() {
    let path = env::args().nth(1).unwrap_or("/dev/dri/card0".to_owned());

    let file = File::open(path).unwrap();

    let resources = Resources::try_from_file(&file).unwrap();

    for connector_id in resources.connector_ids() {
        println!("connector: {}", connector_id);

        let connector = Connector::try_from_file_and_id(&file, *connector_id)
            .unwrap();

        println!("  type: {}", connector_type_str(&connector));
        println!("  ({} modes, {} props, {} encoders)",
            connector.raw.count_modes,
            connector.raw.count_props,
            connector.raw.count_encoders,
        );
        println!("  status: {}", connector_connection_str(&connector));
        if connector.connected() {
            println!("    {}x{}mm",
                connector.raw.mmWidth,
                connector.raw.mmHeight,
            );
            println!("    current encoder: {}", connector.raw.encoder_id);
            println!("    subpixel: {}", connector_subpixel_str(&connector));
        }
    }
}
