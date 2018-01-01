print("Puzzle input (blank line at end to finish)?")

passphrases = []
while True:
    # Gather lines
    passphrase = input()
    if len(passphrase) == 0:
        break

    passphrases.append(passphrase)

# Part 1

def is_valid_passphrase(passphrase):
    words = passphrase.split(' ')
    word_set = set(words)
    return len(word_set) == len(words)

sum = 0
for passphrase in passphrases.split('\n'):
    if is_valid_passphrase(passphrase):
        sum += 1

print(sum)

# Part 2

def is_valid_passphrase2(passphrase):
    words = passphrase.split(' ')
    word_set = set(tuple(sorted(word)) for word in words)
    return len(word_set) == len(words)


sum = 0
for passphrase in puzzle_input.split('\n'):
    if is_valid_passphrase2(passphrase):
        sum += 1
