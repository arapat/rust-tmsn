import os
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