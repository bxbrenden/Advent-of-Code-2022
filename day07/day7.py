from pathlib import Path
import sys


class Dir:
    """All of Dir's children are also of type Dir."""
    def __init__(self, path, files=None, children=None):
        if children is not None:
            self.children = children
        else:
            self.children = []
        if files is not None:
            self.files = files
        else:
            self.files = []
        self.path = Path(path)

    def __repr__(self):
        child_names = [x.path.name for x in self.children]
        file_names = [x.split()[1] for x in self.files]
        return f"Dir<{self.path.absolute()}, children: {child_names}, files: {file_names}>"

    def files_size(self):
        size = 0
        for file in self.files:
            size += int(file.split()[0])
        return size

    def size(self):
        size = 0
        for child in self.children:
            size += child.size()
        size += self.files_size()
        return size


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


def process_commands(lines, direc):
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
                print(f"file {filename}: {byte_val} bytes")

            else:
                #Create Dir objects.
                pass
    if len(lines) > 1:
        return process_commands(lines[1:], fs_bytes, cwd)
    else:
        return fs_bytes



def is_command(line):
    """Return True if the line is a Unix command, else False."""
    return line.startswith("$ ")


def main():
    puz = read_puzzle_input()
    root_dir = Dir("/")

    process_commands(puz, root_dir)
    print(f"Total bytes: {fs_bytes}")

if __name__ == "__main__":
    main()
