TARGET_DEL = 2_476_859

with open("secondary_input.txt", "r") as si:
    second = si.read().strip().split("\n")

potentials = []

for s in second:
    spl = s.split()
    size = int(spl[-1])
    if size >= TARGET_DEL:
        potentials.append(size)

potentials.sort()

print(potentials[0])
