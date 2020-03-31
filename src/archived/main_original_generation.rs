///
/// This program was used to generate the original partial parameters used by the crates
/// in this workspace.
///

use std::fs::File;
use std::io::{Read, Write};

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

fn construct_partial_parameters() {
    let (spend, output) = original_parameters();

    println!("Number of bytes in Sapling spend parameter: {:?}", spend.len());
    println!("Number of bytes in Sapling output parameter: {:?}", output.len());

    let mut spend_fs1 = File::create("src/sapling-spend-1.params").expect("couldn't create Sapling spend 1 parameters file");
    let mut spend_fs2 = File::create("src/sapling-spend-2.params").expect("couldn't create Sapling spend 2 parameters file");
    let mut spend_fs3 = File::create("src/sapling-spend-3.params").expect("couldn't create Sapling spend 3 parameters file");
    let mut spend_fs4 = File::create("src/sapling-spend-4.params").expect("couldn't create Sapling spend 4 parameters file");
    let mut spend_fs5 = File::create("src/sapling-spend-5.params").expect("couldn't create Sapling spend 5 parameters file");
    let mut output_fs1 = File::create("src/sapling-output-1.params").expect("couldn't create Sapling output 1 parameters file");

    let range1 = spend.len() / 5;
    let range2 = range1 * 2;
    let range3 = range1 * 3;
    let range4 = range1 * 4;

    spend_fs1.write_all(&spend[0..range1]).expect("couldn't read Sapling spend 1 parameters buffer to file");
    spend_fs2.write_all(&spend[range1..range2]).expect("couldn't read Sapling spend 2 parameters buffer to file");
    spend_fs3.write_all(&spend[range2..range3]).expect("couldn't read Sapling spend 3 parameters buffer to file");
    spend_fs4.write_all(&spend[range3..range4]).expect("couldn't read Sapling spend 4 parameters buffer to file");
    spend_fs5.write_all(&spend[range4..]).expect("couldn't read Sapling spend 5 parameters buffer to file");
    output_fs1.write_all(&output).expect("couldn't read Sapling output 1 parameters buffer to file");
}

fn reconstruct_parameters() -> (Vec<u8>, Vec<u8>) {
    let mut spend_fs1 = File::open("src/sapling-spend-1.params").expect("couldn't load Sapling spend 1 parameters file");
    let mut spend_fs2 = File::open("src/sapling-spend-2.params").expect("couldn't load Sapling spend 2 parameters file");
    let mut spend_fs3 = File::open("src/sapling-spend-3.params").expect("couldn't load Sapling spend 3 parameters file");
    let mut spend_fs4 = File::open("src/sapling-spend-4.params").expect("couldn't load Sapling spend 4 parameters file");
    let mut spend_fs5 = File::open("src/sapling-spend-5.params").expect("couldn't load Sapling spend 5 parameters file");
    let mut output_fs1 = File::open("src/sapling-output-1.params").expect("couldn't load Sapling output 1 parameters file");

    let mut spend_buffer1 = Vec::new();
    let mut spend_buffer2 = Vec::new();
    let mut spend_buffer3 = Vec::new();
    let mut spend_buffer4 = Vec::new();
    let mut spend_buffer5 = Vec::new();
    let mut output_buffer1 = Vec::new();

    spend_fs1.read_to_end(&mut spend_buffer1).expect("couldn't read Sapling spend 1 parameters file to buffer");
    spend_fs2.read_to_end(&mut spend_buffer2).expect("couldn't read Sapling spend 2 parameters file to buffer");
    spend_fs3.read_to_end(&mut spend_buffer3).expect("couldn't read Sapling spend 3 parameters file to buffer");
    spend_fs4.read_to_end(&mut spend_buffer4).expect("couldn't read Sapling spend 4 parameters file to buffer");
    spend_fs5.read_to_end(&mut spend_buffer5).expect("couldn't read Sapling spend 5 parameters file to buffer");
    output_fs1.read_to_end(&mut output_buffer1).expect("couldn't read Sapling output 1 parameters file to buffer");

    spend_buffer1.append(&mut spend_buffer2);
    spend_buffer1.append(&mut spend_buffer3);
    spend_buffer1.append(&mut spend_buffer4);
    spend_buffer1.append(&mut spend_buffer5);

    let spend_reconstructed = spend_buffer1;
    let output_reconstructed = output_buffer1;

    (spend_reconstructed, output_reconstructed)
}

fn verify_parameters() {
    let (spend, output) = original_parameters();
    let (spend_reconstructed, output_reconstructed) = reconstruct_parameters();

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

fn main() {
    construct_partial_parameters();
    verify_parameters();
}
