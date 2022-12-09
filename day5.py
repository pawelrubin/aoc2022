
from collections import defaultdict
import sys
from typing import Iterable, Iterator, TypeVar

with open(sys.argv[1]) as f:
    lines = list(map(str.strip, f.readlines()))


T = TypeVar("T")
def chunks(iterable: Iterable[T], size: int) -> Iterator

crates: defaultdict[list[str]] = defaultdict(list)

# num_of_stack = len((lines[0] + 1) / 4)
for line in lines:
    for 
