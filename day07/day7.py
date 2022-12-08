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
        dir_lines = []
        cont_line = 0
        for line_num, line in enumerate(lines[1:]):
            if not line.startswith("$"):
                dir_lines.append(line)
            else:
                cont_line = line_num
        for dl in dir_lines:
            if not dl.startswith("dir"):
                spl = dl.split()
                byte_val = int(spl[0])
                filename = spl[1]
                fs_bytes += byte_val
                print(f"file {filename}: {byte_val} bytes")
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
    print(f"Total bytes: {fs_bytes}")

if __name__ == "__main__":
    main()
