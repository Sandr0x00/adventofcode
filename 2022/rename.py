
import os

for i in range(1, 26):
    with open(f"{i:02d}/input") as f:
        a = f.read()

    with open(f"input_{i:02d}.txt", "w") as f:
        f.write(a)