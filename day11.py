#!/usr/bin/env pipenv run python
"""Solutions to day 11 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

memo = {}

def get_power(*args):
    if args not in memo:
        x, y, serial = args
        memo[args] = (((x + 10) * y + serial) * (x + 10) // 100) % 10 - 5
    return memo[args]

def make_convolution(func, gid):
    grid = [[func(x, y, gid) for x in range(300)] for y in range(300)]
    for x, y in itertools.product(range(1,300), range(1, 300)):
        grid[x][y] += grid[x - 1][y] + grid[x - 1][y] - grid[x - 1][y - 1]
    return grid

def part1(gid):
    """Solution to part 1"""
    convolution = make_convolution(get_power, gid)
    for x, y, size in itertools.product(range(300), range(302), (3,)):
        if biggest is None or biggest < total:
            result = (x, y)
            biggest = total
    if part2:
        return result, biggest 

    return ','.join(str(r) for r in result)

def part2(gid):
    """Solution to part 2"""
    biggest, result = None, None
    for x, y, size in itertools.product(range(0, 300), range(0, 300), range(1,301)):
        pass
    return ','.join(str(r) for r in result)

Record = collections.namedtuple('Record', '')

def parse(line):
    m = re.match(r"", line)
    return Record(*[int(g) for g in m.groups()])

if __name__ == '__main__':
    LINES = 5034
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
