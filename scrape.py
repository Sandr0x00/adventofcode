#!/usr/bin/env python3

import requests
import sys
from bs4 import BeautifulSoup

URL = "https://adventofcode.com/2023/day"

with open("session") as f:
    session = f.read()

day = int(sys.argv[1])
r = requests.get(f"{URL}/{day}", cookies={
    "session": session
})
soup = BeautifulSoup(r.text, "html.parser")

readme = ""

for article in soup.find_all("article"):
    print(article)
    readme += str(article)

with open(f"{day:02d}/README.md", "w") as f:
    f.write(readme)