with open("secondary_input.txt", "r") as si:
    second = si.read().strip().split("\n")

total = 0

for s in second:
    spl = s.split()
    size = int(spl[-1])
    if size <= 100_000:
        total += size

print(total)
