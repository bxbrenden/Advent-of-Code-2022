import sys

def read_puzzle_input():
    if len(sys.argv) == 1:
        file_path = "sample_input.txt"
    elif len(sys.argv) == 2:
        file_path = sys.argv[1]
    try:
        with open(file_path, "r") as f:
            file_str = f.read().strip()
            return file_str
    except (FileNotFoundError, PermissionError) as e:
        raise SystemExit(f"Failed to open file \"{file_path}\": {e}")


def main():
    puz = read_puzzle_input()
    print(puz)


if __name__ == "__main__":
    main()
