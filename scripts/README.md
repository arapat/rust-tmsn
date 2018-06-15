## Install dependencies

Run `pip install -r requirements.txt` to install the required python packages.


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
./run-cluster.py -k ./ec2-key-pair.pem --script ./run-on-workers/hello-world.sh
```