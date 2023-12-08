#!/usr/bin/env python3

import requests
import sys
from bs4 import BeautifulSoup
import argparse
import os
import re
import datetime

with open("session") as f:
    session = f.read()

URL = f"https://adventofcode.com"

def get_folder(year, day):
    return f"{year}/src/day_{day:02d}"


def request(url):
    print(url)
    r = requests.get(url, cookies={
        "session": session
    })
    assert r.status_code == 200
    return r.text


def download_input(year, day, force):
    url = f"{URL}/{year}/day/{day}/input"
    folder = get_folder(year, day)
    input_file = f"{folder}/input.txt"

    if not os.path.exists(folder):
        os.mkdir(folder)

    if not force and os.path.exists(input_file):
        print(f"File exists: {input_file}")
        return

    r = request(url)

    with open(input_file, "w") as f:
        f.write(r)


def download_readme(year, day, force):
    url = f"{URL}/{year}/day/{day}"
    folder = get_folder(year, day)
    readme_file = f"{folder}/README.md"

    if not force and os.path.exists(readme_file):
        print(f"File exists: {readme_file}")
        return

    r = request(url)

    soup = BeautifulSoup(r, "html.parser")

    readme = ""
    articles = soup.find_all("article")
    if len(articles) != 2:
        print(f"Part One not yet solved {url}")
        return
    for article in articles:
        # print(article)
        readme += str(article)

    with open(readme_file, "w") as f:
        f.write(readme)


def build_readme():
    readme = """\
# Advent of Code
"""

    years = os.listdir()
    years = [y for y in years if len(y) == 4 and y.startswith("20")]

    for year in years:
        readme += f"""
## {year}

"""

        for day in range(1, 26):
            folder = get_folder(year, day)
            if not os.path.exists(folder):
                continue

            if not os.path.exists(f"{folder}/README.md"):
                continue

            language = None
            if os.path.exists(f"{folder}/solve.py"):
                language = "lang-python.svg"
            elif os.path.exists(f"{folder}/mod.rs"):
                language = "lang-rust.svg"

            with open(f"{folder}/README.md") as f:
                desc = re.search(r"--- (Day [^<]+)", f.read()).group(1)
                assert desc[-4:] == " ---"
                desc = desc[:-4]
                readme += "- "
                if language:
                    readme += f'<img src="res/{language}" width="16" /> '
                readme += f'[{desc}]({folder})\n'

    with open("README.md", "w") as f:
        f.write(readme)


def leaderboard(year):
    me = "Sandr0x00"

    boards = {
        1681151: "hxp",
        3569128: "TUM",
        3559813: "SCS",
    }

    print("               1111111111222222")
    print("      1234567890123456789012345")

    for board, name in boards.items():
        url = f"{URL}/{year}/leaderboard/private/view/{board}"

        print()
        print(name, end=" - ")

        r = request(url)

        soup = BeautifulSoup(r, "html.parser")
        for row in soup.find_all(class_="privboard-row")[1:]:
            row = row.decode_contents()
            if '<span class="privboard-position">' in row:
                pos = re.search(r'\)</span>(?P<pos>[^<]+)<span', row).group(1)
                row = re.sub(r'\)</span>(?P<pos>[^<]+)<span', f'.{pos:>4}<span', row)
            row = row.replace('</span>', '')
            row = re.sub(r'<span class="privboard-(name|position)">', '', row)
            row = row.replace('<span class="privboard-star-both">*', '\x1b[0;33m*\x1b[0m')
            row = row.replace('<span class="privboard-star-firstonly">*', '\x1b[0;36m*\x1b[0m')
            row = re.sub(r'<span class="privboard-star-(unlocked|locked)">\*', r' ', row)
            row = re.sub(r'<a href="[^"]+" target="_blank">([^<]+)</a>', r"\1", row)
            row = row.replace(me, f'\x1b[0;33m{me}\x1b[0m')
            supporter_badge = '<a class="supporter-badge"'
            if supporter_badge in row:
                row = row[:row.index(supporter_badge)]
            x_button = '<input class="privboard-delbtn"'
            if x_button in row:
                row = row[:row.index(x_button)]
            print(row)


if __name__ == "__main__":
    today = datetime.date.today()
    year = today.year
    day = today.day

    parser = argparse.ArgumentParser()
    parser.add_argument("-y", "--year", type=int, default=year)
    parser.add_argument("--force", action="store_true", default=False)
    subparsers = parser.add_subparsers(dest="command", required=True)

    sub_parser = subparsers.add_parser("leaderboard")

    sub_parser = subparsers.add_parser("input")
    sub_parser.add_argument("-d", "--day", type=int, default=day)

    sub_parser = subparsers.add_parser("readme")
    sub_parser.add_argument("-d", "--day", type=int, default=day)

    args = parser.parse_args()

    match args.command:
        case "leaderboard":
            leaderboard(args.year)

        case "input":
            if year != args.year:
                for day in range(1,26):
                    download_input(args.year, day, args.force)
            download_input(args.year, args.day, args.force)

        case "readme":
            if year != args.year:
                for day in range(1,26):
                    download_readme(args.year, day, args.force)
            download_readme(args.year, args.day, args.force)
            build_readme()
