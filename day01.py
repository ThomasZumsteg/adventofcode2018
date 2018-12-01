#!/usr/bin/env pipenv run python
"""Solutions to day 1 of Advent of Code"""

from itertools import cycle

from get_input import get_input, line_parser

def part1(lines):
    """Sum of the list"""
    return sum(lines)


def part2(lines):
    """First time a total occures twice"""
    total = 0
    seen = set()
    for line in cycle(lines):
        if total in seen:
            break
        seen.add(total)
        total += line
    return total

if __name__ == '__main__':
    LINES = line_parser(get_input(day=1, year=2018))
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
