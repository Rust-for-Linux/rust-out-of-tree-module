#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""generate_rust_analyzer - Generates an out-of-tree module `rust-project.json` file for `rust-analyzer`
based on the kernel rust-project.json.
"""

import argparse
import json
import logging
import os
import pathlib
import sys

def generate_rust_project(kdir, root_module):
    with open(kdir / "rust-project.json") as fd:
        rust_project = json.loads(fd.read())

    crate_indices = {}

    for i, crate in enumerate(rust_project["crates"]):
        crate_indices[crate["display_name"]] = i

        # Prepend kdir to existing root_module
        crate["root_module"] = os.path.join(kdir, crate["root_module"])
        if crate.get("source"):
            if "exclude_dirs" in crate["source"]:
                crate["source"]["exclude_dirs"] = [
                    os.path.join(kdir, e_dir) for e_dir in crate["source"]["exclude_dirs"]
                ]
            if "include_dirs" in crate["source"]:
                crate["source"]["include_dirs"] = [
                    os.path.join(kdir, i_dir) for i_dir in crate["source"]["include_dirs"]
                ]

    # Finally, append this module as a crate
    rust_project["crates"].append({
        "display_name": root_module.removesuffix(".rs"),
        "root_module": root_module,
        "is_workspace_member": False,
        "is_proc_macro": False,
        "deps": [
            {"crate": index, "name": name}
            for name, index in crate_indices.items()
            if name == "kernel"
        ],
        "cfg": [],
        "edition": "2021",
        "env": {
            "RUST_MODFILE": "This is only for rust-analyzer"
        }
    })
    return rust_project


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--verbose', '-v', action='store_true')
    parser.add_argument("kdir", type=pathlib.Path)
    parser.add_argument("root_module", type=str)
    args = parser.parse_args()

    logging.basicConfig(
        format="[%(asctime)s] [%(levelname)s] %(message)s",
        level=logging.INFO if args.verbose else logging.WARNING
    )

    rust_project =  generate_rust_project(args.kdir, args.root_module)
    json.dump(rust_project, sys.stdout, sort_keys=True, indent=4)

if __name__ == "__main__":
    main()
