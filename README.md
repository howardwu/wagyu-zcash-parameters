# wagyu-zcash-parameters
[![Crates.io](https://img.shields.io/crates/v/wagyu-zcash-parameters.svg?color=neon)](https://crates.io/crates/wagyu-zcash-parameters)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](./AUTHORS)
[![License](https://img.shields.io/badge/license-MIT/Apache--2.0-blue.svg)](./LICENSE-MIT)

This library provides native Zcash Sapling parameters as buffers.

## Usage

Use this library in your Rust code as follows:

```rust
use wagyu_zcash_parameters::load_sapling_parameters;

// Loads Zcash Sapling parameters as buffers
let (spend, output) = load_sapling_parameters();

println!("Number of bytes in Sapling spend parameter: {:?}", spend.len());
println!("Number of bytes in Sapling output parameter: {:?}", output.len());
```

## Purpose

This crate dynamically produces the Zcash Sapling parameters by fetching and recombining the raw parameter data
from a collection of crates published specifically for constructing the Zcash Sapling parameters.


[Crates.io](https://crates.io) maintains a maximum size limit of 10,485,760 bytes for each published crate.
 This size restriction is beyond the capacity of the Zcash Sapling spend and output parameters,
 which are 47,958,396 bytes and 3,592,860 bytes respectively.

## Developers

To run the test, run:
```$xslt
cargo test
```

To run the example, run:
```$xslt
cargo run --example load_sapling_parameters
```

The example should output:
```$xslt
Number of bytes in Sapling spend parameter: 47958396
Number of bytes in Sapling output parameter: 3592860
```

## License

This work is licensed under either of the following licenses, at your discretion.

- Apache License Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
