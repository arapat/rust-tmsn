# rust-tmsn

RustTMSN is a collection of general modules written in the RUST language, which implement the networking layer for TMSN (Tell Me Something New) distributed learning systems.
* Paper: [Tell me something new: A new framework for asynchronous Parallel Learning](https://arxiv.org/abs/1805.07483) / Julaiti Arapat and Yoav Freund, 2018

## Directory structure 
* **src**: the source code
* **doc**: documentation
   * `doc/rust_tmsn/network/index.html` The root of the html documentation tree.
* **target**: the executables.
* **scripts**: Python scripts for creating and managing a cluster of `n` computers on AWS.


## Get started
1. Clone this repository
2. Install [RUST](https://www.rust-lang.org/en-US/)
2. Build the executable
* Add example commands.
3. Run stand-alone example
## Scripts


## Try out the examples

There are two examples provided in the `/examples` directory.

### Run example on your computer

The first example, `network.rs`, is intended to run on your computer locally.
It creates a TCP connection between the computer and itself, and sends a simple
"Hello World" message via the connection.

```bash
cargo run --example network
```

### Run on a cluster

The second example, `find-prime-nums.rs`, is intended to run on
a cluster deployed on Amazon Web Services. You will need to have an account on AWS to try this.
The deployment and setup of the cluster can be achieved using
the scripts provided in the `/scripts` directory.
A step by step instruction of running this example is provided
in the document at `/examples/find-prime-nums-scripts/README.md`.

## Use Rust-TMSN in your project
* [UsingRustTMSN.md](./UsingRustTMSN.md)