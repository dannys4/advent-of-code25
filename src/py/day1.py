def sign(dir):
    return -1 if dir == 'L' else 1


filename = 'data/day1.txt'
with open(filename, 'r') as f:
    # dial = f.readlines()
    lines = f.readlines()

position: int = 50
zero_count: int = 0

for (j, line) in enumerate(lines):
    line = line.strip()
    dir = line[0]
    val = int(line[1:])

    full, partial = divmod(val, 100)
    zero_count += full

    delta = partial * sign(dir)
    next_position = position + delta

    if position != 0:
        if dir == "L" and next_position <= 0:
            zero_count += 1
        elif dir == "R" and next_position >= 100:
            zero_count += 1

    position = next_position % 100
    print(f'{j}:\t{position}\t{zero_count}')

print(f"final count: {zero_count}")
