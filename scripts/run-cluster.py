#!/usr/bin/env python
import argparse
import os
import subprocess
import yaml


def check_exists(path):
    return os.path.isfile(path) 


def parse_file_path(path):
    return (path, path.rsplit('/', 1)[1])


def main(args):
    if not check_exists(args["key"]):
        print("Error: File '{}' does not exist.".format(args["key"]))
        return
    if not check_exists(args["neighbors"]):
        print("Error: File '{}' does not exist.".format(args["neighbors"]))
        return
    if args["files"] is None:
        args["files"] = []
    for filepath in args["files"]:
        if not check_exists(filepath):
            print("Error: File '{}' does not exist.".format(filepath))
            return

    with open(args["neighbors"]) as f:
        status = f.readline()
        if status[0] != 'R':  # Not "Ready."
            print("Please run `check-cluster.py` first and "
                  "make sure all instances in the cluster is up and running.")
            return
        instances = [t.strip() for t in f if t.strip()]

    key = args["key"]
    base_path = args["base_path"]
    fullpath, filename = parse_file_path(args["script"])
    remote_file_path = os.path.join(base_path, filename)
    run_in_foreground = args["output"]
    log_path = "/tmp/run.log"
    for url in instances:
        # Create base path
        command = ("ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                   "\"mkdir -p {}\";").format(key, url, base_path)
        # Send all support files
        for filepath in args["files"]:
            command += (" scp -o StrictHostKeyChecking=no -i {} {} ubuntu@{}:{}"
                        ";").format(key, filepath, url, base_path)
        # Send the script and make it runnable
        if not run_in_foreground:
            command += (" scp -o StrictHostKeyChecking=no -i {} {} ubuntu@{}:{}"
                        ";").format(key, fullpath, url, base_path)
            command += (" ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                        "\"sudo chmod u+x {}\";").format(key, url, remote_file_path)
        # Execute the script
        command += (" ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                    "").format(key, url)
        if run_in_foreground:
            with open(fullpath) as f:
                command += "\" {} \"".format(f.read())
        else:
            command += "\"{} > {} 2>&1 < /dev/null\"".format(remote_file_path, log_path)
            command = "({}) &".format(command)

        if run_in_foreground:
            print()
        print("Running on '{}'".format(url))
        subprocess.run(command, shell=True, check=True)

    if not run_in_foreground:
        print("\nThe script '{}' has been started in the background on all instances. "
            "Note that we don't check if the script is launched successfully "
            "or is finished.\n"
            "The stdout/stderr of the script has been redirected to the file {} "
            "on the remote instance".format(fullpath, log_path))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Run a script on all instances of a cluster")
    parser.add_argument("-s", "--script",
                        required=True,
                        help="File path of the script that needs to run on the cluster")
    parser.add_argument("--files",
                        nargs='+',
                        help=("File path of the file that needs to be sent to the instances. "
                                "For multiple files, separate them using spaces."))
    parser.add_argument("--output",
                        action="store_true",
                        help=("If set, wait till the script exits on the instances and print its "
                              "output to the commandline. Otherwise, run the script in the "
                              "background and redirect the stdout/stderr of the script to "
                              "a log file on the instance."))
    # parser.add_argument("-k", "--key",
    #                     required=True,
    #                     help="File path of the EC2 key pair file")
    args = vars(parser.parse_args())
    args["neighbors"] = "./neighbors.txt"
    args["base_path"] = "/home/ubuntu/workspace"
    with open("credentials.yml") as f:
        creds = yaml.load(f)
        creds = list(creds.values())[0]
        args["key"] = creds["ssh_key"]
    main(args)