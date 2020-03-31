use std::fs::File;
use std::io::Read;

pub fn load_partial_parameters() -> Vec<u8> {
    let mut spend_fs = File::open("wagyu-zcash-parameters-1/src/sapling-spend-1.params").expect("couldn't load Sapling spend parameters file");
    let mut spend_buffer = Vec::new();
    spend_fs.read_to_end(&mut spend_buffer).expect("couldn't read Sapling spend parameters file to buffer");
    spend_buffer
}
