#![allow(dead_code)]

///
/// wagyu-zcash-parameters
///
/// This crate dynamically produces the Zcash Sapling parameters by fetching and recombining the raw parameter data
/// from a collection of crates published specifically for constructing the Zcash Sapling parameters.
///

#[cfg(test)]
mod tests;

/// Returns a tuple of `(Spend, Output)` parameters as buffers.
pub fn load_sapling_parameters() -> (Vec<u8>, Vec<u8>) {
    let mut spend_buffer1 = wagyu_zcash_parameters_1::load_partial_parameters();
    let mut spend_buffer2 = wagyu_zcash_parameters_2::load_partial_parameters();
    let mut spend_buffer3 = wagyu_zcash_parameters_3::load_partial_parameters();
    let mut spend_buffer4 = wagyu_zcash_parameters_4::load_partial_parameters();
    let mut spend_buffer5 = wagyu_zcash_parameters_5::load_partial_parameters();
    let output_buffer1 = wagyu_zcash_parameters_6::load_partial_parameters();

    spend_buffer1.append(&mut spend_buffer2);
    spend_buffer1.append(&mut spend_buffer3);
    spend_buffer1.append(&mut spend_buffer4);
    spend_buffer1.append(&mut spend_buffer5);

    let spend_reconstructed = spend_buffer1;
    let output_reconstructed = output_buffer1;

    (spend_reconstructed, output_reconstructed)
}
