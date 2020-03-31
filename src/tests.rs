use crate::load_sapling_parameters;

use std::fs::File;
use std::io::Read;

const SAPLING_SPEND_PATH: &str = "src/sapling-spend.params";
const SAPLING_OUTPUT_PATH: &str = "src/sapling-output.params";

fn original_parameters() -> (Vec<u8>, Vec<u8>) {
    let mut spend_fs = File::open(SAPLING_SPEND_PATH).expect("couldn't load Sapling spend parameters file");
    let mut output_fs = File::open(SAPLING_OUTPUT_PATH).expect("couldn't load Sapling output parameters file");

    let mut spend_buffer = Vec::new();
    let mut output_buffer = Vec::new();

    spend_fs.read_to_end(&mut spend_buffer).expect("couldn't read Sapling spend parameters file to buffer");
    output_fs.read_to_end(&mut output_buffer).expect("couldn't read Sapling output parameters file to buffer");

    (spend_buffer, output_buffer)
}

#[test]
pub fn test_parameter_reconstruction() {
    let (spend, output) = original_parameters();
    let (spend_reconstructed, output_reconstructed) = load_sapling_parameters();

    assert_eq!(spend.len(), spend_reconstructed.len());
    assert_eq!(output.len(), output_reconstructed.len());

    for i in 0..spend.len() {
        assert_eq!(spend[i], spend_reconstructed[i]);
    }
    for i in 0..output.len() {
        assert_eq!(output[i], output_reconstructed[i]);
    }

    println!("Parameter reconstruction was successful!");
}