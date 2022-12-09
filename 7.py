from __future__ import annotations

import sys
import json

with open(sys.argv[1]) as f:
    lines = list(map(str.strip, f.readlines()))

file_system_path_table: dict[str, int] = {}

_, cwd = lines[0].split("$ cd ")

for line in lines[1:]:
    if line == "$ cd ..":
        cwd = "/".join(cwd.split("/")[:-2]) + "/"
    elif line.startswith("$ cd "):
        cwd = cwd + line.split("$ cd ")[1] + "/"
    elif line.startswith("$ ls") or line.startswith("dir"):
        pass
    else:
        size, name = line.split(" ")
        file_system_path_table[cwd + name] = int(size)
    # print(cwd)

# TreeDict = dict[str, int | "TreeDict"]


def parse_path_table(path_table: dict[str, int]):
    tree = {"/": {}}
    for path, file_size in path_table.items():
        subdirs = path.split("/")[1:]

        current_dir = tree["/"]

        for subdir in subdirs[:-1]:
            current_dir[subdir] = current_dir.get(subdir, {})
            current_dir = current_dir[subdir]

        current_dir[subdirs[-1]] = file_size

    return tree

def dir_size(tree: dict) -> int:
    for name, value in tree.items():
        if isinstance(value)

# print(json.dumps(dict(sorted(file_system_path_table.items()))))
print(json.dumps(parse_path_table(file_system_path_table)))

