#!/usr/bin/env python
import argparse
import json
import subprocess

from operator import itemgetter
from config import load_config


def main(args):
    query_command = """
    AWS_ACCESS_KEY_ID="{}" AWS_SECRET_ACCESS_KEY="{}" \
    aws ec2 describe-instances \
        --filter Name=tag:cluster-name,Values={} \
        --query 'Reservations[*].Instances[*].[State.Name,PublicIpAddress]'
    """.format(args["aws_access_key_id"], args["aws_secret_access_key"], args["name"])
    result = subprocess.run(query_command, shell=True, check=True, stdout=subprocess.PIPE)
    output = result.stdout
    status = json.loads(output)
    if len(status) == 0:
        print("No instance found in the cluster '{}'. Quit.".format(args["name"]))
        return
    status = status[-1]

    total = len(status)
    ready = sum(t[0] == "running" for t in status)
    neighbors = list(map(itemgetter(1), status))
    print("Total instances: {}\nReady and running: {}".format(total, ready))
    if ready == 0:
        return
    with open("neighbors.txt", 'w') as f:
        if total == ready:
            f.write("Ready. ")
        else:
            f.write("NOT ready. ")
        f.write("IP addresses of all instances:\n")
        f.write('\n'.join(neighbors))
    print("The public IP addresses of the instances have been written into `./neighbors.txt`")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Check the status of a cluster")
    parser.add_argument("--name",
                        required=True,
                        help="cluster name")
    parser.add_argument("--credential",
                        help="path to the credential file")
    config = load_config(vars(parser.parse_args()))
    main(config)