from __future__ import annotations
import json
import sys

DEBUG = False
Packet = int | list["Packet"]


def parse_packet(s: str) -> list[Packet]:
    return json.loads(s)


def right_order(left: list[Packet], right: list[Packet], indent=0) -> bool:
    for a, b in zip(left, right):
        if DEBUG:
            print((indent + 1) * " " + f"- Compare {a} vs {b}")

        match a, b:
            case int(a), int(b):
                if a == b:
                    continue

                return a < b

            case [*xs], [*ys]:
                if right_order(xs, ys, indent + 1):
                    if len(xs) == len(ys):
                        continue

                    return len(xs) < len(ys)

                return False

            case int(x), [*ys]:
                return right_order([x], ys, indent + 1)

            case [*xs], int(y):
                return right_order(xs, [y], indent + 1)

    return len(left) <= len(right)


with open(sys.argv[1]) as f:
    contents = f.read()

result = 0
for i, pair in enumerate(contents.split("\n\n"), 1):
    a, b = pair.split("\n")
    a, b = parse_packet(a), parse_packet(b)
    if right_order(a, b):
        if DEBUG:
            print("\nRIGHT\n")
        result += i
    elif DEBUG:
        print("\nWRONG\n")

print(result)  # 3050 is not the right answer
