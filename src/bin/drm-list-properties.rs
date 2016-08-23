extern crate drm_rs;
extern crate linux_video_tools;

use std::env;
use std::fs::File;
use std::ffi::CStr;

use linux_video_tools::{
    Resources,
    Connector,
    Property,
};


fn main() {
    let path = env::args().nth(1).unwrap_or("/dev/dri/card0".to_owned());

    let file = File::open(path).unwrap();

    let resources = Resources::try_from_file(&file).unwrap();

    for connector_id in resources.connector_ids() {
        let connector = Connector::try_from_file_and_id(&file, *connector_id)
            .unwrap();

        if ! connector.connected() {
            println!("Connector {} is disconnected", connector_id);
            continue;
        }

        println!("Connector {} has properties...", connector_id);

        // for property_id in connector.property_ids() {
        for (property_id, property_value) in connector.property_id_value_pairs() {
            let property = Property::try_from_file_and_id(&file, *property_id)
                .unwrap();

            let property_name = unsafe { String::from(CStr::from_ptr(&property.raw.name[0]).to_string_lossy().into_owned()) };
            println!("    {} ({}): {}", property_name, property_id, property_value);
            println!("        ({} values, {} enums, {} blobs)", property.raw.count_values, property.raw.count_enums, property.raw.count_blobs);
        }
    }
}
