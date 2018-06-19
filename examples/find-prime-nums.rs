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
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::time::Duration;

use rust_tmsn::network::start_network;


#[derive(Deserialize)]
struct Config {
    id: u32,
    left: u32,
    right: u32
}


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message_type: String,
    from: u32,
    data: u32
}


impl Message {
    fn new(message_type: &str, worker_id: u32, data: u32) -> Message {
        Message {
            message_type: String::from(message_type),
            from: worker_id,
            data: data
        }
    }
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
    let (remote_data_send, remote_data_recv): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    // Local data queue, where the data generated locally would be put in
    let (local_data_send, local_data_recv) = mpsc::channel();
    start_network("local", &neighbors, 8000, false, remote_data_send, local_data_recv);

    // Discover stage:
    //   Workers broadcast how many workers they can currently receive message from (discover).
    //   Once it discover all other workers, it is _ready_.
    //
    // Searching stage:
    //   If a worker knows that all workers are ready (can receive message from all other workers),
    //   it starts scanning and sending all prime numbers in its range.
    //
    // Finished stage:
    //   After the searching is done, it broadcasts a "_finish_" signal.
    let mut worker_discover = vec![0; neighbors.len()];
    let mut num_discovered = 0;
    let mut num_workers_ready = 0;
    let mut num_workers_finish = 0;

    // Vectors to store all prime numbers received from network
    let mut all_primes: Vec<u32> = vec![];

    // Exit condition:
    //   Once all workers broadcasted the "finish" signals, the program can exit.
    while num_workers_finish < neighbors.len() {
        println!("status, {}, {}, {}", num_discovered, num_workers_ready, num_workers_finish);
        if let Ok(message) = remote_data_recv.try_recv() {
            println!("received, {:?}", message);
            let message_type = message.message_type;
            let machine_id = message.from as usize;
            let data = message.data;
            match message_type.as_ref() {
                "discover" => {
                    if worker_discover[machine_id] == 0 {
                        // a new (unseen) worker is ready
                        num_discovered += 1;
                    }
                    if worker_discover[machine_id] < data && data as usize == neighbors.len() {
                        num_workers_ready += 1;
                        // if all workers are ready, start search and broadcast prime numbers
                        if num_workers_ready == neighbors.len() {
                            start_search(worker_id, left, right, local_data_send.clone());
                        }
                    }
                    worker_discover[machine_id] = data;
                },
                "searching" => {
                    all_primes.push(data);
                },
                "finish" => {
                    num_workers_finish += 1;
                },
                _ => {
                    println!("Error: Received an undefined message")
                }
            }
        }
        // if not all workers are ready, keep sending some signals
        // so that new workers can see this worker
        if num_workers_ready < neighbors.len() {
            local_data_send.send(
                Message::new("discover", worker_id, num_discovered)
            ).unwrap();
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
fn start_search(worker_id: u32, left: u32, right: u32, local_data_send: Sender<Message>) {
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
                local_data_send.send(
                    Message::new("searching", worker_id, num)
                ).unwrap();
            }
        }
        // Broadcast 0 when the searching is done
        local_data_send.send(
            Message::new("finish", worker_id, 0)
        ).unwrap();
    });
}