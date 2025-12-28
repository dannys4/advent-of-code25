import os


def overclock_joltage(bank: str) -> int:
    def inner(start_idx: int, remaining: int) -> int:
        largest = 0
        largest_idx = 0
        for i in range(start_idx, len(bank) - remaining + 1):
            v = int(bank[i])
            if v > largest:
                largest = v
                largest_idx = i
        if remaining > 1:
            return largest * 10**(remaining - 1) + inner(largest_idx + 1, remaining - 1)
        else:
            return largest
    return inner(0, 12)


if __name__ == '__main__':
    script_path = os.path.dirname(os.path.realpath(__file__))
    with open(script_path + "/../../data/day3.txt", 'r') as f:
        banks = f.readlines()
    print(sum(overclock_joltage(bank.strip()) for bank in banks))
