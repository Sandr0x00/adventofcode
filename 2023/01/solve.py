import re

MAP = {
    "one":   "1",
    "two":   "2",
    "three": "3",
    "four":  "4",
    "five":  "5",
    "six":   "6",
    "seven": "7",
    "eight": "8",
    "nine":  "9",
}

one = 0
two = 0
re_one = r"(\d)"
re_two = r"(\d|" + r"|".join(MAP.keys()) + r")"

with open("input") as f:
    for line in f.read().splitlines():
        if len(line) == 0:
            continue

        # part one
        first = re.search(re_one, line).group(1)
        second = re.search(r".*" + re_one, line).group(1)
        one += int(first + second)

        # part two
        first = re.search(re_two, line).group(1)
        second = re.search(r".*" + re_two, line).group(1)
        two += int((MAP[first] if first in MAP else first) + (MAP[second] if second in MAP else second))

print("Part One", one)
print("Part Two", two)
