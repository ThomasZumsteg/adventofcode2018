#!/usr/bin/env pipenv run python
"""Solutions to day 6 of Advent of Code"""

import re
from collections import namedtuple

from get_input import get_input, line_parser

Record = namedtuple('Record', 'x y')

class State:
    def __init__(self):
        self.count = 0
        self.area = set()

def distance(r, p):
    return abs(r.x - p.x) + abs(r.y - p.y)

def part1(records):
    """Solution to part 1"""
    seen = set()
    areas = {r: State() for r in records}
    max_d = max([distance(r, p) for r in records for p in records])

    new_queue = records[:]
    for count in range(1, (max_d + 1) // 2):
        queue = new_queue
        new_queue = []
        for p in queue:
            if p in seen:
                continue
            seen.add(p)

            min_dist = None
            closest = []
            for r in records:
                dist = distance(r, p)
                if min_dist is None or dist < min_dist:
                    min_dist = dist
                    closest = [r]
                elif dist == min_dist:
                    closest.append(r)
            if len(closest) != 1:
                continue
            areas[closest[0]].area.add(p)
            areas[closest[0]].count = count
            for q in (Record(p.x+1,p.y),Record(p.x-1,p.y),Record(p.x,p.y+1),Record(p.x,p.y-1)):
                if q not in seen:

                    new_queue.append(q)
    return max(len(s.area) for s in areas.values() if s.count < count)


def part2(records, limit=10000):
    """Solution to part 2"""
    queue = records[:]
    seen = set()
    area = set()
    while queue:
        p = queue.pop(0)
        if p in seen:
            continue
        seen.add(p)

        dist = sum(distance(p, r) for r in records)
        if dist >= limit:
            continue
        area.add(p)
        queue.extend([
            Record(p.x+1, p.y),
            Record(p.x-1, p.y),
            Record(p.x, p.y+1),
            Record(p.x, p.y-1)])
    return len(area)


def parse(line):
    m = re.match(r"(\d+), (\d+)", line)
    return Record(*[int(g) for g in m.groups()])


if __name__ == '__main__':
    LINES = line_parser(get_input(day=6, year=2018), parse=parse)
    print("Part 1: {}".format(part1(LINES)))
    print("Part 2: {}".format(part2(LINES)))
