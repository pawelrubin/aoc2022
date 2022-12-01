import sys
from typing import Callable, Iterator, Sequence, TypeVar


T = TypeVar("T")


def split(sequence: Sequence[T], predicate: Callable[[T], bool]) -> Iterator[list[T]]:
    bucket: list[T] = []
    for elem in sequence:
        if predicate(elem):
            yield bucket
            bucket.clear()
        else:
            bucket.append(elem)


with open(sys.argv[1]) as f:
    lines = list(map(str.strip, f.readlines()))


elves = sorted(
    (sum(map(int, group)) for group in split(lines, lambda line: line == "")),
    reverse=True,
)

print(elves[0])
print(sum(elves[::-3]))


