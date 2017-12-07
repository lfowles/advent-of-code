puzzle_input = int(input("Puzzle input? "))


# Part 1
def ring_size(ring_num):
    if ring_num == 0:
        return 1
    return 8 * ring_num


def ring_max(ring_num):
    return (2 * ring_num + 1) ** 2 - 1


def ring_min(ring_num):
    return ring_max(ring_num) - ring_size(ring_num) + 1


def edge_distance(ring_num):
    return ring_size(ring_num) // 4


def ring_cardinal_index(ring_num, direction):
    if not (0 <= direction <= 3):
        raise Exception("Bad direction")
    return ring_min(ring_num) + ring_num - 1 + direction * edge_distance(ring_num)


def get_distance(index):
    # memory index is 1 based
    index -= 1

    ring_num = 0
    while ring_max(ring_num) < puzzle_input:
        ring_num += 1

    cardinals = (ring_cardinal_index(ring_num, dir) for dir in range(4))
    differences = (abs(cardinal - index) for cardinal in cardinals)
    ring_distance = min(differences)

    total_distance = ring_num + ring_distance
    return total_distance


print("input {} is distance {}".format(puzzle_input, get_distance(puzzle_input)))

# Part 2
import math

from typing import Iterable


class SpiralMemory(object):
    def __init__(self, memory: Iterable[int]):
        self.memory = list(memory)

    def __getitem__(self, key):
        x, y = key
        index = self.get_index(x, y)
        if index >= len(self.memory):
            return 0
        else:
            return self.memory[index]

    def __setitem__(self, key, value):
        row_off, col_off = key
        index = self.get_index(x, y)
        if index >= len(self.memory):
            self.memory.extend([0] * (index + 1 - len(self.memory)))
        self.memory[index] = value

    def get_index(self, x, y):

        if y == 0 and x == 0:
            return 0

        ring = max(abs(y), abs(x))
        if abs(y) < abs(x):
            ring_index = y
        else:
            ring_index = x

        start_index = 1
        for i in range(1, ring):
            start_index += ring_size(i)

        # if they are equal, let the y>=x case handle it to deal with the bottom right corner correctly
        if abs(x) > abs(y):
            if x > 0:
                east_index = start_index + ring - 1
                assert (east_index + ring_index < start_index + ring_size(ring))
                return east_index + ring_index
            else:
                west_index = start_index + ring - 1 + 2 * edge_distance(ring)
                assert (west_index + ring_index < start_index + ring_size(ring))
                return west_index - ring_index
        elif abs(y) >= abs(x):
            if y > 0:
                north_index = start_index + ring - 1 + edge_distance(ring)
                assert (north_index + ring_index < start_index + ring_size(ring))
                return north_index - ring_index
            else:
                south_index = start_index + ring - 1 + 3 * edge_distance(ring)
                assert (south_index + ring_index < start_index + ring_size(ring))
                return south_index + ring_index

    def get_offsets(self, index):
        if index == 0:
            return (0, 0)

        ring_start = 1
        ring = 1
        while index >= ring_start + ring_size(ring):
            ring_start += ring_size(ring)
            ring += 1

        ring_index = index - ring_start
        east_index = ring - 1
        north_index = east_index + edge_distance(ring)
        west_index = north_index + edge_distance(ring)
        south_index = west_index + edge_distance(ring)
        dist = lambda cardinal_index: abs(ring_index - cardinal_index)
        min_cardinal_dist = min(dist(east_index), dist(north_index), dist(west_index), dist(south_index))

        if abs(ring_index - east_index) <= edge_distance(ring) // 2:
            return (ring, ring_index - east_index)
        elif abs(west_index - ring_index) <= edge_distance(ring) // 2:
            return (-ring, west_index - ring_index)
        elif abs(north_index - ring_index) <= edge_distance(ring) // 2:
            return (north_index - ring_index, ring)
        elif abs(ring_index - south_index) <= edge_distance(ring) // 2:
            return (ring_index - south_index, -ring)

    def print_me(self, ring=None):
        if ring is None:
            ring = max(abs(o) for o in self.get_offsets(len(self.memory) - 1))

        mem_size = max(self.get_index(ring, -ring) + 1, len(self.memory))
        digits = math.ceil(math.log10(max(self.memory[:mem_size])))
        format_str = "{{:{}d}}".format(digits)
        for y in range(ring, -ring - 1, -1):
            print(" ".join((format_str.format(mem[x, y]) for x in range(-ring, ring + 1))))


mem = SpiralMemory([1])

value = 0
i = 0
while value < puzzle_input:
    i += 1
    x, y = mem.get_offsets(i)
    # now sum in adjacent
    sum = 0
    for x_off in (-1, 0, 1):
        for y_off in (-1, 0, 1):
            # value is already zero, so adding in 0,0 is benign
            # And now that I've explained that, I could have already done it in the length of these two comments..
            sum += mem[x + x_off, y + y_off]

    mem[x, y] = sum
    value = sum

mem.print_me()
print("first value greater than {} is {}".format(puzzle_input, value))
