mod fdetector;
use machineid_rs::{Encryption, IdBuilder};

fn get_unique_identifier() -> Option<Vec<u8>> {
    let mut builder = IdBuilder::new(Encryption::SHA1);
    builder.add_component(machineid_rs::HWIDComponent::SystemID);

    return match builder.build("id") {
        Ok(v) => Some(v.as_bytes().to_vec()),
        Err(_) => None,
    };
}

fn main() {
    let id = match get_unique_identifier() {
        Some(id) => id,
        None => return,
    };

}
