from pathlib import Path
import sys


def read_puzzle_input():
    """Read the puzzle input and return it as a string."""
    if len(sys.argv) == 1:
        file_path = "sample_input.txt"
    elif len(sys.argv) == 2:
        file_path = sys.argv[1]
    else:
        raise SystemExit("Usage: python3 day7.py <PUZZLE_INPUT_FILE>")

    try:
        with open(file_path, "r") as f:
            puz = f.read().strip().split("\n")
            return puz
    except (FileNotFoundError, PermissionError) as e:
        raise SystemExit(f"Failed to open file '{file_path}':\n{e}")


def process_commands(lines, fs_bytes, cwd):
    """Process the Unix commands."""
    line = lines[0]
    if line.startswith("$ cd"):
        if line == '$ cd /':
            cwd = Path("/")
        elif line == '$ cd ..':
            cwd = cwd.parent.absolute()
        else:
            cwd /= line[5:]
        print(f"Current directory is now: {cwd}")
    elif line.startswith("$ ls"):
        print("found an `ls` command.")

    if len(lines) > 1:
        return process_commands(lines[1:], fs_bytes, cwd)
    else:
        return fs_bytes



def is_command(line):
    """Return True if the line is a Unix command, else False."""
    return line.startswith("$ ")


def main():
    puz = read_puzzle_input()
    cwd = Path("/")
    fs_bytes = 0

    fs_bytes = process_commands(puz, fs_bytes, cwd)

if __name__ == "__main__":
    main()
