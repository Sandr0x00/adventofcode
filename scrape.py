#!/usr/bin/env python3

import requests
import sys
from bs4 import BeautifulSoup
from readme_builder import build

if len(sys.argv) < 2:
    print("Usage: ./scrape.py <year> <day>")
    exit(-1)

URL = f"https://adventofcode.com/{sys.argv[1]}/day"

with open("session") as f:
    session = f.read()

day = int(sys.argv[2])
r = requests.get(f"{URL}/{day}", cookies={
    "session": session
})
soup = BeautifulSoup(r.text, "html.parser")

readme = ""

for article in soup.find_all("article"):
    print(article)
    readme += str(article)

with open(f"{sys.argv[1]}/{day:02d}/README.md", "w") as f:
    f.write(readme)

build(["2023", "2022"])