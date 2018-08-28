
## Run Jupyter on AWS

1. Start the cluster

```bash
./create-cluster.py -c 1 --name jupyter
```

To speicify a custom AMI image, append "--ami <ami_id>" to the cluster creation command above.

2. Check the cluster is up

```bash
./check-cluster.py --name jupyter
```

3. Run the setup script on the cluster

```bash
./run-cluster.py -s script-examples/install-jupyter.sh --output
```

4. Open the URL printed out in the Step 3.

5. Shut down the instance

```bash
./terminate-cluster.py --name jupyter
```

6. To SSH into the first node, run

```bash
./ssh-headnode.py --name jupyter
```
