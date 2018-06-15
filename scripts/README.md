## Install dependencies

Run `pip install -r requirements.txt` to intall the required python packages.


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

`terminate-cluster.py` terminates a cluster by stopping terminating all instances in this cluster.

### Example
```bash
./terminate-cluster.py --name testing
```

* `init-single_worker.sh`: start up script for single node.
* `setup-cluster.sh`: script for setting up a cluster of multiple workers, the IPs of the workers are provided in the `neighbors.txt` file.
