use wagyu_zcash_parameters::load_sapling_parameters;

pub fn main() {
    // Loads Zcash Sapling parameters as buffers
    let (spend, output) = load_sapling_parameters();

    println!("Number of bytes in Sapling spend parameter: {:?}", spend.len());
    println!("Number of bytes in Sapling output parameter: {:?}", output.len());
}
