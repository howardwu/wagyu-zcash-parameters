#[cfg(test)]
mod tests;

pub fn load_partial_parameters() -> Vec<u8> {
    let spend_buffer = include_bytes!("./sapling-spend-2.params");
    spend_buffer.to_vec()
}
