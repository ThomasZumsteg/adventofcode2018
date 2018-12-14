#!/usr/bin/env pipenv run python
"""Solutions to day 13 of Advent of Code"""

import re
import itertools
import collections
from dataclasses import dataclass

from get_input import get_input, line_parser


def part1(digits):
    """Solution to part 1"""
    a, b = 0, 1
    recipies = "37"
    while True:
        int_a, int_b = int(recipies[a]), int(recipies[b])
        new = str(int_a + int_b)
        for n in new:
            recipies += n
            if 10 + int(digits) <= len(recipies):
                return recipies[-10:]
        a = (a + int_a + 1) % len(recipies)
        b = (b + int_b + 1) % len(recipies)


def part2(digits):
    """Solution to part 2"""
    a, b = 0, 1
    recipies = "37"
    while True:
        int_a, int_b = int(recipies[a]), int(recipies[b])
        new = str(int_a + int_b)
        for n in new:
            recipies += n
            if recipies.endswith(digits):
                return len(recipies) - len(digits)
        a = (a + int_a + 1) % len(recipies)
        b = (b + int_b + 1) % len(recipies)


Record = collections.namedtuple('Record', '')

if __name__ == '__main__':
    LINES = get_input(day=14, year=2018).strip()
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
