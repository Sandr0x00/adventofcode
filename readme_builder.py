#!/usr/bin/env python3

import re
import os

def build():
    readme = """\
# Advent of Code

## 2023

"""

    for item in sorted(os.listdir()):
        if not os.path.isdir(item):
            continue

        if not os.path.exists(f"{item}/README.md"):
            continue

        with open(f"{item}/README.md") as f:
            desc = re.search(r"--- (Day [^<]+)", f.read()).group(1)
            assert desc[-4:] == " ---"
            desc = desc[:-4]
            readme += f"- [{desc}]({item})\n"

    with open("README.md", "w") as f:
        f.write(readme)

if __name__ == "__main__":
    build()