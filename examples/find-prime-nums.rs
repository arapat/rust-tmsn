#[macro_use] extern crate serde_derive;
extern crate env_logger;
extern crate rust_tmsn;
extern crate serde_json;
extern crate time;

use std::io::prelude::*;
use std::sync::mpsc;
use std::thread::sleep;
use std::thread::spawn;
use time::get_time;

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
    // set logger
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
    let mut up_workers = vec![false; neighbors.len()];
    let mut num_workers_up = 0;
    let mut num_workers_down = 0;
    // Vectors to store all prime numbers received from network
    let mut all_primes: Vec<u32> = vec![];

    // Workers send a number `1` when they are up;
    // then start sending all prime numbers they found;
    // finally send a number `0` before terminating
    //
    // The worker start the thread that actually search for
    // the prime numbers after it receives the "up" signals (the number `1`)
    // from all workers.
    //
    // After that, it keeps listening to the network until
    // it receives "down" signals (the number `0`) from all workers.
    while num_workers_up < neighbors.len() || num_workers_down < neighbors.len() {
        println!("status, {}, {}", num_workers_up, num_workers_down);
        if let Ok((machine_id, num)) = remote_data_recv.try_recv() {
            println!("received, {}, {}", machine_id, num);
            if num != 0 && num != 1 {
                // if the incoming number is neither "up" nor "down" signals
                // then it is the prime number found by other machines
                all_primes.push(num);
            } else if num == 0 {
                // a worker is stopped
                num_workers_down += 1;
            }
            if up_workers[machine_id as usize] == false {
                // a new (unseen) worker is up
                up_workers[machine_id as usize] = true;
                num_workers_up += 1;
                // if all workers are up, start search and broadcast prime numbers
                if num_workers_up == neighbors.len() {
                    start_search(worker_id, left, right, local_data_send.clone());
                }
            }
        }
        // if not all workers are up, keep sending some signals
        // so that new workers can see this worker
        if num_workers_up < neighbors.len() {
            local_data_send.send((worker_id, 1)).unwrap();
            sleep(Duration::from_millis(500));
        }
    }
    all_primes.sort();

    let mut file = File::create("primes.txt").unwrap();
    let str_prime_nums: Vec<String> = all_primes.iter()
                                                .map(|num| num.to_string())
                                                .collect();
    writeln!(file, "Result from Worker {}:\n{}", worker_id, str_prime_nums.join(" ")).unwrap();
}


// Start searching and broadcasting prime numbers in [left, right)
// in a separate thread
fn start_search(worker_id: u32, left: u32, right: u32, local_data_send: Sender<(u32, u32)>) {
    spawn(move|| {
        for num in left..right {
            let mut is_prime = true;
            let mut k = 2;
            while is_prime && k * k <= num {
                if num % k == 0 {
                    is_prime = false;
                }
                k += 1;
            }
            if num > 1 && is_prime {
                // Found a non-prime number, broadcast to network
                local_data_send.send((worker_id.clone(), num)).unwrap();
            }
        }
        // Broadcast 0 when the searching is done
        local_data_send.send((worker_id, 0)).unwrap();
    });
}