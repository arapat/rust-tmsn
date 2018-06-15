#!/usr/bin/env python
import argparse
import subprocess


def main(args):
    create_command = """
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
        args["ami"],
        args["count"],
        args["type"],
        args["key"],
        args["name"]
    )
    subprocess.run(create_command, shell=True, check=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description = "Crate a cluster using AWS spot instances")
    parser.add_argument("-c", "--count",
                        required=True,
                        help="the number of instances in the cluster")
    parser.add_argument("-k", "--key",
                        required=True,
                        help="the EC2 key pair name for creating the instances")
    parser.add_argument("--name",
                        required=True,
                        help="cluster name")
    # parser.add_argument("-t", "--type",
    #                     required=True,
    #                     help="the type of the instances")
    args = vars(parser.parse_args())
    args["ami"] = "ami-a4dc46db"
    args["type"] = "m3.xlarge"
    main(args)
