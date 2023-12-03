import re

one = 0
two = 0

with open("input") as f:
    field = f.read().strip().splitlines()

    # numbers per line
    numbers = []
    for line in field:
        numbers.append([m for m in re.finditer(r"\d+", line)])

    for y, line in enumerate(field):
        for x, c in enumerate(line):
            if re.match(r"[\.\d]", c):
                # skip numbers and dots
                continue

            parts_two = []
            def parts(row):
                global one
                for mn in numbers[row]:
                    if mn.span()[0] >= x + 2 or mn.span()[1] <= max(x - 1, 0):
                        # check if window relevant for us
                        continue
                    no = int(mn.group(0))
                    if c == "*":
                        parts_two.append(no)
                    one += no

            if y > 0:
                # not first row
                parts(y-1)
            parts(y)
            if y < len(field) - 1:
                # not last row
                parts(y+1)

            if len(parts_two) == 2:
                two += parts_two[0] * parts_two[1]


print("Part One", one)
print("Part Two", two)
