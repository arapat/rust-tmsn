/*!
`rust_tmsn` is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Use rust_tmsn with Cargo

Please download the source code of `rust_tmsn` to your computer, and
append following lines to the `Cargo.toml` file in your project
with the path should be the actual location of `rust_tmsn`.

```ignore
[dependencies]
rust_tmsn = { path = "../rust-tmsn" }
```
*/
#[macro_use] extern crate log;
extern crate bufstream;
extern crate serde;
extern crate serde_json;

/// Establish network connections between the workers in the cluster
pub mod network;