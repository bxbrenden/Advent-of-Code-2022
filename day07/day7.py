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
            puz = f.read().strip()
            return puz
    except (FileNotFoundError, PermissionError) as e:
        raise SystemExit(f"Failed to open file '{file_path}':\n{e}")


def main():
    puz = read_puzzle_input()
    print(puz)


if __name__ == "__main__":
    main()
