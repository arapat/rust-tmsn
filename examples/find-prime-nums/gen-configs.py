#!/usr/bin/env python

# This file generates individual configuration files for each instance in the cluster
# so that the workload is properly distributed over the cluster.

import argparse
import json
import os

from math import ceil


def main(args):
    dir_path = "example-configs"
    n = int(args["limit"])
    k = int(args["count"])
    if not os.path.exists(dir_path):
        os.makedirs(dir_path)
    left = 2
    for i in range(k):
        right = min(n, left + ceil(1.0 * n / k))
        config = {
            "left": left,
            "right": right
        }
        with open(os.path.join(dir_path, "config-{}.json".format(i)), 'w') as f:
            json.dump(config, f, indent=4)
        left = right


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description = "Crate configuration files")
    parser.add_argument("-n", "--limit",
                        required=True,
                        help="the upper limit for the search of the prime numbers")
    parser.add_argument("-c", "--count",
                        required=True,
                        help="the number of instances in the cluster")
    args = vars(parser.parse_args())
    main(args)