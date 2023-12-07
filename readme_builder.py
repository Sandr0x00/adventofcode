#!/usr/bin/env python3

import re
import os

def build(years):
    readme = """\
# Advent of Code
"""

    for year in years:
        readme += f"""
## {year}

"""

        for item in sorted(os.listdir(year)):
            if not os.path.isdir(f"{year}/{item}"):
                continue

            if not os.path.exists(f"{year}/{item}/README.md"):
                continue

            language = None
            if os.path.exists(f"{year}/{item}/solve.py"):
                language = "lang-python.svg"
            elif os.path.exists(f"{year}/{item}/Cargo.toml"):
                language = "lang-rust.svg"


            with open(f"{year}/{item}/README.md") as f:
                desc = re.search(r"--- (Day [^<]+)", f.read()).group(1)
                assert desc[-4:] == " ---"
                desc = desc[:-4]
                readme += "- "
                if language:
                    readme += f'<img src="res/{language}" width="16" /> '
                readme += f'[{desc}]({year}/{item})\n'

    with open("README.md", "w") as f:
        f.write(readme)

if __name__ == "__main__":
    build(["2023", "2022"])