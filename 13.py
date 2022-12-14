from __future__ import annotations
import json
import sys
from enum import Enum, auto

DEBUG = False
PacketData = int | list["PacketData"]

Packet = list[PacketData]


def parse_packet(s: str) -> Packet:
    return json.loads(s)


class Result(Enum):
    WRONG = auto()
    RIGHT = auto()
    LEFT_EXHAUSTED = auto()
    EQUAL_LENGTH = auto()
    RIGHT_EXHAUSTED = auto()

    @classmethod
    def from_(cls, boolean: bool) -> Result:
        return Result.RIGHT if boolean else Result.WRONG

    def __bool__(self) -> bool:
        return self in (
            Result.RIGHT,
            Result.LEFT_EXHAUSTED,
            Result.EQUAL_LENGTH,
        )


def right_order(left_packet: Packet, right_packet: Packet) -> Result:
    for left, right in zip(left_packet, right_packet):
        match left, right:
            case int(left), int(right):
                if left == right:
                    continue

                return Result.from_(left < right)

            case [*xs], [*ys]:
                match right_order(xs, ys):
                    case Result.RIGHT | Result.LEFT_EXHAUSTED:
                        return Result.RIGHT

                    case Result.WRONG | Result.RIGHT_EXHAUSTED:
                        return Result.WRONG

                    case Result.EQUAL_LENGTH:
                        continue

            case int(x), [*ys]:
                return right_order([x], ys)

            case [*xs], int(y):
                return right_order(xs, [y])

    if len(left_packet) < len(right_packet):
        return Result.LEFT_EXHAUSTED

    if len(left_packet) == len(right_packet):
        return Result.EQUAL_LENGTH

    return Result.RIGHT_EXHAUSTED


with open(sys.argv[1]) as f:
    contents = f.read()

result = 0
for i, pair in enumerate(contents.split("\n\n"), 1):
    a, b = pair.split("\n")
    left, right = parse_packet(a), parse_packet(b)

    if right_order(left, right):
        result += i

print(result)  # 3050 is not the right answer


with open(sys.argv[1]) as f:
    contents = f.read()

packets: list[Packet] = []


for pair in contents.split("\n\n"):
    a, b = pair.split("\n")
    packets.extend([parse_packet(a), parse_packet(b)])
