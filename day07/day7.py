from pathlib import Path
import sys


class Dir:
    def __init__(self, path, files=None, children=None):
        if children is not None:
            self.children = children
        else:
            self.children = dict()
        if files is not None:
            self.files = files
        else:
            self.files = []
        self.path = Path(path)
        self.cwd = self.path

    def __repr__(self):
        child_names = [x for x in self.children.keys()]
        file_names = [x.split()[1] for x in self.files]
        return f"Dir<{self.path.absolute()}, cwd: {self.cwd}, children: {child_names}, files: {file_names}>"

    def tree(self):
        num_slashes = str(self.path.absolute()).count("/")
        indent = " " * (num_slashes * 2)
        main_indent = indent[:-2]
        if str(self.path.absolute()) == "/":
            print(f"{main_indent}- / (dir)")
        else:
            print(f"{main_indent}-", self.path.name)
        for file in self.files:
            spl = file.split()
            print(f"{indent}- {spl[1]} (file, size={spl[0]})")

        for child in self.children:
            print(f"{indent}- {child} (dir)")
            self.children[child].tree()

    def files_size(self):
        size = 0
        for file in self.files:
            size += int(file.split()[0])
        return size

    def size(self):
        size = 0
        for child in self.children:
            child_size = self.children[child].size()
            print(f"{self.children[child].path.absolute()}: {child_size}")
            size += child_size
        size += self.files_size()
        return size


root_dir = Dir("/")


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


def process_commands(iter_num, lines, direc):
    """Process the Unix commands."""
    global root_dir
    # print(f"Iteration: {iter_num}, direc: {direc}, root_dir: {root_dir}")
    line = lines[0]
    # print(f"Current raw line: {line}")
    if line.startswith("$ cd"):
        # print(f"Found a `cd` command: {line}")
        # a `cd /` command happens only once at the beginning of every file
        if line == '$ cd /':
            if len(lines) > 1:
                iter_num += 1
                return process_commands(iter_num, lines[1:], direc)
        elif line == '$ cd ..':
            if len(lines) > 1:
                iter_num += 1
                direc.cwd = direc.cwd.parent.absolute()
                # use direc.cwd to traverse the tree
                path_list = str(direc.cwd).split("/")[1:]
                new_direc_str = "root_dir.children["
                pll = len(path_list)
                for i, pl in enumerate(path_list):
                    if pl == "":
                        pl = "/"
                    new_direc_str += "'" + pl + "'"
                    if i < pll - 1:
                        new_direc_str += "].children["
                    else:
                        new_direc_str += "]"
                # print(f"New direc str: {new_direc_str}")
                if new_direc_str == "root_dir.children['/']":
                    new_direc = root_dir
                else:
                    new_direc = eval(new_direc_str)
                return process_commands(iter_num, lines[1:], new_direc)
            else:
                raise SystemExit("No previous directory to return to")
        else:
            new_cwd = line.replace("$ cd ", "")
            # print(f"new_cwd value: {new_cwd}")
            if len(lines) > 1:
                iter_num += 1
                return process_commands(iter_num, lines[1:], direc.children[new_cwd])
        # print(f"Current directory is now: {direc.path.absolute()}")
    elif line.startswith("$ ls"):
        # print(f"found an `ls` command: {line}")
        dir_lines = []
        cont_line = 1
        for line_num, l in enumerate(lines[1:]):
            if not l.startswith("$"):
                # print(f"Appending {l} to dir_lines")
                dir_lines.append(l)
            else:
                cont_line += line_num
                # print("End of files and child dirs for this dir")
                break
        for dl in dir_lines:
            if dl.startswith("dir"):
                new_dirname = dl.split()[1]
                d = Dir(direc.path.absolute() / new_dirname)
                # print(f"Creating new child dir: {d}")
                # print(f"The dict key for the new child dir is: {new_dirname}")
                direc.children[new_dirname] = d
            elif dl.startswith("$"):
                raise SystemExit("This should never happen")
            else:
                # print(f"Found a file: {dl}")
                # print(f"Appending file to directory \"{direc.path.absolute()}\": {dl}")
                direc.files.append(dl)
        if len(lines) > 1:
            # print(f"cont_line value: {cont_line}")
            iter_num += 1
            return process_commands(iter_num, lines[cont_line:], direc)


def is_command(line):
    """Return True if the line is a Unix command, else False."""
    return line.startswith("$ ")


def main():
    global root_dir
    puz = read_puzzle_input()
    # print("Creating root directory with path '/'")
    iter_num = 1

    process_commands(iter_num, puz, root_dir)
    print(f"/: {root_dir.size()}\n")
    # root_dir.tree()


if __name__ == "__main__":
    main()
