#!/usr/bin/env python3

import requests
import sys
import os

year = 0
day = 0

if len(sys.argv) == 2:
    _, year = sys.argv
elif len(sys.argv) == 3:
    _, year, day = sys.argv
else:
    print("Usage: ./scrape.py <year> <day>")
    exit(-1)

with open("session") as f:
    session = f.read()

def download(year, day):
    input_file = f"{year}/{day:02d}/input"

    if not os.path.exists(f"{year}/{day:02d}"):
        os.mkdir(f"{year}/{day:02d}")

    if os.path.exists(input_file):
        print("File exists")
        return

    url = f"https://adventofcode.com/{year}/day/{day}/input"
    print(f"Download {url}")
    r = requests.get(url, cookies={
        "session": session
    })
    assert r.status_code == 200

    with open(input_file, "wb") as f:
        f.write(r.content)


if day != 0:
    download(year, day)
else:
    for day in range(1, 26):
        download(year, day)