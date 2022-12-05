from collections import deque
import sys

import pandas as pd


def read_puzzle_input():
    if len(sys.argv) == 1 or len(sys.argv) > 2:
        file_path = "sample_input.txt"
    elif len(sys.argv) == 2:
        file_path = sys.argv[1]
    try:
        with open(file_path, "r") as f:
            file_str = f.read().strip()
            return file_str
    except (FileNotFoundError, PermissionError) as e:
        raise SystemExit(f'Failed to open file "{file_path}": {e}')


def split_crates_and_instructions(puz):
    """Given puzzle input, split into crates and instructions."""
    spl = puz.split("\n\n")
    crates = spl[0]
    instructions = spl[1]
    return (crates, instructions)


def crates_to_dataframe(crates):
    """Given a raw crates string, make a Pandas DataFrame out of it.

        Crates:
    [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3
    """
    rows = crates.strip().split("\n")
    num_cols = len(rows[-1].strip().split())
    print(f"There are {num_cols} columns")

    col1 = deque()
    col2 = deque()
    col3 = deque()

    for index, row in enumerate(rows[:-1]):
        for i in range(1, len(row) - 1, 4):
            if i == 1:
                col1.append(row[i])
            elif i == 5:
                col2.append(row[i])
            elif i == 9:
                col3.append(row[i])
            else:
                print(f"Unexpected column value: {i}")

    print(f"Column 1: {col1}")
    print(f"Column 2: {col2}")
    print(f"Column 3: {col3}")


def main():
    puz = read_puzzle_input()
    # print(puz)
    crates, instructions = split_crates_and_instructions(puz)
    print(f"Crates:\n{crates}")
    print(f"Instructions:\n{instructions}")
    crates_to_dataframe(crates)


if __name__ == "__main__":
    main()
