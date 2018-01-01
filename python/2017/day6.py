puzzle_input = input("Puzzle input? ")
puzzle_banks = [int(num) for num in puzzle_input.split('\t')]


# Part 1

def redistribute(banks):
    # find largest
    largest_index = banks.index(max(banks))
    value = banks[largest_index]
    banks[largest_index] = 0
    i = largest_index
    while value > 0:
        i += 1
        banks[i % len(banks)] += 1
        value -= 1


def find_redistribution_cycle(banks):
    banks = list(banks)
    seen_banks = set()
    cycles = 0

    while tuple(banks) not in seen_banks:
        cycles += 1
        seen_banks.add(tuple(banks))
        redistribute(banks)

    return cycles


print(find_redistribution_cycle(puzzle_banks))


# Part 2
def find_specific_redistribution_cycle(banks):
    banks = list(banks)
    seen_banks = dict()
    cycles = 0

    while tuple(banks) not in seen_banks:
        seen_banks[tuple(banks)] = cycles
        cycles += 1
        redistribute(banks)

    return cycles - seen_banks[tuple(banks)]


print(find_specific_redistribution_cycle(puzzle_banks))
