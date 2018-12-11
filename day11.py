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

def part1(gid, size=None):
    """Solution to part 1"""
    part2 = True
    if size is None:
        size = 3
        part2 = False
    biggest = None
    for x, y in itertools.product(range(0, 302-size), range(0, 302-size)):
        total = 0
        for i, j in itertools.product(range(x,x+size), range(y,y+size)):
            assert not (i == 301 and j == 301)
            total += get_power(i, j, gid)

        if biggest is None or biggest < total:
            result = (x, y)
            biggest = total
    if part2:
        return result, biggest 

    return ','.join(str(r) for r in result)

def part2(gid):
    """Solution to part 2"""
    biggest, result = None, None
    for size in range(1, 30):
        coords, score = part1(gid, size)
        if biggest is None or score > biggest:
            biggest = score
            result = (*coords, size)
    return ','.join(str(r) for r in result)

Record = collections.namedtuple('Record', '')

def parse(line):
    m = re.match(r"", line)
    return Record(*[int(g) for g in m.groups()])

if __name__ == '__main__':
    LINES = 5034
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
