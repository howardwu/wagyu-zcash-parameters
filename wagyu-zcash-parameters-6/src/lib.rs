#[cfg(test)]
mod tests;

pub fn load_partial_parameters() -> Vec<u8> {
    let output_buffer = include_bytes!("./sapling-output-1.params");
    output_buffer.to_vec()
}
