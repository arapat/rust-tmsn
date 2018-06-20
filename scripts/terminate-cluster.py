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
        --query 'Reservations[*].Instances[*].[InstanceId]'
    """.format(args["aws_access_key_id"], args["aws_secret_access_key"], args["name"])
    result = subprocess.run(query_command, shell=True, check=True, stdout=subprocess.PIPE)
    output = result.stdout
    all_status = json.loads(output)
    if len(all_status) == 0:
        print("No instance found in the cluster '{}'. Quit.".format(args["name"]))
        return

    total = len(all_status[-1])
    confirm = input("Shutting down {} instances in the cluster '{}'. Are you sure? (y/N) ".format(
        total, args["name"]))
    if confirm.strip().lower() != 'y':
        print("Operation cancelled. Nothing is changed. Quit.")
        return

    term_command = ""
    for status in all_status:
        ids = ' '.join(map(itemgetter(0), status))
        term_command += """
        AWS_ACCESS_KEY_ID="{}" AWS_SECRET_ACCESS_KEY="{}" aws ec2 terminate-instances --instance-ids {};
        """.format(args["aws_access_key_id"], args["aws_secret_access_key"], ids)
    subprocess.run(term_command, shell=True, check=True)
    print("Done.")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Terminate a cluster")
    parser.add_argument("--name",
                        required=True,
                        help="cluster name")
    parser.add_argument("--credential",
                        help="path to the credential file")
    config = load_config(vars(parser.parse_args()))
    main(config)