extern crate drm_rs;
extern crate linux_video_tools;

use std::fs::File;

use linux_video_tools::{
    Resources,
    Connector,
};


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

        for mode in connector.modes() {
            println!("    {}x{} @ {}", mode.hdisplay, mode.vdisplay, mode.vrefresh);
        }
    }
}
