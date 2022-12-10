import sys
import math

with open(sys.argv[1]) as f:
    lines = list(map(str.strip, f.readlines()))

grid = [list(map(int, line)) for line in lines]

result = 0

for i, row in enumerate(grid):
    for j, tree in enumerate(row):
        tree_lines = (
            row[:j],
            row[j + 1 :],
            [row[j] for row in grid[:i]],
            [row[j] for row in grid[i + 1 :]],
        )
        if any(all(tree > other for other in line) for line in tree_lines):
            result += 1

print(result)


def take_until(predicate, iterable):
    for item in iterable:
        yield item
        if predicate(item):
            return


scenic_scores = []

for i, row in enumerate(grid):
    for j, tree in enumerate(row):
        tree_lines = (
            list(reversed(row[:j])),  # left
            row[j + 1 :],  # right
            list(reversed([row[j] for row in grid[:i]])),  # up
            [row[j] for row in grid[i + 1 :]],  # down
        )

        viewing_distances = [
            len(list(take_until(lambda other: other >= tree, line)))
            for line in tree_lines
        ]

        scenic_scores.append(math.prod(viewing_distances))


print(max(scenic_scores))
