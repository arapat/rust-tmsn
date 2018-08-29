#!/usr/bin/env python
import argparse
import subprocess

from common import load_config
from common import query_status


def main(args):
    check_security_group = """
    AWS_ACCESS_KEY_ID="{}" AWS_SECRET_ACCESS_KEY="{}" \
    aws ec2 describe-security-groups --group-names default
    """.format(
        args["aws_access_key_id"],
        args["aws_secret_access_key"],
    )
    print("Checking security group...")
    subprocess.run(check_security_group, shell=True, check=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Print diagnose info on an exsting cluster")
    parser.add_argument("--name",
                        help="cluster name")
    parser.add_argument("--credential",
                        help="path to the credential file")
    args = vars(parser.parse_args())
    config = load_config(args)
    main(config)