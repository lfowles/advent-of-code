puzzle_input = input("Puzzle input? ").strip()

# Part 1
sum = 0
for i in range(len(puzzle_input)):
    if puzzle_input[i] == puzzle_input[(i + 1) % len(puzzle_input)]:
        sum += ord(puzzle_input[i]) - ord('0')

print(sum)

# Part 2
sum = 0
half = len(puzzle_input) // 2
for i in range(len(puzzle_input) - half):
    if puzzle_input[i] == puzzle_input[i + half]:
        sum += ord(puzzle_input[i]) - ord('0')

sum *= 2

print(sum)
