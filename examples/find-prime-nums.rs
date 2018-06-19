#[macro_use] extern crate serde_derive;
extern crate rust_tmsn;
extern crate serde_json;

use std::io::prelude::*;
use std::sync::mpsc;
use std::thread::sleep;
use std::thread::spawn;

use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::time::Duration;

use rust_tmsn::network::start_network;


#[derive(Deserialize)]
struct Config {
    id: u32,
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
    let worker_id = config.id;

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

    // Wait for all computers to be online
    let mut all_workers_up = false;
    let mut up_workers = vec![false; neighbors.len()];
    // Vectors to store all prime numbers received from network
    let mut all_primes: Vec<u32> = vec![];
    let mut num_working_workers = 0;

    // Workers send a number `1` when they are up;
    // then start sending all prime numbers they found;
    // finally send a number `0` before terminating

    // The worker start the thread that actually search for
    // the prime numbers after it receives the "up" signals (the number `1`)
    // from all workers.
    // After that, it keeps listening to the network until
    // the number of workers that send the "down" signal (the number `0`)
    // is equal to the number of workers in the network

    while !all_workers_up || num_working_workers > 0 {
        println!("status, {}, {}", all_workers_up, num_working_workers);
        if !all_workers_up {
            local_data_send.send((worker_id.clone(), 1)).unwrap();
            sleep(Duration::from_millis(500));
        }
        if let Ok((machine_id, num)) = remote_data_recv.try_recv() {
            println!("received, {}, {}", machine_id, num);
            if num != 0 && num != 1 {
                // if the incoming number is neither "up" nor "down" signals
                // then it is the prime number found by other machines
                all_primes.push(num);
            }
            else if num == 1 && up_workers[machine_id as usize] == false {
                // a new (unseen) worker is up
                up_workers[machine_id as usize] = true;
                num_working_workers += 1;
                // if all workers are up
                if num_working_workers == neighbors.len() {
                    all_workers_up = true;
                    start_search(worker_id, left, right, local_data_send.clone());
                }
            } else if num == 0 {
                // a worker is stopped
                num_working_workers -= 1;
            }
        }
    }
    all_primes.sort();

    let mut file = File::create("primes.txt").unwrap();
    let str_prime_nums: Vec<String> = all_primes.iter()
                                                .map(|num| num.to_string())
                                                .collect();
    writeln!(file, "{}", str_prime_nums.join(" ")).unwrap();
}


// Start searching and broadcasting prime numbers in [left, right)
// in a separate thread
fn start_search(worker_id: u32, left: u32, right: u32, local_data_send: Sender<(u32, u32)>) {
    spawn(move|| {
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
                local_data_send.send((worker_id.clone(), num)).unwrap();
            }
        }
        // Broadcast 0 when the searching is done
        local_data_send.send((worker_id, 0)).unwrap();
    });
}