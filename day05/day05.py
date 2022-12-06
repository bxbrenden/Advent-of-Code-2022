import sys
from collections import deque


def read_puzzle_input():
    if len(sys.argv) == 1 or len(sys.argv) > 2:
        raise SystemExit("USAGE: python3 day05.py <PUZZLE_INPUT>")
    elif len(sys.argv) == 2:
        file_path = sys.argv[1]
    try:
        with open(file_path, "r") as f:
            file_str = f.read()
            return file_str
    except (FileNotFoundError, PermissionError) as e:
        raise SystemExit(f'Failed to open file "{file_path}": {e}')


def split_crates_and_instructions(puz):
    """Given puzzle input, split into crates and instructions."""
    spl = puz.split("\n\n")
    crates = spl[0]
    instructions = spl[1]
    return (crates, instructions)


def crates_to_deques(crates):
    """Given a raw crates string, make a Pandas DataFrame out of it.

        Crates:
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
    """
    rows = crates.split("\n")[:-1]

    col1 = deque()
    col2 = deque()
    col3 = deque()
    col4 = deque()
    col5 = deque()
    col6 = deque()
    col7 = deque()
    col8 = deque()
    col9 = deque()

    for row in rows:
        for i in range(1, len(row) + 1, 4):
            if i == 1:
                if row[i] != ' ':
                    col1.append(row[i])
            elif i == 5:
                if row[i] != ' ':
                    col2.append(row[i])
            elif i == 9:
                if row[i] != ' ':
                    col3.append(row[i])
            elif i == 13:
                if row[i] != ' ':
                    col4.append(row[i])
            elif i == 17:
                if row[i] != ' ':
                    col5.append(row[i])
            elif i == 21:
                if row[i] != ' ':
                    col6.append(row[i])
            elif i == 25:
                if row[i] != ' ':
                    col7.append(row[i])
            elif i == 29:
                if row[i] != ' ':
                    col8.append(row[i])
            elif i == 33:
                if row[i] != ' ':
                    col9.append(row[i])
            else:
                print(f"Unexpected column value: {i}")

    col1.reverse()
    col2.reverse()
    col3.reverse()
    col4.reverse()
    col5.reverse()
    col6.reverse()
    col7.reverse()
    col8.reverse()
    col9.reverse()

    columns = [col1, col2, col3, col4, col5, col6, col7, col8, col9]

    return columns


def break_up_instructions(inst_str):
    """Given a newline-delimited string of instructions, split them up."""
    if not isinstance(inst_str, str):
        print(
                f"Expected string of instructions, got type {type(inst_str)}"
        )
    return inst_str.strip().split("\n")


def process_inst(inst):
    """Given a string like "move 1 from 2 to 1", return a tuple (1, 2, 1).

    move 1 from 2 to 1
    move 3 from 1 to 3
    """

    spl = inst.split(" from ")
    num_pops = int(spl[0].replace("move ", ""))

    spl2 = spl[1].split(" to ")
    src_col = int(spl2[0])
    dst_col = int(spl2[1])

    return (num_pops, src_col - 1, dst_col - 1)


def use_crane(crates, instructions):
    """The main machinery of the puzzle.

    Given a list of three-tuple instructions like (1, 2, 1), run pop()
    on the source deque (column) N times where N == instruction[0].
    """
    print(f"Initial state of crates:\n{crates}")
    for inst in instructions:
        num_crates, src, dst = process_inst(inst)
        # print(f"Moving {num_crates} crates from col. {src} to col. {dst}")
        # print(f"Inst: {inst}")
        print(crates)
        for n in range(num_crates):
            # print(f"popping from column {src}")
            try:
                crate = crates[src].pop()
            except IndexError:
                print("❌ Failed to pop from an empty deque")
                continue
            # print(f"appending to column {dst}")
            crates[dst].append(crate)
            # print(crates)
    print(crates)

    return crates


def get_answer(crates):
    """Given the crates in their final resting state, get the answer."""
    answer_list = []
    for crate in crates:
        try:
            a = crate.pop()
            answer_list.append(a)
        except IndexError:
            continue

    answer = "".join(answer_list)

    return answer


def main():
    puz = read_puzzle_input()
    crates, instructions = split_crates_and_instructions(puz)
    instructions = break_up_instructions(instructions)
    print(f"Crates (raw):\n{crates}")
    columns = crates_to_deques(crates)
    print(f"Columns (processed): {columns}")

    final = use_crane(columns, instructions)
    answer = get_answer(final)
    print(f"Answer: {answer}")


if __name__ == "__main__":
    main()
