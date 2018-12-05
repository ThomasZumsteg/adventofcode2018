#!/usr/bin/env pipenv run python
"""Solutions to day 5 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

def part1(polymer):
    """Solution to part 1"""
    old_polymer = None
    while not old_polymer == polymer:
        old_polymer = tuple(polymer)


def part2(polymer):
    """Solution to part 2"""
    ...


if __name__ == '__main__':
    LINE = get_input(day=5, year=2018)
    print("Part 1: {}".format(part1(LINE)))
    print("Part 2: {}".format(part2(LINE)))
