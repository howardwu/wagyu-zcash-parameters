# wagyu-zcash-parameters
[![Crates.io](https://img.shields.io/crates/v/wagyu-zcash-parameters.svg?color=neon)](https://crates.io/crates/wagyu-zcash-parameters)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](./AUTHORS)
[![License](https://img.shields.io/badge/license-MIT/Apache--2.0-blue.svg)](./LICENSE-MIT)

## Overview

This crate dynamically produces the Zcash Sapling parameters by fetching and recombining the raw parameter data
from a collection of crates published specifically for constructing the Zcash Sapling parameters.

## Purpose

[Crates.io](https://crates.io) maintains a maximum size limit of 10,485,760 bytes for each published crate.
 This size restriction is beyond the capacity of the Zcash Sapling spend and output parameters,
 which are 47,958,396 bytes and 3,592,860 bytes respectively.

## License

This work is licensed under either of the following licenses, at your discretion.

- Apache License Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
