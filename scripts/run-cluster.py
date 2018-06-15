#!/usr/bin/env python
import argparse
import os
import subprocess


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
    log_path = os.path.join(base_path, "run.log")
    for url in instances:
        print("Running on '{}'".format(url))

        # Create base path
        command = ("ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                   "\"mkdir -p {}\";").format(key, url, base_path)
        # Send all support files
        for filepath in args["files"]:
            command += (" scp -o StrictHostKeyChecking=no -i {} {} ubuntu@{}:{}"
                        ";").format(key, filepath, url, base_path)
        # Send the script
        command += (" scp -o StrictHostKeyChecking=no -i {} {} ubuntu@{}:{}"
                    ";").format(key, fullpath, url, base_path)
        # Make it runnable
        command += (" ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                    "\"sudo chmod u+x {}\";").format(key, url, remote_file_path)
        # Execute the script
        command += (" ssh -o StrictHostKeyChecking=no -i {} ubuntu@{} "
                    "\"{} > {} 2>&1 < /dev/null\"").format(key, url, remote_file_path, log_path)

        command_in_background = "({}) &".format(command)
        subprocess.run(command_in_background, shell=True, check=True)

    print("\nThe script '{}' has been started on all instances. "
          "Note that we don't check if the script is launched successfully "
          "or is finished.\n"
          "The stdout/stderr from the script has been redirected to the file {} "
          "on the remote instance".format(fullpath, log_path))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description = "Crate a cluster using AWS spot instances")
    parser.add_argument("-s", "--script",
                        required=True,
                        help="File path of the script that needs to run on the cluster")
    parser.add_argument("-k", "--key",
                        required=True,
                        help="File path of the EC2 key pair file")
    parser.add_argument("--files",
                        nargs='+',
                        help=("File path of the file that needs to be sent to the instances. "
                                "For multiple files, separate them using spaces."))
    args = vars(parser.parse_args())
    args["neighbors"] = "./neighbors.txt"
    args["base_path"] = "/home/ubuntu/workspace"
    main(args)