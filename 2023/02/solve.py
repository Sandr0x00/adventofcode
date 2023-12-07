import re
from functools import reduce
import operator

bag = {
    "red":   12,
    "green": 13,
    "blue":  14,
}
colors = bag.keys()

one = 0
two = 0
with open("input") as f:
    for line in f.read().splitlines():
        if len(line) == 0:
            continue
        m = re.match(r"Game (?P<id>\d+)", line)
        game_id = int(m.group("id"))
        m = re.findall(r"[:;] [^;]+", line[m.span()[1]:])

        # part one
        for s in m:
            for c in colors:
                r = re.search(rf" (\d+) {c}", s)
                if r and int(r.group(1)) > bag[c]:
                    break
            else:
                continue
            break
        else:
            one += game_id

        # part two
        minimal = {c: 0 for c in colors}
        for s in m:
            for c in colors:
                r = re.search(rf" (\d+) {c}", s)
                if r and (r:=int(r.group(1))) > minimal[c]:
                    minimal[c] = r
        two += reduce(operator.mul, minimal.values())

print("Part One", one)
print("Part Two", two)
