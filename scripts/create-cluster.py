#!/usr/bin/env python
import argparse
import subprocess

from config import load_config


def main(args):
    create_command = """
    AWS_ACCESS_KEY_ID="{}" AWS_SECRET_ACCESS_KEY="{}" \
    aws ec2 run-instances \
        --image-id {} \
        --count {} \
        --instance-type {} \
        --key-name {} \
        --instance-market-options 'MarketType=spot,SpotOptions={{MaxPrice='0.3'}}' \
        --tag-specifications 'ResourceType=instance,Tags=[{{Key=cluster-name,Value={}}}]' \
        --associate-public-ip-address \
        --block-device-mappings \
            '[{{\"DeviceName\":\"/dev/xvdb\",\"VirtualName\":\"ephemeral0\"}}, \
              {{\"DeviceName\":\"/dev/xvdc\",\"VirtualName\":\"ephemeral1\"}}]' \
        --no-dry-run
    """.format(
        args["aws_access_key_id"],
        args["aws_secret_access_key"],
        args["ami"],
        args["count"],
        args["type"],
        args["key"],
        args["name"]
    )
    subprocess.run(create_command, shell=True, check=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Crate a cluster using AWS spot instances")
    parser.add_argument("-c", "--count",
                        required=True,
                        help="the number of instances in the cluster")
    parser.add_argument("--name",
                        required=True,
                        help="cluster name")
    # parser.add_argument("-t", "--type",
    #                     required=True,
    #                     help="the type of the instances")
    parser.add_argument("--credential",
                        help="path to the credential file")
    args = vars(parser.parse_args())
    args["ami"] = "ami-a4dc46db"
    args["type"] = "m3.xlarge"
    config = load_config(args)
    main(config)
