## Install dependencies

Run `pip install -r requirements.txt` to install the required python packages.


## Help message

Run any script in this directory with `-h` argument will print the help message of the script.


## Create a new cluster

`create-cluster.py` creates a cluster on the `m3.xlarge` instance using an AMI based on Ubuntu.

### Example
```bash
./create-cluster.py -c 2 --name testing --key ec2-key-pair
```


## Check if a cluster is ready

`check-cluster.py` checks if a cluster is up and running. In addition, it also creates a
`neighbors.txt` file which contains the IP addresses of all the instances in the cluster.

### Example
```bash
./check-cluster.py --name testing
```


## Terminate a cluster

`terminate-cluster.py` terminates a cluster by stopping and terminating all instances
in this cluster.

### Example
```bash
./terminate-cluster.py --name testing
```


## Run a script on a cluster

`run-cluster.py` runs a given script on all instances in the cluster.
It starts the script in the background, and redirect the stdout/stderr
into a file on the instances which can be checked later.
Thus it terminates does not necessarily mean the script has finshed executing on the cluster.
In addition, it only launches the script on all instances, but does _not_ check if the script
executes without error.

### Example
```bash
./run-cluster.py -k ./ec2-key-pair.pem --script ./script-examples/hello-world.sh
```


## Send configuration files to all instances in a cluster

In most cases, we would like the different workers/instances in a cluster run with
different parameters. We can achieve that by generating a different configuration file
for each worker, and letting the program read its parameter from this file.
The script `send-configs.py` is used for sending the configuration files to the workers.
Please refer to the Find Prime Number examples in `/example` for the demonstration of using
this script.

### Example
After generating a cluster with `N` workers, one can write a custom script to generate `N`
configuration files, one for each worker, and save all configuration files in the some directory
(e.g. `./example-configs/`). After that, run following command

```bash
./send-configs.py -k ./ec2-key-pair.pem --config ./example-configs/
```


## Retrieve files from all instances in a cluster

`retrieve-files.py` retrieve files from the same location on all instances of a cluster.
It can be used to collect the output of the program from the workers.
A local directory for saving the downloaded files should be provided to this script.
This script will create a separate sub-directory for each worker and download its files
to this sub-directory.

### Example
```bash
./retrieve-files.py -k ./ec2-key-pair.pem --remote /home/ubuntu/workspace/rust-tmsn/output.txt --local ./_result/
```