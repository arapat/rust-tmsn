# rust-tmsn

RustTMSN is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Use rust_tmsn with Cargo

Please download the source code of RustTMSN to your computer, and
append following lines to the `Cargo.toml` file in your project
with the path should be the actual location of RustTMSN.

```
[dependencies]
rust_tmsn = { path = "../rust-tmsn" }
```

## Managing a cluster on AWS

Helper scripts are provided in the `scripts/` directory for managing a cluster
on Amazon Web Services and starting the TMSN code over multiple instances in parallel.