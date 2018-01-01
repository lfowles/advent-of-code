print("Puzzle input (blank line at end to finish)?")

puzzle_nums = []
while True:
    # Gather lines
    offset = input()
    if len(offset) == 0:
        break

    puzzle_nums.append(int(offset))

# Part 1
def steps_to_exit(jumps):
    jumps = list(jumps)
    position = 0
    step = 0
    while 0 <= position < len(jumps):
        jump_offset = jumps[position]
        jumps[position] += 1
        position += jump_offset
        step += 1

    return step

print(steps_to_exit(puzzle_nums))

# Part 2
def steps_to_exit2(jumps):
    jumps = list(jumps)
    position = 0
    step = 0
    while 0 <= position < len(jumps):
        # print("Step {}: Position {} Content {}".format(step+1, position, jumps[position]))
        jump_offset = jumps[position]
        if jumps[position] >= 3:
            jumps[position] -= 1
        else:
            jumps[position] += 1
        position += jump_offset
        step += 1

    return step

print(steps_to_exit2(puzzle_nums))