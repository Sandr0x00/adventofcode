import requests
from bs4 import BeautifulSoup
import re

URL = "https://adventofcode.com/2023/leaderboard/private/view"

me = "Sandr0x00"

boards = {
    1681151: "hxp",
    3569128: "TUM",
    3559813: "SCS",
}

with open("session") as f:
    session = f.read()

print("               1111111111222222")
print("      1234567890123456789012345")

for board, name in boards.items():
    print()
    print(name)
    r = requests.get(f"{URL}/{board}", cookies={
        "session": session
    })

    soup = BeautifulSoup(r.text, "html.parser")
    for row in soup.find_all(class_="privboard-row")[1:]:
        row = row.decode_contents()
        if '<span class="privboard-position">' in row:
            pos = re.search(r'\)</span>(?P<pos>[^<]+)<span', row).group(1)
            row = re.sub(r'\)</span>(?P<pos>[^<]+)<span', f'.{pos:>4}<span', row)
        row = row.replace('</span>', '')
        row = re.sub(r'<span class="privboard-(name|position)">', '', row)
        row = row.replace('<span class="privboard-star-both">*', '\x1b[0;33m*\x1b[0m')
        row = row.replace('<span class="privboard-star-firstonly">*', '*')
        row = row.replace('<span class="privboard-star-unlocked">*', ' ')
        row = row.replace('<span class="privboard-star-locked">*', ' ')
        row = re.sub(r'<a href="[^"]+" target="_blank">([^<]+)</a>', r"\1", row)
        row = row.replace(me, f'\x1b[0;33m{me}\x1b[0m')
        print(row)

