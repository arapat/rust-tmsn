import json
import os
import subprocess
import sys
import yaml


def load_config(args, config_path="~/.tmsn_config"):
    def load_credential(config):
        with open(config["credential"]) as f:
            creds = yaml.load(f)
            creds = list(creds.values())[0]
            config["key"] = creds["key_name"]
            config["key_path"] = creds["ssh_key"]
            config["aws_access_key_id"] = creds["access_key_id"]
            config["aws_secret_access_key"] = creds["secret_access_key"]

    # Load configuration
    config_path = os.path.expanduser(config_path)
    config = {}
    if os.path.isfile(config_path):
        with open(config_path) as f:
            config = yaml.load(f)
    # Load arguments
    for t in args:
        if args[t] is not None:
            config[t] = args[t]
    # Check the credential file
    if "credential" not in config or not config["credential"]:
        print("Error: Please provide the path to the credential file.")
        sys.exit(1)
    config["credential"] = os.path.abspath(config["credential"])
    # Save the configuration
    with open(config_path, 'w') as f:
        yaml.dump(config, f)
    # Load credential
    load_credential(config)
    return config


def query_status(args):
    query_command = """
    AWS_ACCESS_KEY_ID="{}" AWS_SECRET_ACCESS_KEY="{}" \
    aws ec2 describe-instances \
        --filter Name=tag:cluster-name,Values={} \
        --query 'Reservations[*].Instances[*].[State.Name,PublicIpAddress]'
    """.format(args["aws_access_key_id"], args["aws_secret_access_key"], args["name"])
    result = subprocess.run(query_command, shell=True, check=True, stdout=subprocess.PIPE)
    output = result.stdout
    all_status = json.loads(output)
    return all_status


def check_connections(instances, args, timeout=2):
    def try_ssh_instance(url):
        command = ("ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                    "\"echo Hello > /dev/null\"").format(args["key_path"], url)
        try:
            t = subprocess.run(command, shell=True, timeout=timeout)
        except subprocess.TimeoutExpired:
            return False
        return t.returncode == 0

    print("Checking the network connections...")
    for url in instances:
        if not try_ssh_instance(url):
            print("Error: Cannot SSH to the instance '{}'. ".format(url) +
                  "Did you run `./check-cluster.py`?")
            return False
    return True