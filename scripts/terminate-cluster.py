#!/usr/bin/env python
import argparse
import json
import subprocess

from operator import itemgetter


def main(args):
    query_command = """
    aws ec2 describe-instances \
        --filter Name=tag:cluster-name,Values={} \
        --query 'Reservations[*].Instances[*].[InstanceId]'
    """.format(args["name"])
    result = subprocess.run(query_command, shell=True, check=True, stdout=subprocess.PIPE)
    output = result.stdout
    status = json.loads(output)
    if len(status) == 0:
        print("No instance found in the cluster '{}'. Quit.".format(args["name"]))
        return
    status = status[0]

    total = len(status)
    confirm = input("Shutting down {} instances in the cluster '{}'. Are you sure? (y/N) ".format(
        total, args["name"]))
    if confirm.strip().lower() != 'y':
        print("Operation cancelled. Nothing is changed. Quit.")
        return

    ids = ' '.join(map(itemgetter(0), status))
    term_command = "aws ec2 terminate-instances --instance-ids " + ids
    subprocess.run(term_command, shell=True, check=True)
    print("Done.")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description = "Terminate a cluster")
    parser.add_argument("--name",
                        required=True,
                        help="cluster name")
    args = vars(parser.parse_args())
    main(args)
