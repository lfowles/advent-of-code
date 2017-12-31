print("Puzzle input (blank line at end to finish)?")

puzzle_nums = []
while True:
    # Gather lines
    row = input()
    if len(row) == 0:
        break

    puzzle_nums.append([int(c) for c in row.split('\t')])

# Part 1
checksum = 0
for row in puzzle_nums:
    checksum += max(row) - min(row)

print(checksum)


# Part 2
def check(num1, num2):
    if num1 % num2 == 0:
        return num1 // num2
    elif num2 % num1 == 0:
        return num2 // num1
    else:
        return 0


def check_row(row):
    for lhs_i in range(len(row)):
        lhs = row[lhs_i]
        for rhs in row[lhs_i + 1:]:
            divide_num = check(lhs, rhs)
            if divide_num > 0:
                return divide_num
    raise Exception("No divisible numbers found in row: {}".format(row))


checksum = sum(check_row(row) for row in puzzle_nums)

print(checksum)
