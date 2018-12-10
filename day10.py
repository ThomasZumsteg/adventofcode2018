#!/usr/bin/env pipenv run python
"""Solutions to day 10 of Advent of Code"""

import re
import itertools
import collections

from get_input import get_input, line_parser

def part1(points):
    """Solution to part 1"""
    new = points
    size = None
    for s in itertools.count():
        grid, new = new, []
        max_x = max(grid, key=lambda p: p.px).px
        max_y = max(grid, key=lambda p: p.py).py
        min_x = min(grid, key=lambda p: p.px).px
        min_y = min(grid, key=lambda p: p.py).py
        
        last, size = size, (max_x - min_x) * (max_y - min_y)
        if last is not None and last < size:
            return output
        if 100 > (max_x - min_x) and 50 > (max_y - min_y):
            lines = [[' '] * (max_x - min_x + 1) for _ in range(max_y - min_y + 1)]
            for p in grid:
                lines[p.py - min_y][p.px - min_x] = '#'
            for l, line in enumerate(lines):
                lines[l] = ''.join(line)
            output = ''.join('\n' + line for line in lines)
        for p in grid:
            new.append(Record(p.px + p.vx, p.py + p.vy, p.vx, p.vy))
        s += 1

def part2(points):
    """Solution to part 2"""
    new = points
    size = None
    for s in itertools.count():
        grid, new = new, []
        max_x = max(grid, key=lambda p: p.px).px
        max_y = max(grid, key=lambda p: p.py).py
        min_x = min(grid, key=lambda p: p.px).px
        min_y = min(grid, key=lambda p: p.py).py
        
        last, size = size, (max_x - min_x) * (max_y - min_y)
        if last is not None and last < size:
            return s - 1
        for p in grid:
            new.append(Record(p.px + p.vx, p.py + p.vy, p.vx, p.vy))
        s += 1

Record = collections.namedtuple('Record', 'px py vx vy')

def parse(line):
    m = re.match(r"position=<([ -]\d+), ([ -]\d+)> velocity=<([ -]\d+), ([ -]\d+)>", line)
    return Record(*[int(g) for g in m.groups()])

if __name__ == '__main__':
    LINES = line_parser(get_input(day=10, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
