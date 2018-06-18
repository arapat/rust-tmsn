#[macro_use] extern crate serde_derive;
extern crate rust_tmsn;
extern crate serde_json;

use std::io::prelude::*;
use std::sync::mpsc;
use std::thread::sleep;
use std::thread::spawn;

use rust_tmsn::network::start_network;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;


#[derive(Deserialize)]
struct Config {
    left: u32,
    right: u32
}


fn main() {
    let remote_base_dir = String::from("/home/ubuntu/workspace/");

    // Load config file
    let mut f = File::open(remote_base_dir.clone() + "configuration")
                    .expect("Config file not found.");
    let mut json = String::new();
    f.read_to_string(&mut json)
     .expect("Error: cannot read the configuration file.");
    let config: Config = serde_json::from_str(&json).expect(
        "Cannot parse the configuration file in JSON."
    );
    let left = config.left;
    let right = config.right;

    // Read the list of neighbors
    let mut neighbors = vec![];
    let buf_reader = BufReader::new(
        File::open(remote_base_dir.clone() + "neighbors.txt")
            .expect("file not found")
    );
    for (idx, line) in buf_reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        neighbors.push(line.unwrap().trim().to_string())
    }

    // Remote data queue, where the data received from network would be put in
    let (remote_data_send, remote_data_recv) = mpsc::channel();
    // Local data queue, where the data generated locally would be put in
    let (local_data_send, local_data_recv) = mpsc::channel();
    start_network("local", &neighbors, 8000, false, remote_data_send, local_data_recv);
    sleep(Duration::from_secs(1));

    spawn(move|| {
        // search for the prime numbers in the range [left, right)
        for num in left..right {
            let mut is_nonprime = num != 2 && num % 2 == 0;
            let mut k = 3;
            while !is_nonprime && k * k <= num {
                if num % k == 0 {
                    is_nonprime = true;
                }
                k += 2;
            }
            if is_nonprime {
                // Found a non-prime number, broadcast to network
                local_data_send.send(num).unwrap();
            }
        }
        // Broadcast 0 when the searching is done
        local_data_send.send(0).unwrap();
    });

    let mut all_primes = vec![];
    let mut num_workers_still_running = neighbors.len();
    while num_workers_still_running > 0 {
        let num = remote_data_recv.recv().unwrap();
        if num == 0 {
            num_workers_still_running -= 1;
        } else {
            all_primes.push(num.to_string());
        }
    }
    all_primes.sort();

    let mut file = File::create("primes.txt").unwrap();
    writeln!(file, "{}", all_primes.join(" ")).unwrap();
}