from collections import defaultdict
from itertools import accumulate
import sys

with open(sys.argv[1]) as f:
    lines = list(map(str.strip, f.readlines()))

dir_sizes: dict[str, int] = defaultdict(int)
cwd = []

for line in lines:
    match line.split():
        case "$", "cd", "/":
            cwd = ["/"]

        case "$", "cd", "..":
            cwd.pop()

        case "$", "cd", path:
            cwd.append(path + "/")

        case ("$", "ls") | ("dir", _):
            pass

        case file_size, _:
            for directory in accumulate(cwd):
                dir_sizes[directory] += int(file_size)

part1 = sum(size for size in dir_sizes.values() if size <= 100_000)
print(part1)

total_size = dir_sizes["/"]
unused_space = 70000000 - total_size
required_unused_space = 30000000
part2 = (
    min(
        unused_space + size
        for size in dir_sizes.values()
        if unused_space + size >= required_unused_space
    )
    - unused_space
)
print(part2)
