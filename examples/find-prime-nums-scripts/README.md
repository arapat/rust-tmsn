
## Demo: Find Prime Numbers

In this example, we want to find all prime numbers that no larger than a given positive number N.
The purpose of this example is to show how to set up a cluster using the scripts provided in
the `/scripts` directory and communicate between the instances using the `tmsn_rust::network`
module.

Navigate to the directory that contains this `README.md` file, and follow the steps below.

### Step 1: Create the cluster

Create a cluster that consists of 3 instances
```bash
../../scripts/create-cluster.py -c 3 --name find-prime
```

Check the cluster status and obtain the `neighbors.txt` file
```bash
../../scripts/check-cluster.py --name find-prime
```

### Step 2: Set up the cluster

Set up the instances and clone this repository to all instances
```bash
../../scripts/run-cluster.py --script ../../scripts/script-examples/init-worker.sh
```

### Step 3: Generate configuration files for each instance and transfer them to the instances

Generate individual configuration files for each instance
```bash
./gen-configs.py -n 1000 -c 3
```
Specifically in this example, we want to generate configuration files that collectively
find all prime numbers that no larger than 1000 using a cluster that consists of 3 instances.

After running the above command, a directory named "example-configs" containing three JSON files
should have created
```bash
$ ls example-configs/
config-0.json  config-1.json  config-2.json
```

Now we can tranfer these configuration files to the instances
```bash
../../scripts/send-configs.py --config ./example-configs/
```

### Step 4: Run the program that finds prime numbers on the cluster

Create a script that starts your program, such as `./find-primes.sh` in this example,
and launch this script on all instances using the `run-cluster.py` tool.
```bash
../../scripts/run-cluster.py --files ./neighbors.txt --script ./find-primes.sh
```

### Step 5: Check if the program has finished executing

Create a script that checks if your program is still running, such as `./check-prog-status.sh`
in this example, and launch this script on all instances using the `run-cluster.py` tool.
```bash
../../scripts/run-cluster.py --script ./check-prog-status.sh --output
```

### Step 6a: Retrieve output files from the instances

Once all instances finished running, we can retrieve the output files from them.

```bash
../../scripts/retrieve-files.py --remote /home/ubuntu/workspace/rust-tmsn/primes.txt --local ./_result/
```

The script will create a sub-directory in `./_result` for every worker, and download the files from the workers
to corresponding sub-directories.


### Step 6b: Retrieve log files from the instances

We can also retrieve the log files from the instances.

```bash
../../scripts/retrieve-files.py --remote /tmp/run.log --local ./_logs
```

The logs should be downloaded to `./_logs` directory.


### Step 7: Terminate the cluster

```bash
../../scripts/terminate-cluster.py --name find-prime
```
