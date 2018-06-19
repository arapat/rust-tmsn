# rust-tmsn

RustTMSN is a collection of general modules to help implementing TMSN
for various learning algorithms.

## Try out the examples

There are two examples provided in the `/examples` directory.

### Run on your computer

The first example, `network.rs`, is intended to run on your computer locally.
It creates a TCP connection between the computer and itself, and sends a simple
"Hello World" message via the connection.

```bash
cargo run --example network
```

### Run on a cluster

The second example, `find-prime-nums.rs`, is intended to run on
a cluster deployed on Amazon Web Services.
The deployment and setup of the cluster can be achieved using
the scripts provided in the `/scripts` directory.
A step by step instruction of running this example is provided
in the document at `/examples/find-prime-nums-scripts/README.md`.

## Use `rust_tmsn` in your projects

To use `rust_tmsn` in your own project, please download this repository
to your computer, and append following lines to the `Cargo.toml` file
_in your project_ where the path should be the **actual** location of where
this repository is downloaded.

```
[dependencies]
rust_tmsn = { path = "../rust-tmsn" }
```

#### Usage

1. Clone this project to your computer

```bash
git clone https://github.com/arapat/rust-tmsn.git
```

2. In the same directory, create your own project

```bash
cargo new my_project
```

Now you should see two directories in your workspace

```bash
$ ls
my_project rust-tmsn
```

3. Append the dependency to `rust-tmsn` in your project

Use your favorite editor and edit Cargo.toml file.
After the editing, your should see the following
```bash
$ cat my_project/Cargo.toml
[package]
name = "my_project"
version = "0.1.0"
authors = ["Your Name <your@email.com>"]

[dependencies]
rust_tmsn = { path = "../rust-tmsn" }
```

4. Write your program in `my_project/src/main.rs`. For example,
you can copy and paste the content of the example `network.rs`
we provided.

5. (Optional) To print the runtime logs to the terminal, you should use
the `env_logger` package in your project code.

First add the dependency to the `env_logger` in the `Cargo.toml` file.

```toml
[dependencies]
env_logger = "0.5.5"
time = "0.1.39"
```

The `time` crate is optional, and used for specifying the logging format (see below).

Then specify the dependency in the project code by adding
the following line in the very begining of `src/main.rs`

```rust
extern crate env_logger;
extern crate time;  // optional

use time::get_time; // optional
```

Finally, define the logging format in the definition of the `main()` function,
for example

```rust
fn main() {
    // get_time() function requires the `time` crate dependency
    let base_timestamp = get_time().sec;
    env_logger::Builder
              ::from_default_env()
              .format(move|buf, record| {
                  let timestamp = get_time();
                  let epoch_since_apr18: i64 = timestamp.sec - base_timestamp;
                  let formatted_ts = format!("{}.{:09}", epoch_since_apr18, timestamp.nsec);
                  writeln!(
                      buf, "{}, {}, {}, {}",
                      record.level(), formatted_ts, record.module_path().unwrap(), record.args()
                  )
              })
              .init();
    // ... more code
}
```

Please refer to the `find-prime-nums.rs` file under the `/examples` directory for a more specific
example of enabling the logging.

Note that `env_logger` is used for configuration so that the logs would be written to stdout/stderr
(which can then be redirected to other files using the commandline arguments if you like).
To use the actual logging APIs (e.g. `info!()`, `error!()`, etc.), one should use the `log` crate.
For more information on using logs in Rust, please check out
[the documentation of the `log` crate](https://docs.rs/log/latest/log/).


5. Compile your program and run it

```bash
cd my_project

# Compile
cargo build  # without optimization
cargo build --release  # with optimization

# Run
cargo run  # without printing logs
RUST_LOG=debug cargo run --example find-prime-nums  # with printing logs
```
