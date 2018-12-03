#!/usr/bin/env pipenv run python
"""Solutions to day 3 of Advent of Code"""

import itertools
import re

from collections import namedtuple

from get_input import get_input, line_parser


def make_sheet(claims):
    """Make a sheet 1000x1000 wide and identify which claims"""
    sheet = [[[] for _ in range(1000)] for _ in range(1000)]
    for claim in claims:
        for i, j in itertools.product(
                range(claim.from_left, claim.from_left + claim.width),
                range(claim.from_top, claim.from_top + claim.height)):
            sheet[i][j].append(claim.id)
    return tuple(tuple(tuple(ids) for ids in row) for row in sheet)


def part1(claims):
    """Solution to part 1, how many square inches overlap"""
    sheet = make_sheet(claims)
    return sum(1 for square in itertools.chain.from_iterable(
        zip(*sheet)) if len(square) > 1)


def part2(claims):
    """Solution to part 2, which claim does not overlap"""
    sheet = make_sheet(claims)
    solutions = []
    for claim in claims:
        for i, j in itertools.product(
                range(claim.from_left, claim.from_left + claim.width),
                range(claim.from_top, claim.from_top + claim.height)):
            if len(sheet[i][j]) > 1:
                break
        else:
            solutions.append(claim.id)
    if len(solutions) == 1:
        return solutions[0]
    raise Exception("To many solutions")


Claim = namedtuple('Claim', 'id from_left from_top width height')

def parse(line):
    """Parse a single line of text into claims
    claims have the form:
    '#<id> @ <from_left>,<from_top>: <width>x<height>
    example:
    #6 @ 796,785: 17x18
    """
    match = re.match(r"#(?P<id>\d+) @ (?P<from_left>\d+),(?P<from_top>\d+): "\
                     r"(?P<width>\d+)x(?P<height>\d+)", line)
    return Claim(*[int(g) for g in match.groups()])

if __name__ == '__main__':
    LINES = line_parser(get_input(day=3, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
